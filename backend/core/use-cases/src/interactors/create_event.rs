use ::async_trait::async_trait;

use crate::boundaries::*;
use crate::gateways::*;

#[derive(::bon::Builder)]
pub struct CreateEventInteractor {
    event_repository: ::std::sync::Arc<dyn EventRepository + ::core::marker::Send + ::core::marker::Sync>,
    user_repository: ::std::sync::Arc<dyn UserRepository + ::core::marker::Send + ::core::marker::Sync>,

    uuid_generator: ::std::sync::Arc<dyn UuidGenerator + ::core::marker::Send + ::core::marker::Sync>,
    auth_token_generator:
        ::std::sync::Arc<dyn AuthenticationTokenGenerator + ::core::marker::Send + ::core::marker::Sync>,
}

#[async_trait]
impl CreateEventBoundary for CreateEventInteractor {
    async fn apply(
        self: ::std::sync::Arc<Self>, request: CreateEventRequest,
    ) -> ::axiom::result::Fallible<CreateEventResponse> {
        use ::axiom::time::TimestampExt as _;
        use ::axiom::iter::IteratorTryCollectAllExt as _;

        let user_id = match ::std::sync::Arc::clone(&self.auth_token_generator).get_payload(request.token).await? {
            ::core::option::Option::None =>
                return ::axiom::err!(CreateEvent @ AuthenticationTokenInvalid),
            ::core::option::Option::Some
            (AuthenticationTokenPayload { user_id, user_role: ::domain::UserRole::EventManager, expiry_timestamp }) => {
                if expiry_timestamp < ::axiom::time::Timestamp::now() {
                    return ::axiom::err!(CreateEvent @ AuthenticationTokenExpired);
                }

                match ::std::sync::Arc::clone(&self.user_repository).get_by_id(user_id).await? {
                    ::core::option::Option::Some(user) => {
                        if ::core::matches!(user.statuses.last(), ::domain::UserStatus::Suspended { .. }) {
                            return ::axiom::err!(CreateEvent @ UserSuspended);
                        }
                    },
                    ::core::option::Option::None =>
                        return ::axiom::err!(CreateEvent @ UserNotFound),
                }

                user_id
            },
            ::core::option::Option::Some(AuthenticationTokenPayload { user_role, .. }) =>
                return ::axiom::err!(CreateEvent @ UserUnauthorized { user_role: user_role.into() }),
        };

        let mut errors = ::std::vec::Vec::new();

        let event_name = ::domain::EventName::try_from(request.event_name)
            .map_err(|error| errors.push(CreateEventErrResponse::EventNameInvalid { event_name: error.into() }));

        let event_description = ::domain::EventDescription::try_from(request.event_description)
            .map_err(|error| errors.push(CreateEventErrResponse::EventDescriptionInvalid { event_description: error.into() }));

        let event_categories = request.event_categories
            .into_iter()
            .try_collect_all::<::std::vec::Vec<_>, ::std::vec::Vec<_>, _, _, _>(::domain::EventCategory::try_from)
            .map_err(|errors_| errors.push(CreateEventErrResponse::EventCategoriesInvalid { event_categories: errors_.into_iter().map(::core::convert::Into::into).collect() }));

        let event_location = ::domain::EventLocation::try_from(request.event_location)
            .map_err(|error| errors.push(CreateEventErrResponse::EventLocationInvalid { event_location: error.into() }));

        let (::core::result::Result::Ok(event_name), ::core::result::Result::Ok(event_description), ::core::result::Result::Ok(event_categories), ::core::result::Result::Ok(event_location)) = (event_name, event_description, event_categories, event_location) else {
            return ::axiom::errs!(CreateEvent @ errors);
        };

        if !errors.is_empty() {
            return ::axiom::errs!(CreateEvent @ errors);
        }

        let event_id = loop {
            let uuid = ::std::sync::Arc::clone(&self.uuid_generator).generate().await?;

            if !::std::sync::Arc::clone(&self.user_repository).contains_id(uuid).await? {
                break uuid;
            }
        };

        let event = ::domain::Event::builder()
            .id(event_id)
            .statuses(::vec1::vec1!(::domain::EventStatus::Created { created_by_manager_id: user_id }))
            .name(event_name)
            .description(event_description)
            .categories(event_categories)
            .location(event_location)
            .build();

        ::std::sync::Arc::clone(&self.event_repository).save(event).await?;

        ::axiom::ok!(CreateEvent)
    }
}
