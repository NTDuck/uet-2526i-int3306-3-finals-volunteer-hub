use ::axiom::prelude::*;
use ::futures::prelude::*;

use crate::boundaries::*;
use crate::gateways::*;

#[derive(::bon::Builder)]
pub struct ViewEventHistoryInteractor {
    event_repository: ::std::sync::Arc<dyn EventRepository + ::core::marker::Send + ::core::marker::Sync>,
    event_registration_repository:
        ::std::sync::Arc<dyn EventRegistrationRepository + ::core::marker::Send + ::core::marker::Sync>,
    user_repository: ::std::sync::Arc<dyn UserRepository + ::core::marker::Send + ::core::marker::Sync>,

    uuid_codec: ::std::sync::Arc<dyn UuidCodec + ::core::marker::Send + ::core::marker::Sync>,
    timestamp_codec: ::std::sync::Arc<dyn TimestampCodec + ::core::marker::Send + ::core::marker::Sync>,
    auth_token_generator:
        ::std::sync::Arc<dyn AuthTokenGenerator + ::core::marker::Send + ::core::marker::Sync>,
}

#[async_trait]
impl ViewEventHistoryBoundary for ViewEventHistoryInteractor {
    async fn apply(
        self: ::std::sync::Arc<Self>, request: Request,
    ) -> ::axiom::result::Fallible<Response> {
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

                if !::std::sync::Arc::clone(&self.user_repository).contains_id(user_id).await? {
                    return super::err!(UserNotFound);
                }

                user_id
            },
            ::core::option::Option::Some(AuthTokenPayload { user_role, .. }) =>
                return super::err!(UserUnauthorized { user_role: user_role.into(), allowed_user_roles: ::std::vec![UserRole::Volunteer] }),
        };

        let events = ::std::sync::Arc::clone(&self.event_registration_repository)
            .view_by_volunteer_id(actor_id)
            .await?
            .into_stream()
            .map(|event_registration| (event_registration.event_id, *event_registration.statuses.last()))
            .then(|(event_id, event_registration_status)| {
                let event_repository = ::std::sync::Arc::clone(&self.event_repository);

                async move {
                    ::std::sync::Arc::clone(&event_repository)
                        .get_by_id(event_id)
                        .await?
                        .map(|event| (event, event_registration_status))
                        .into_ok()
                }
            })
            .filter_map(|transposable| async move { transposable.transpose() })
            .and_then(|(event, event_registration_status)| {
                let uuid_codec = ::std::sync::Arc::clone(&self.uuid_codec);
                let timestamp_codec = ::std::sync::Arc::clone(&self.timestamp_codec);

                async move {
                    Event::build_from(event, event_registration_status)
                        .with_uuid_codec(::std::sync::Arc::clone(&uuid_codec))
                        .with_timestamp_codec(::std::sync::Arc::clone(&timestamp_codec))
                        .try_build()
                        .await
                }
            })
            .try_collect::<::std::vec::Vec<_>>()
            .await?;

        let response = OkResponse::builder().events(events).build();
        super::ok!(response)
    }
}

type Request = ViewEventHistoryRequest;
type Response = ViewEventHistoryResponse;
type OkResponse = ViewEventHistoryOkResponse;
type ErrResponse = ViewEventHistoryErrResponse;
type UserRole = ViewEventHistoryUserRole;
type Event = ViewEventHistoryEvent;
