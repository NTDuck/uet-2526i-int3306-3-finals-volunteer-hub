use ::axiom::prelude::*;

use crate::boundaries::*;
use crate::gateways::*;

#[derive(::bon::Builder)]
pub struct UnsubscribeFromEventInteractor {
    event_recommender: ::std::sync::Arc<dyn EventRecommender + ::core::marker::Send + ::core::marker::Sync>,
    event_registration_repository:
        ::std::sync::Arc<dyn EventRegistrationRepository + ::core::marker::Send + ::core::marker::Sync>,
    user_repository: ::std::sync::Arc<dyn UserRepository + ::core::marker::Send + ::core::marker::Sync>,

    uuid_codec: ::std::sync::Arc<dyn UuidCodec + ::core::marker::Send + ::core::marker::Sync>,
    auth_token_generator:
        ::std::sync::Arc<dyn AuthenticationTokenGenerator + ::core::marker::Send + ::core::marker::Sync>,
}

#[async_trait]
impl UnsubscribeFromEventBoundary for UnsubscribeFromEventInteractor {
    async fn apply(
        self: ::std::sync::Arc<Self>, request: UnsubscribeFromEventRequest,
    ) -> ::axiom::result::Fallible<UnsubscribeFromEventResponse> {
        let actor_id = match ::std::sync::Arc::clone(&self.auth_token_generator)
            .get_payload(request.token)
            .await?
        {
            ::core::option::Option::None => return ::axiom::err!(UnsubscribeFromEvent @ AuthenticationTokenInvalid),
            ::core::option::Option::Some(AuthenticationTokenPayload {
                user_id,
                user_role: ::domain::UserRole::Volunteer,
                expiry_timestamp,
            }) => {
                if expiry_timestamp < ::axiom::time::Timestamp::now() {
                    return ::axiom::err!(UnsubscribeFromEvent @ AuthenticationTokenExpired);
                }

                match ::std::sync::Arc::clone(&self.user_repository).get_by_id(user_id).await? {
                    ::core::option::Option::Some(user) => {
                        if ::core::matches!(user.statuses.last(), ::domain::UserStatus::Suspended { .. }) {
                            return ::axiom::err!(UnsubscribeFromEvent @ UserSuspended);
                        }
                    },
                    ::core::option::Option::None => return ::axiom::err!(UnsubscribeFromEvent @ UserNotFound),
                }

                user_id
            },
            ::core::option::Option::Some(AuthenticationTokenPayload { user_role, .. }) =>
                return ::axiom::err!(UnsubscribeFromEvent @ UserUnauthorized { user_role: user_role.into(), allowed_user_roles: ::std::vec![UnsubscribeFromEventUserRole::Volunteer] }),
        };

        let event_or_registration_id = ::std::sync::Arc::clone(&self.uuid_codec)
            .parse(request.event_or_registration_id)
            .await?;

        let ::core::option::Option::Some(mut event_registration) =
            ::std::sync::Arc::clone(&self.event_registration_repository)
                .get_by_id(event_or_registration_id)
                .await?
                .try_or_else_async(|| async {
                    ::std::sync::Arc::clone(&self.event_registration_repository)
                        .get_by_event_and_user_id(event_or_registration_id, actor_id)
                        .await
                })
                .await?
        else {
            return ::axiom::err!(UnsubscribeFromEvent @ EventRegistrationNotFound);
        };

        let event_registration_status = event_registration.statuses.last();

        if !::core::matches!(event_registration_status, ::domain::EventRegistrationStatus::Pending { .. }) {
            return ::axiom::err!(UnsubscribeFromEvent @ EventRegistrationStatusNotEligible { event_registration_status: (*event_registration_status).into(), allowed_event_registration_statuses: ::std::vec![UnsubscribeFromEventEventRegistrationStatus::Pending] });
        }

        event_registration.statuses.push(::domain::EventRegistrationStatus::Withdrawn {
            withdrawn_at: ::axiom::time::Timestamp::now(),
        });

        let event_id = event_registration.event_id;

        ::std::sync::Arc::clone(&self.event_registration_repository)
            .save(event_registration)
            .await?;

        ::std::sync::Arc::clone(&self.event_recommender)
            .untrack_subscribed(event_id, actor_id)
            .await?;

        ::axiom::ok!(UnsubscribeFromEvent)
    }
}
