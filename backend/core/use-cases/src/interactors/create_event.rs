use ::axiom::prelude::*;

use crate::boundaries::*;
use crate::gateways::*;

#[derive(::bon::Builder)]
pub struct CreateEventInteractor {
    event_repository: ::std::sync::Arc<dyn EventRepository + ::core::marker::Send + ::core::marker::Sync>,
    user_repository: ::std::sync::Arc<dyn UserRepository + ::core::marker::Send + ::core::marker::Sync>,
    media_repository: ::std::sync::Arc<dyn MediaRepository + ::core::marker::Send + ::core::marker::Sync>,

    uuid_generator: ::std::sync::Arc<dyn UuidGenerator + ::core::marker::Send + ::core::marker::Sync>,
    auth_token_generator: ::std::sync::Arc<dyn AuthTokenGenerator + ::core::marker::Send + ::core::marker::Sync>,
}

#[async_trait]
impl CreateEventBoundary for CreateEventInteractor {
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
                    allowed_user_roles: ::std::vec![UserRole::EventManager,],
                }),
        };

        let mut errors = ::std::vec::Vec::new();

        let event_name = ::domain::EventName::try_from(request.event_name)
            .map_err(|error| errors.push(ErrResponse::EventNameInvalid { event_name: error.into() }));

        let event_description = ::domain::EventDescription::try_from(request.event_description)
            .map_err(|error| errors.push(ErrResponse::EventDescriptionInvalid { event_description: error.into() }));

        let event_categories = request
            .event_categories
            .into_iter()
            .try_collect_all::<::std::vec::Vec<_>, ::std::vec::Vec<_>, _, _, _>(::domain::EventCategory::try_from)
            .map_err(|errors_| {
                errors.push(ErrResponse::EventCategoriesInvalid {
                    event_categories: errors_.into_iter().map(::core::convert::Into::into).collect(),
                })
            });

        let event_location = ::domain::EventLocation::try_from(request.event_location)
            .map_err(|error| errors.push(ErrResponse::EventLocationInvalid { event_location: error.into() }));

        // let event_image = ::axiom::bytes::Bytes::from(request.event_image);
        let event_image = ::axiom::bytes::Bytes::from("Hello world");

        if !::std::sync::Arc::clone(&self.media_repository)
            .verify(event_image.clone())
            .await?
        {
            errors.push(ErrResponse::EventImageInvalid);
        }

        let event_image_url = ::std::sync::Arc::clone(&self.media_repository).save(event_image).await?;

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

        let event_id = loop {
            let uuid = ::std::sync::Arc::clone(&self.uuid_generator).generate().await?;
            if !::std::sync::Arc::clone(&self.event_repository).contains_id(uuid).await? {
                break uuid;
            }
        };

        let event = ::domain::Event::builder()
            .id(event_id)
            .statuses(::vec1::vec1!(::domain::EventStatus::Created {
                created_by_manager_id: actor_id,
                created_at: ::axiom::time::Timestamp::now()
            }))
            .name(event_name)
            .description(event_description)
            .categories(event_categories)
            .location(event_location)
            .image_url(event_image_url)
            .build();

        ::std::sync::Arc::clone(&self.event_repository).save(event).await?;

        super::ok!(())
    }
}

type Request = CreateEventRequest;
type Response = CreateEventResponse;
type ErrResponse = CreateEventErrResponse;
type UserRole = CreateEventUserRole;
