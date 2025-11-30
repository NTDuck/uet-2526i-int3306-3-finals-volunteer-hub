use ::axiom::prelude::*;
use ::futures::prelude::*;

use crate::boundaries::*;
use crate::gateways::*;

#[derive(::bon::Builder)]
pub struct ViewEventHistoryInteractor {
    event_repository: ::std::sync::Arc<dyn EventRepository + ::core::marker::Send + ::core::marker::Sync>,
    event_registration_repository: ::std::sync::Arc<dyn EventRegistrationRepository + ::core::marker::Send + ::core::marker::Sync>,
    user_repository: ::std::sync::Arc<dyn UserRepository + ::core::marker::Send + ::core::marker::Sync>,

    uuid_codec: ::std::sync::Arc<dyn UuidCodec + ::core::marker::Send + ::core::marker::Sync>,
    auth_token_generator:
        ::std::sync::Arc<dyn AuthenticationTokenGenerator + ::core::marker::Send + ::core::marker::Sync>,
}

#[async_trait]
impl ViewEventHistoryBoundary for ViewEventHistoryInteractor {
    async fn apply(
        self: ::std::sync::Arc<Self>, request: ViewEventHistoryRequest,
    ) -> ::axiom::result::Fallible<ViewEventHistoryResponse> {
        let actor_id = match ::std::sync::Arc::clone(&self.auth_token_generator).get_payload(request.token).await? {
            ::core::option::Option::None =>
                return ::axiom::err!(ViewEventHistory @ AuthenticationTokenInvalid),
            ::core::option::Option::Some
            (AuthenticationTokenPayload { user_id, user_role: ::domain::UserRole::Volunteer, expiry_timestamp }) => {
                if expiry_timestamp < ::axiom::time::Timestamp::now() {
                    return ::axiom::err!(ViewEventHistory @ AuthenticationTokenExpired);
                }

                if !::std::sync::Arc::clone(&self.user_repository).contains_id(user_id).await? {
                    return ::axiom::err!(ViewEventHistory @ UserNotFound);
                }

                user_id
            },
            ::core::option::Option::Some(AuthenticationTokenPayload { user_role, .. }) =>
                return ::axiom::err!(ViewEventHistory @ UserUnauthorized { user_role: user_role.into() }),
        };

        let event_registrations = ::std::sync::Arc::clone(&self.event_registration_repository).view_by_user_id(actor_id).await?;

        let events = ::futures::stream::iter(event_registrations)
            .map(|event_registration| (event_registration.event_id, *event_registration.statuses.last()))
            .filter_map(|(event_id, event_registration_status)| {
                let event_repository = ::std::sync::Arc::clone(&self.event_repository);

                async move {
                    event_repository.get_by_id(event_id).await.ok()?
                        .map(|event| (event, event_registration_status))
                }
            })
            .filter_map(|(event, event_registration_status)| {
                let uuid_codec = ::std::sync::Arc::clone(&self.uuid_codec);

                async move {
                    ViewEventHistoryEvent::build_from(event, event_registration_status)
                        .with_uuid_codec(uuid_codec)
                        .try_build().await
                        .ok()
                }
            })
            .collect::<::std::vec::Vec<_>>().await;

        let response = ViewEventHistoryOkResponse::builder().events(events).build();
        ::axiom::ok!(ViewEventHistory @ response)
    }
}
