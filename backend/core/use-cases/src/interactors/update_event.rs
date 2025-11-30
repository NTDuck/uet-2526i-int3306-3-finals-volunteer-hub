use ::axiom::prelude::*;

use crate::boundaries::*;
use crate::gateways::*;

#[derive(::bon::Builder)]
pub struct UpdateEventInteractor {
    event_repository: ::std::sync::Arc<dyn EventRepository + ::core::marker::Send + ::core::marker::Sync>,
    user_repository: ::std::sync::Arc<dyn UserRepository + ::core::marker::Send + ::core::marker::Sync>,

    uuid_codec: ::std::sync::Arc<dyn UuidCodec + ::core::marker::Send + ::core::marker::Sync>,
    auth_token_generator:
        ::std::sync::Arc<dyn AuthenticationTokenGenerator + ::core::marker::Send + ::core::marker::Sync>,
}

#[async_trait]
impl UpdateEventBoundary for UpdateEventInteractor {
    async fn apply(
        self: ::std::sync::Arc<Self>, request: UpdateEventRequest,
    ) -> ::axiom::result::Fallible<UpdateEventResponse> {
        let user_id = match ::std::sync::Arc::clone(&self.auth_token_generator).get_payload(request.token).await? {
            ::core::option::Option::None =>
                return ::axiom::err!(UpdateEvent @ AuthenticationTokenInvalid),
            ::core::option::Option::Some
            (AuthenticationTokenPayload { user_id, user_role: ::domain::UserRole::EventManager, expiry_timestamp }) => {
                if expiry_timestamp < ::axiom::time::Timestamp::now() {
                    return ::axiom::err!(UpdateEvent @ AuthenticationTokenExpired);
                }

                match ::std::sync::Arc::clone(&self.user_repository).get_by_id(user_id).await? {
                    ::core::option::Option::Some(user) => {
                        if ::core::matches!(user.statuses.last(), ::domain::UserStatus::Suspended { .. }) {
                            return ::axiom::err!(UpdateEvent @ UserSuspended);
                        }
                    },
                    ::core::option::Option::None =>
                        return ::axiom::err!(UpdateEvent @ UserNotFound),
                }

                user_id
            },
            ::core::option::Option::Some(AuthenticationTokenPayload { user_role, .. }) =>
                return ::axiom::err!(UpdateEvent @ UserUnauthorized { user_role: user_role.into() }),
        };

        let mut errors = ::std::vec::Vec::new();

        let event_id = ::std::sync::Arc::clone(&self.uuid_codec).parse(request.event_id).await?;

        let event = ::std::sync::Arc::clone(&self.event_repository).get_by_id(event_id).await?;

        match event {
            ::core::option::Option::Some(::domain::Event { ref statuses, .. }) => {
                let event_status = statuses.last();

                if !::core::matches!(event_status, ::domain::EventStatus::Created { .. } | ::domain::EventStatus::Updated { .. }) {
                    errors.push(UpdateEventErrResponse::EventStatusNotEligible { event_status: (*event_status).into() });
                }
            },

            ::core::option::Option::None => {
                errors.push(UpdateEventErrResponse::EventNotFound);
            },
        }

        let event_name = request.event_name
            .map(|event_name| ::domain::EventName::try_from(event_name)
                .map_err(|error| errors.push(UpdateEventErrResponse::EventNameInvalid { event_name: error.into() })))
                .transpose();

        let event_description = request.event_description
            .map(|event_description| ::domain::EventDescription::try_from(event_description)
                .map_err(|error| errors.push(UpdateEventErrResponse::EventDescriptionInvalid { event_description: error.into() })))
                .transpose();

        let event_categories = request.event_categories
            .map(|event_categories| event_categories
                .into_iter()
                .try_collect_all::<::std::vec::Vec<_>, ::std::vec::Vec<_>, _, _, _>(::domain::EventCategory::try_from)
                .map_err(|errors_| errors.push(UpdateEventErrResponse::EventCategoriesInvalid { event_categories: errors_.into_iter().map(::core::convert::Into::into).collect() })))
                .transpose();

        let event_location = request.event_location
            .map(|event_location| ::domain::EventLocation::try_from(event_location)
                .map_err(|error| errors.push(UpdateEventErrResponse::EventLocationInvalid { event_location: error.into() })))
                .transpose();

        let (
            ::core::result::Result::Ok(event_name),
            ::core::result::Result::Ok(event_description),
            ::core::result::Result::Ok(event_categories),
            ::core::result::Result::Ok(event_location),
        ) = (event_name, event_description, event_categories, event_location) else { return ::axiom::errs!(UpdateEvent @ errors) };

        if !errors.is_empty() {
            return ::axiom::errs!(UpdateEvent @ errors);
        }

        let mut event = unsafe { event.unwrap_unchecked() };

        event.statuses.push(::domain::EventStatus::Updated { updated_by_manager_id: user_id, updated_at: ::axiom::time::Timestamp::now() });
        event_name.map(|event_name| event.name = event_name);
        event_description.map(|event_description| event.description = event_description);
        event_categories.map(|event_categories| event.categories = event_categories);
        event_location.map(|event_location| event.location = event_location);

        ::std::sync::Arc::clone(&self.event_repository).save(event).await?;

        ::axiom::ok!(UpdateEvent)
    }
}
