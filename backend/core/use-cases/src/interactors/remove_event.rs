use ::axiom::prelude::*;

use crate::boundaries::*;
use crate::gateways::*;

#[derive(::bon::Builder)]
pub struct RemoveEventInteractor {
    event_repository: ::std::sync::Arc<dyn EventRepository + ::core::marker::Send + ::core::marker::Sync>,
    user_repository: ::std::sync::Arc<dyn UserRepository + ::core::marker::Send + ::core::marker::Sync>,

    uuid_codec: ::std::sync::Arc<dyn UuidCodec + ::core::marker::Send + ::core::marker::Sync>,
    auth_token_generator: ::std::sync::Arc<dyn AuthTokenGenerator + ::core::marker::Send + ::core::marker::Sync>,
}

#[async_trait]
impl RemoveEventBoundary for RemoveEventInteractor {
    async fn apply(self: ::std::sync::Arc<Self>, request: Request) -> ::axiom::result::Fallible<Response> {
        match ::std::sync::Arc::clone(&self.auth_token_generator)
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
                    errors.push(RemoveEventErrResponse::EventStatusNotEligible {
                        event_status: event_status.into(),
                        allowed_event_statuses: ::std::vec![EventStatus::Created, EventStatus::Updated,],
                    });
                }
            },

            ::core::option::Option::None => {
                errors.push(ErrResponse::EventNotFound);
            },
        };

        if !errors.is_empty() {
            return super::errs!(errors);
        }

        let event = unsafe { event.unwrap_unchecked() };

        ::std::sync::Arc::clone(&self.event_repository).remove(event.id).await?;

        super::ok!(())
    }
}

type Request = RemoveEventRequest;
type Response = RemoveEventResponse;
type ErrResponse = RemoveEventErrResponse;
type UserRole = RemoveEventUserRole;
type EventStatus = RemoveEventEventStatus;
