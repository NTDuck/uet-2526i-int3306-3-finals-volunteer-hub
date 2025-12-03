use ::axiom::prelude::*;

use crate::boundaries::*;
use crate::gateways::*;

#[derive(::bon::Builder)]
pub struct RemoveEventInteractor {
    event_repository: ::std::sync::Arc<dyn EventRepository + ::core::marker::Send + ::core::marker::Sync>,
    user_repository: ::std::sync::Arc<dyn UserRepository + ::core::marker::Send + ::core::marker::Sync>,

    uuid_codec: ::std::sync::Arc<dyn UuidCodec + ::core::marker::Send + ::core::marker::Sync>,
    auth_token_generator:
        ::std::sync::Arc<dyn AuthenticationTokenGenerator + ::core::marker::Send + ::core::marker::Sync>,
}

#[async_trait]
impl RemoveEventBoundary for RemoveEventInteractor {
    async fn apply(
        self: ::std::sync::Arc<Self>, request: RemoveEventRequest,
    ) -> ::axiom::result::Fallible<RemoveEventResponse> {
        match ::std::sync::Arc::clone(&self.auth_token_generator)
            .get_payload(request.token)
            .await?
        {
            ::core::option::Option::None => return ::axiom::err!(RemoveEvent @ AuthenticationTokenInvalid),
            ::core::option::Option::Some(AuthenticationTokenPayload {
                user_id,
                user_role: ::domain::UserRole::EventManager,
                expiry_timestamp,
            }) => {
                if expiry_timestamp < ::axiom::time::Timestamp::now() {
                    return ::axiom::err!(RemoveEvent @ AuthenticationTokenExpired);
                }

                match ::std::sync::Arc::clone(&self.user_repository).get_by_id(user_id).await? {
                    ::core::option::Option::Some(user) => {
                        if ::core::matches!(user.statuses.last(), ::domain::UserStatus::Suspended { .. }) {
                            return ::axiom::err!(RemoveEvent @ UserSuspended);
                        }
                    },
                    ::core::option::Option::None => return ::axiom::err!(RemoveEvent @ UserNotFound),
                }
            },
            ::core::option::Option::Some(AuthenticationTokenPayload { user_role, .. }) =>
                return ::axiom::err!(RemoveEvent @ UserUnauthorized { user_role: user_role.into(), allowed_user_roles: ::std::vec![RemoveEventUserRole::EventManager] }),
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
                    errors.push(RemoveEventErrResponse::EventStatusNotEligible {
                        event_status: event_status.into(),
                        allowed_event_statuses: ::std::vec![
                            RemoveEventEventStatus::Created,
                            RemoveEventEventStatus::Updated
                        ],
                    });
                }
            },

            ::core::option::Option::None => {
                errors.push(RemoveEventErrResponse::EventNotFound);
            },
        };

        if !errors.is_empty() {
            return ::axiom::errs!(RemoveEvent @ errors);
        }

        let event = unsafe { event.unwrap_unchecked() };

        ::std::sync::Arc::clone(&self.event_repository).remove(event.id).await?;

        ::axiom::ok!(RemoveEvent)
    }
}
