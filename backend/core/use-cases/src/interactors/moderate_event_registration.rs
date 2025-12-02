use ::axiom::prelude::*;

use crate::boundaries::*;
use crate::gateways::*;

#[derive(::bon::Builder)]
pub struct ModerateEventRegistrationInteractor {
    event_registration_repository: ::std::sync::Arc<dyn EventRegistrationRepository + ::core::marker::Send + ::core::marker::Sync>,
    user_repository: ::std::sync::Arc<dyn UserRepository + ::core::marker::Send + ::core::marker::Sync>,

    uuid_codec: ::std::sync::Arc<dyn UuidCodec + ::core::marker::Send + ::core::marker::Sync>,
    auth_token_generator:
        ::std::sync::Arc<dyn AuthenticationTokenGenerator + ::core::marker::Send + ::core::marker::Sync>,
}

#[async_trait]
impl ModerateEventRegistrationBoundary for ModerateEventRegistrationInteractor {
    async fn apply(
        self: ::std::sync::Arc<Self>, request: ModerateEventRegistrationRequest,
    ) -> ::axiom::result::Fallible<ModerateEventRegistrationResponse> {
        let actor_id = match ::std::sync::Arc::clone(&self.auth_token_generator).get_payload(request.token).await? {
            ::core::option::Option::None =>
                return ::axiom::err!(ModerateEventRegistration @ AuthenticationTokenInvalid),
            ::core::option::Option::Some
            (AuthenticationTokenPayload { user_id, user_role: ::domain::UserRole::EventManager, expiry_timestamp }) => {
                if expiry_timestamp < ::axiom::time::Timestamp::now() {
                    return ::axiom::err!(ModerateEventRegistration @ AuthenticationTokenExpired);
                }

                match ::std::sync::Arc::clone(&self.user_repository).get_by_id(user_id).await? {
                    ::core::option::Option::Some(user) => {
                        if ::core::matches!(user.statuses.last(), ::domain::UserStatus::Suspended { .. }) {
                            return ::axiom::err!(ModerateEventRegistration @ UserSuspended);
                        }
                    },
                    ::core::option::Option::None =>
                        return ::axiom::err!(ModerateEventRegistration @ UserNotFound),
                }

                user_id
            },
            ::core::option::Option::Some(AuthenticationTokenPayload { user_role, .. }) =>
                return ::axiom::err!(ModerateEventRegistration @ UserUnauthorized { user_role: user_role.into(), allowed_user_roles: ::std::vec![ModerateEventRegistrationUserRole::EventManager] }),
        };

        let event_registration_id = ::std::sync::Arc::clone(&self.uuid_codec).parse(request.event_registration_id).await?;

        let ::core::option::Option::Some(mut event_registration) = ::std::sync::Arc::clone(&self.event_registration_repository).get_by_id(event_registration_id).await? else {
            return ::axiom::err!(ModerateEventRegistration @ EventRegistrationNotFound);
        };

        let event_registration_status = *event_registration.statuses.last();

        match request.event_registration_status {
            ModerateEventRegistrationNewEventRegistrationStatus::Accepted => {
                if !::core::matches!(event_registration_status, ::domain::EventRegistrationStatus::Pending { .. } | ::domain::EventRegistrationStatus::Declined { .. }) {
                    return ::axiom::err!(ModerateEventRegistration @ EventRegistrationStatusNotEligible {
                        event_registration_status: event_registration_status.into(),
                        allowed_event_registration_statuses: ::std::vec![
                            ModerateEventRegistrationEventRegistrationStatus::Pending,
                            ModerateEventRegistrationEventRegistrationStatus::Declined,
                        ],
                    });
                }

                event_registration.statuses.push(::domain::EventRegistrationStatus::Accepted { accepted_by_manager_id: actor_id, accepted_at: ::axiom::time::Timestamp::now() });
            },

            ModerateEventRegistrationNewEventRegistrationStatus::Declined => {
                if !::core::matches!(event_registration_status, ::domain::EventRegistrationStatus::Pending { .. } | ::domain::EventRegistrationStatus::Accepted { .. }) {
                    return ::axiom::err!(ModerateEventRegistration @ EventRegistrationStatusNotEligible {
                        event_registration_status: event_registration_status.into(),
                        allowed_event_registration_statuses: ::std::vec![
                            ModerateEventRegistrationEventRegistrationStatus::Pending,
                            ModerateEventRegistrationEventRegistrationStatus::Accepted,
                        ],
                    });
                }

                event_registration.statuses.push(::domain::EventRegistrationStatus::Declined { declined_by_manager_id: actor_id, declined_at: ::axiom::time::Timestamp::now() });
            },

            ModerateEventRegistrationNewEventRegistrationStatus::Completed => {
                if !::core::matches!(event_registration_status, ::domain::EventRegistrationStatus::Accepted { .. } | ::domain::EventRegistrationStatus::Declined { .. }) {
                    return ::axiom::err!(ModerateEventRegistration @ EventRegistrationStatusNotEligible {
                        event_registration_status: event_registration_status.into(),
                        allowed_event_registration_statuses: ::std::vec![
                            ModerateEventRegistrationEventRegistrationStatus::Accepted,
                            ModerateEventRegistrationEventRegistrationStatus::Declined,
                        ],
                    });
                }

                event_registration.statuses.push(::domain::EventRegistrationStatus::Completed { completed_by_manager_id: actor_id, completed_at: ::axiom::time::Timestamp::now() });
            },
        }

        ::std::sync::Arc::clone(&self.event_registration_repository).save(event_registration).await?;

        ::axiom::ok!(ModerateEventRegistration)
    }
}
