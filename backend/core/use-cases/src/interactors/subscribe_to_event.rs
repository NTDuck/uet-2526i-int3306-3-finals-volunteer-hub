use ::axiom::prelude::*;

use crate::boundaries::*;
use crate::gateways::*;

#[derive(::bon::Builder)]
pub struct SubscribeToEventInteractor {
    event_repository: ::std::sync::Arc<dyn EventRepository + ::core::marker::Send + ::core::marker::Sync>,
    event_recommender: ::std::sync::Arc<dyn EventRecommender + ::core::marker::Send + ::core::marker::Sync>,
    event_registration_repository:
        ::std::sync::Arc<dyn EventRegistrationRepository + ::core::marker::Send + ::core::marker::Sync>,
    user_repository: ::std::sync::Arc<dyn UserRepository + ::core::marker::Send + ::core::marker::Sync>,

    uuid_generator: ::std::sync::Arc<dyn UuidGenerator + ::core::marker::Send + ::core::marker::Sync>,
    uuid_codec: ::std::sync::Arc<dyn UuidCodec + ::core::marker::Send + ::core::marker::Sync>,
    auth_token_generator: ::std::sync::Arc<dyn AuthTokenGenerator + ::core::marker::Send + ::core::marker::Sync>,
}

#[async_trait]
impl SubscribeToEventBoundary for SubscribeToEventInteractor {
    async fn apply(self: ::std::sync::Arc<Self>, request: Request) -> ::axiom::result::Fallible<Response> {
        let actor_id = match ::std::sync::Arc::clone(&self.auth_token_generator)
            .get_payload(request.token)
            .await?
        {
            ::core::option::Option::None => return super::err!(AuthenticationTokenInvalid),
            ::core::option::Option::Some(AuthTokenPayload {
                user_id,
                user_role: ::domain::UserRole::Volunteer,
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
                    allowed_user_roles: ::std::vec![UserRole::Volunteer]
                }),
        };

        let event_id = ::std::sync::Arc::clone(&self.uuid_codec).parse(request.event_id).await?;

        if !::std::sync::Arc::clone(&self.event_repository).contains_id(event_id).await? {
            return super::err!(EventNotFound);
        }

        let event_registration = match ::std::sync::Arc::clone(&self.event_registration_repository)
            .get_by_event_and_volunteer_id(event_id, actor_id)
            .await?
        {
            ::core::option::Option::Some(mut event_registration) => {
                let event_registration_status = *event_registration.statuses.last();

                if !::core::matches!(event_registration_status, ::domain::EventRegistrationStatus::Withdrawn { .. }) {
                    return super::err!(EventRegistrationStatusNotEligible {
                        event_registration_status: event_registration_status.into(),
                        allowed_event_registration_statuses: ::std::vec![EventRegistrationStatus::Withdrawn,],
                    });
                }

                event_registration.statuses.push(::domain::EventRegistrationStatus::Pending {
                    pending_at: ::axiom::time::Timestamp::now(),
                });

                event_registration
            },

            ::core::option::Option::None => {
                let event_registration_id = loop {
                    let uuid = ::std::sync::Arc::clone(&self.uuid_generator).generate().await?;
                    if !::std::sync::Arc::clone(&self.user_repository).contains_id(uuid).await? {
                        break uuid;
                    }
                };

                ::domain::EventRegistration::builder()
                    .id(event_registration_id)
                    .event_id(event_id)
                    .volunteer_id(actor_id)
                    .statuses(::vec1::vec1!(::domain::EventRegistrationStatus::Pending {
                        pending_at: ::axiom::time::Timestamp::now()
                    }))
                    .build()
            },
        };

        ::std::sync::Arc::clone(&self.event_registration_repository)
            .save(event_registration)
            .await?;

        ::std::sync::Arc::clone(&self.event_recommender)
            .track_subscribed(event_id, actor_id)
            .await?;

        super::ok!(())
    }
}

type Request = SubscribeToEventRequest;
type Response = SubscribeToEventResponse;
type ErrResponse = SubscribeToEventErrResponse;
type UserRole = SubscribeToEventUserRole;
type EventRegistrationStatus = SubscribeToEventEventRegistrationStatus;
