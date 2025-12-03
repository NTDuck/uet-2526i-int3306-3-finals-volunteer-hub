use ::axiom::prelude::*;

use crate::boundaries::*;
use crate::gateways::*;

#[derive(::bon::Builder)]
pub struct ModerateEventInteractor {
    event_repository: ::std::sync::Arc<dyn EventRepository + ::core::marker::Send + ::core::marker::Sync>,
    event_recommender: ::std::sync::Arc<dyn EventRecommender + ::core::marker::Send + ::core::marker::Sync>,
    user_repository: ::std::sync::Arc<dyn UserRepository + ::core::marker::Send + ::core::marker::Sync>,

    uuid_codec: ::std::sync::Arc<dyn UuidCodec + ::core::marker::Send + ::core::marker::Sync>,
    auth_token_generator:
        ::std::sync::Arc<dyn AuthenticationTokenGenerator + ::core::marker::Send + ::core::marker::Sync>,
}

#[async_trait]
impl ModerateEventBoundary for ModerateEventInteractor {
    async fn apply(
        self: ::std::sync::Arc<Self>, request: ModerateEventRequest,
    ) -> ::axiom::result::Fallible<ModerateEventResponse> {
        let actor_id = match ::std::sync::Arc::clone(&self.auth_token_generator)
            .get_payload(request.token)
            .await?
        {
            ::core::option::Option::None => return ::axiom::err!(ModerateEvent @ AuthenticationTokenInvalid),
            ::core::option::Option::Some(AuthenticationTokenPayload {
                user_id,
                user_role: ::domain::UserRole::Administrator,
                expiry_timestamp,
            }) => {
                if expiry_timestamp < ::axiom::time::Timestamp::now() {
                    return ::axiom::err!(ModerateEvent @ AuthenticationTokenExpired);
                }

                if !::std::sync::Arc::clone(&self.user_repository).contains_id(user_id).await? {
                    return ::axiom::err!(ModerateEvent @ UserNotFound);
                }

                user_id
            },
            ::core::option::Option::Some(AuthenticationTokenPayload { user_role, .. }) =>
                return ::axiom::err!(ModerateEvent @ UserUnauthorized { user_role: user_role.into(), allowed_user_roles: ::std::vec![ModerateEventUserRole::Administrator] }),
        };

        let event_id = ::std::sync::Arc::clone(&self.uuid_codec).parse(request.event_id).await?;
        let ::core::option::Option::Some(mut event) =
            ::std::sync::Arc::clone(&self.event_repository).get_by_id(event_id).await?
        else {
            return ::axiom::err!(ModerateEvent @ EventNotFound);
        };

        let event_status = *event.statuses.last();

        match request.event_status {
            ModerateEventNewEventStatus::Approved => {
                if !::core::matches!(
                    event_status,
                    ::domain::EventStatus::Created { .. }
                        | ::domain::EventStatus::Updated { .. }
                        | ::domain::EventStatus::Approved { .. }
                ) {
                    return ::axiom::err!(ModerateEvent @ EventStatusNotEligible {
                        event_status: event_status.into(),
                        allowed_event_statuses: ::std::vec![
                            ModerateEventEventStatus::Created,
                            ModerateEventEventStatus::Updated,
                            ModerateEventEventStatus::Approved,
                        ],
                    });
                }

                event.statuses.push(::domain::EventStatus::Approved {
                    approved_by_administrator_id: actor_id,
                    approved_at: ::axiom::time::Timestamp::now(),
                });

                ::std::sync::Arc::clone(&self.event_recommender)
                    .track_approved(event_id)
                    .await?;
            },
            ModerateEventNewEventStatus::Rejected => {
                if !::core::matches!(
                    event_status,
                    ::domain::EventStatus::Created { .. }
                        | ::domain::EventStatus::Updated { .. }
                        | ::domain::EventStatus::Rejected { .. }
                ) {
                    return ::axiom::err!(ModerateEvent @ EventStatusNotEligible {
                        event_status: event_status.into(),
                        allowed_event_statuses: ::std::vec![
                            ModerateEventEventStatus::Created,
                            ModerateEventEventStatus::Updated,
                            ModerateEventEventStatus::Rejected,
                        ],
                    });
                }

                event.statuses.push(::domain::EventStatus::Rejected {
                    rejected_by_administrator_id: actor_id,
                    rejected_at: ::axiom::time::Timestamp::now(),
                });

                ::std::sync::Arc::clone(&self.event_recommender)
                    .untrack_approved(event_id)
                    .await?;
            },
        }

        ::std::sync::Arc::clone(&self.event_repository).save(event).await?;

        ::axiom::ok!(ModerateEvent)
    }
}
