use ::axiom::prelude::*;

use crate::boundaries::*;
use crate::gateways::*;

#[derive(::bon::Builder)]
pub struct UpdateEventInteractor {
    event_repository: ::std::sync::Arc<dyn EventRepository + ::core::marker::Send + ::core::marker::Sync>,
    user_repository: ::std::sync::Arc<dyn UserRepository + ::core::marker::Send + ::core::marker::Sync>,
    media_repository: ::std::sync::Arc<dyn MediaRepository + ::core::marker::Send + ::core::marker::Sync>,

    uuid_codec: ::std::sync::Arc<dyn UuidCodec + ::core::marker::Send + ::core::marker::Sync>,
    auth_token_generator: ::std::sync::Arc<dyn AuthTokenGenerator + ::core::marker::Send + ::core::marker::Sync>,
}

#[async_trait]
impl UpdateEventBoundary for UpdateEventInteractor {
    async fn apply(self: ::std::sync::Arc<Self>, request: Request) -> ::axiom::result::Fallible<Response> {
        let actor_id = match ::std::sync::Arc::clone(&self.auth_token_generator)
            .get_payload(request.token)
            .await?
        {
            ::core::option::Option::None => return super::err!(AuthenticationTokenInvalid),
            ::core::option::Option::Some(AuthTokenPayload {
                user_id,
                user_role: ::domain::UserRole::EventManager,
                expiry_timestamp,
            }) => {
                if expiry_timestamp < ::axiom::time::Timestamp::now() {
                    return super::err!(AuthenticationTokenExpired);
                }

                match ::std::sync::Arc::clone(&self.user_repository).get_by_id(user_id).await? {
                    ::core::option::Option::Some(::domain::User { statuses, .. }) => {
                        if ::core::matches!(statuses[..], [.., ::domain::UserStatus::Suspended { .. }]) {
                            return super::err!(UserSuspended);
                        }
                    },
                    ::core::option::Option::None => return super::err!(UserNotFound),
                }

                user_id
            },
            ::core::option::Option::Some(AuthTokenPayload { user_role, .. }) =>
                return super::err!(UserUnauthorized {
                    user_role: user_role.into(),
                    allowed_user_roles: ::std::vec![UserRole::EventManager]
                }),
        };

        let mut errors = ::std::vec::Vec::new();

        let event_id = ::std::sync::Arc::clone(&self.uuid_codec).parse(request.event_id).await?;

        let event = ::std::sync::Arc::clone(&self.event_repository).get_by_id(event_id).await?;

        match event {
            ::core::option::Option::Some(::domain::Event { ref statuses, .. }) => {
                let event_status = *statuses.last();

                if !::core::matches!(
                    event_status,
                    ::domain::EventStatus::Created { .. } | ::domain::EventStatus::Updated { .. }
                ) {
                    errors.push(ErrResponse::EventStatusNotEligible {
                        event_status: event_status.into(),
                        allowed_event_statuses: ::std::vec![
                            UpdateEventEventStatus::Created,
                            UpdateEventEventStatus::Updated
                        ],
                    });
                }
            },

            ::core::option::Option::None => {
                errors.push(ErrResponse::EventNotFound);
            },
        }

        let event_name = request
            .event_name
            .map(|event_name| {
                ::domain::EventName::try_from(event_name)
                    .map_err(|error| errors.push(ErrResponse::EventNameInvalid { event_name: error.into() }))
            })
            .transpose();

        let event_description = request
            .event_description
            .map(|event_description| {
                ::domain::EventDescription::try_from(event_description).map_err(|error| {
                    errors.push(ErrResponse::EventDescriptionInvalid { event_description: error.into() })
                })
            })
            .transpose();

        let event_categories = request
            .event_categories
            .map(|event_categories| {
                event_categories
                    .into_iter()
                    .try_collect_all::<::std::vec::Vec<_>, ::std::vec::Vec<_>, _, _, _>(
                        ::domain::EventCategory::try_from,
                    )
                    .map_err(|errors_| {
                        errors.push(ErrResponse::EventCategoriesInvalid {
                            event_categories: errors_.into_iter().map(::core::convert::Into::into).collect(),
                        })
                    })
            })
            .transpose();

        let event_location = request
            .event_location
            .map(|event_location| {
                ::domain::EventLocation::try_from(event_location)
                    .map_err(|error| errors.push(ErrResponse::EventLocationInvalid { event_location: error.into() }))
            })
            .transpose();

        let event_image_url = request
            .event_image
            .map(::core::convert::Into::<::axiom::bytes::Bytes>::into)
            .map_async(|image| {
                let media_repository = ::std::sync::Arc::clone(&self.media_repository);

                async move {
                    let verified = ::std::sync::Arc::clone(&media_repository).verify(image.clone()).await?;

                    (image, verified).into_ok()
                }
            })
            .await
            .transpose()?
            .map(|(image, verified)| {
                if !verified {
                    errors.push(ErrResponse::EventImageInvalid);
                }

                image
            })
            .map_async(|image| {
                let media_repository = ::std::sync::Arc::clone(&self.media_repository);

                async move { ::std::sync::Arc::clone(&media_repository).save(image).await?.into_ok() }
            })
            .await
            .transpose()?;

        let (
            ::core::result::Result::Ok(event_name),
            ::core::result::Result::Ok(event_description),
            ::core::result::Result::Ok(event_categories),
            ::core::result::Result::Ok(event_location),
        ) = (event_name, event_description, event_categories, event_location)
        else {
            return super::errs!(errors);
        };

        if !errors.is_empty() {
            return super::errs!(errors);
        }

        let mut event = unsafe { event.unwrap_unchecked() };

        event.statuses.push(::domain::EventStatus::Updated {
            updated_by_manager_id: actor_id,
            updated_at: ::axiom::time::Timestamp::now(),
        });
        event_name.map(|event_name| event.name = event_name);
        event_description.map(|event_description| event.description = event_description);
        event_categories.map(|event_categories| event.categories = event_categories);
        event_location.map(|event_location| event.location = event_location);
        event_image_url
            .map(::core::convert::Into::into)
            .map(|event_image_url| event.image_url = event_image_url);

        ::std::sync::Arc::clone(&self.event_repository).save(event).await?;

        super::ok!(())
    }
}

type Request = UpdateEventRequest;
type Response = UpdateEventResponse;
type ErrResponse = UpdateEventErrResponse;
type UserRole = UpdateEventUserRole;
