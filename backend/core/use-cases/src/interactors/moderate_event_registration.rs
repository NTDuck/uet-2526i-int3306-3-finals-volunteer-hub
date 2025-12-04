use ::axiom::prelude::*;

use crate::boundaries::*;
use crate::gateways::*;

#[derive(::bon::Builder)]
pub struct ModerateEventRegistrationInteractor {
    event_registration_repository:
        ::std::sync::Arc<dyn EventRegistrationRepository + ::core::marker::Send + ::core::marker::Sync>,
    user_repository: ::std::sync::Arc<dyn UserRepository + ::core::marker::Send + ::core::marker::Sync>,

    uuid_codec: ::std::sync::Arc<dyn UuidCodec + ::core::marker::Send + ::core::marker::Sync>,
    auth_token_generator:
        ::std::sync::Arc<dyn AuthTokenGenerator + ::core::marker::Send + ::core::marker::Sync>,
}

#[async_trait]
impl ModerateEventRegistrationBoundary for ModerateEventRegistrationInteractor {
    async fn apply(
        self: ::std::sync::Arc<Self>, request: Request,
    ) -> ::axiom::result::Fallible<Response> {
        let actor_id = match ::std::sync::Arc::clone(&self.auth_token_generator)
            .get_payload(request.token)
            .await?
        {
            ::core::option::Option::None =>
                return super::err!(AuthenticationTokenInvalid),
            ::core::option::Option::Some(AuthenticationTokenPayload {
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
            ::core::option::Option::Some(AuthenticationTokenPayload { user_role, .. }) =>
                return super::err!(UserUnauthorized { user_role: user_role.into(), allowed_user_roles: ::std::vec![UserRole::EventManager] }),
        };

        let event_registration_id = ::std::sync::Arc::clone(&self.uuid_codec)
            .parse(request.event_registration_id)
            .await?;

        let ::core::option::Option::Some(mut event_registration) =
            ::std::sync::Arc::clone(&self.event_registration_repository)
                .get_by_id(event_registration_id)
                .await?
        else {
            return super::err!(EventRegistrationNotFound);
        };

        let event_registration_status = *event_registration.statuses.last();

        match request.event_registration_status {
            NewEventRegistrationStatus::Accepted => {
                if !::core::matches!(
                    event_registration_status,
                    ::domain::EventRegistrationStatus::Pending { .. }
                        | ::domain::EventRegistrationStatus::Declined { .. }
                ) {
                    return super::err!(EventRegistrationStatusNotEligible {
                        event_registration_status: event_registration_status.into(),
                        allowed_event_registration_statuses: ::std::vec![
                            EventRegistrationStatus::Pending,
                            EventRegistrationStatus::Declined,
                        ],
                    });
                }

                event_registration.statuses.push(::domain::EventRegistrationStatus::Accepted {
                    accepted_by_manager_id: actor_id,
                    accepted_at: ::axiom::time::Timestamp::now(),
                });
            },

            NewEventRegistrationStatus::Declined => {
                if !::core::matches!(
                    event_registration_status,
                    ::domain::EventRegistrationStatus::Pending { .. }
                        | ::domain::EventRegistrationStatus::Accepted { .. }
                ) {
                    return super::err!(EventRegistrationStatusNotEligible {
                        event_registration_status: event_registration_status.into(),
                        allowed_event_registration_statuses: ::std::vec![
                            EventRegistrationStatus::Pending,
                            EventRegistrationStatus::Accepted,
                        ],
                    });
                }

                event_registration.statuses.push(::domain::EventRegistrationStatus::Declined {
                    declined_by_manager_id: actor_id,
                    declined_at: ::axiom::time::Timestamp::now(),
                });
            },

            NewEventRegistrationStatus::Completed => {
                if !::core::matches!(
                    event_registration_status,
                    ::domain::EventRegistrationStatus::Accepted { .. }
                        | ::domain::EventRegistrationStatus::Declined { .. }
                ) {
                    return super::err!(EventRegistrationStatusNotEligible {
                        event_registration_status: event_registration_status.into(),
                        allowed_event_registration_statuses: ::std::vec![
                            EventRegistrationStatus::Accepted,
                            EventRegistrationStatus::Declined,
                        ],
                    });
                }

                event_registration.statuses.push(::domain::EventRegistrationStatus::Completed {
                    completed_by_manager_id: actor_id,
                    completed_at: ::axiom::time::Timestamp::now(),
                });
            },
        }

        ::std::sync::Arc::clone(&self.event_registration_repository)
            .save(event_registration)
            .await?;

        super::ok!(())
    }
}

type Request = ModerateEventRegistrationRequest;
type Response = ModerateEventRegistrationResponse;
type ErrResponse = ModerateEventRegistrationErrResponse;
type UserRole = ModerateEventRegistrationUserRole;
type EventRegistrationStatus = ModerateEventRegistrationEventRegistrationStatus;
type NewEventRegistrationStatus = ModerateEventRegistrationNewEventRegistrationStatus;