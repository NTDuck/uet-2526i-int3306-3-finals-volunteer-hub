use ::axiom::prelude::*;

use crate::boundaries::*;
use crate::gateways::*;

#[derive(::bon::Builder)]
pub struct SubscribeToEventInteractor {
    event_repository: ::std::sync::Arc<dyn EventRepository + ::core::marker::Send + ::core::marker::Sync>,
    event_recommender: ::std::sync::Arc<dyn EventRecommender + ::core::marker::Send + ::core::marker::Sync>,
    event_registration_repository: ::std::sync::Arc<dyn EventRegistrationRepository + ::core::marker::Send + ::core::marker::Sync>,
    user_repository: ::std::sync::Arc<dyn UserRepository + ::core::marker::Send + ::core::marker::Sync>,

    uuid_generator: ::std::sync::Arc<dyn UuidGenerator + ::core::marker::Send + ::core::marker::Sync>,
    uuid_codec: ::std::sync::Arc<dyn UuidCodec + ::core::marker::Send + ::core::marker::Sync>,
    auth_token_generator: ::std::sync::Arc<dyn AuthenticationTokenGenerator + ::core::marker::Send + ::core::marker::Sync>,
}

#[async_trait]
impl SubscribeToEventBoundary for SubscribeToEventInteractor {
    async fn apply(
        self: ::std::sync::Arc<Self>, request: SubscribeToEventRequest,
    ) -> ::axiom::result::Fallible<SubscribeToEventResponse> {
        let actor_id = match ::std::sync::Arc::clone(&self.auth_token_generator).get_payload(request.token).await? {
            ::core::option::Option::None =>
                return ::axiom::err!(SubscribeToEvent @ AuthenticationTokenInvalid),
            ::core::option::Option::Some
            (AuthenticationTokenPayload { user_id, user_role: ::domain::UserRole::Volunteer, expiry_timestamp }) => {
                if expiry_timestamp < ::axiom::time::Timestamp::now() {
                    return ::axiom::err!(SubscribeToEvent @ AuthenticationTokenExpired);
                }

                match ::std::sync::Arc::clone(&self.user_repository).get_by_id(user_id).await? {
                    ::core::option::Option::Some(user) => {
                        if ::core::matches!(user.statuses.last(), ::domain::UserStatus::Suspended { .. }) {
                            return ::axiom::err!(SubscribeToEvent @ UserSuspended);
                        }
                    },
                    ::core::option::Option::None =>
                        return ::axiom::err!(SubscribeToEvent @ UserNotFound),
                }

                user_id
            },
            ::core::option::Option::Some(AuthenticationTokenPayload { user_role, .. }) =>
                return ::axiom::err!(SubscribeToEvent @ UserUnauthorized { user_role: user_role.into(), allowed_user_roles: ::std::vec![SubscribeToEventUserRole::Volunteer] }),
        };

        let event_id = ::std::sync::Arc::clone(&self.uuid_codec).parse(request.event_id).await?;

        if !::std::sync::Arc::clone(&self.event_repository).contains_id(event_id).await? {
            return ::axiom::err!(SubscribeToEvent @ EventNotFound);
        }

        let event_registration = match ::std::sync::Arc::clone(&self.event_registration_repository).get_by_event_and_user_id(event_id, actor_id).await? {
            ::core::option::Option::Some(mut event_registration) => {
                let event_registration_status = event_registration.statuses.last();

                if !::core::matches!(event_registration_status, ::domain::EventRegistrationStatus::Withdrawn { .. }) {
                    return ::axiom::err!(SubscribeToEvent @ EventRegistrationStatusNotEligible { event_registration_status: (*event_registration_status).into(), allowed_event_registration_statuses: ::std::vec![SubscribeToEventEventRegistrationStatus::Withdrawn] });
                }

                event_registration.statuses.push(::domain::EventRegistrationStatus::Pending { pending_at: ::axiom::time::Timestamp::now() });

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
                    .statuses(::vec1::vec1!(::domain::EventRegistrationStatus::Pending { pending_at: ::axiom::time::Timestamp::now() }))
                    .build()
            },
        };

        ::std::sync::Arc::clone(&self.event_registration_repository).save(event_registration).await?;

        ::std::sync::Arc::clone(&self.event_recommender).track_subscribed(event_id, actor_id).await?;
        
        ::axiom::ok!(SubscribeToEvent)
    }
}
