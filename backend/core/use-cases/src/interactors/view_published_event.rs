use ::axiom::prelude::*;

use crate::boundaries::*;
use crate::gateways::*;

#[derive(::bon::Builder)]
pub struct ViewPublishedEventInteractor {
    event_repository: ::std::sync::Arc<dyn EventRepository + ::core::marker::Send + ::core::marker::Sync>,
    user_repository: ::std::sync::Arc<dyn UserRepository + ::core::marker::Send + ::core::marker::Sync>,

    uuid_codec: ::std::sync::Arc<dyn UuidCodec + ::core::marker::Send + ::core::marker::Sync>,
    timestamp_codec: ::std::sync::Arc<dyn TimestampCodec + ::core::marker::Send + ::core::marker::Sync>,
    auth_token_generator: ::std::sync::Arc<dyn AuthTokenGenerator + ::core::marker::Send + ::core::marker::Sync>,
}

#[async_trait]
impl ViewPublishedEventBoundary for ViewPublishedEventInteractor {
    async fn apply(self: ::std::sync::Arc<Self>, request: Request) -> ::axiom::result::Fallible<Response> {
        match ::std::sync::Arc::clone(&self.auth_token_generator)
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
            },
            ::core::option::Option::Some(AuthTokenPayload { user_role, .. }) =>
                return super::err!(UserUnauthorized {
                    user_role: user_role.into(),
                    allowed_user_roles: ::std::vec![UserRole::Volunteer]
                }),
        }

        let event_id = ::std::sync::Arc::clone(&self.uuid_codec).parse(request.event_id).await?;
        let ::core::option::Option::Some(event) = ::std::sync::Arc::clone(&self.event_repository).get_by_id(event_id).await? else {
            return super::err!(EventNotFound);
        };

        let event = Event::build_from(event)
            .with_uuid_codec(::std::sync::Arc::clone(&self.uuid_codec))
            .with_timestamp_codec(::std::sync::Arc::clone(&self.timestamp_codec))
            .try_build()
            .await?;

        let response = OkResponse::builder().event(event).build();
        super::ok!(response)
    }
}

type Request = ViewPublishedEventRequest;
type Response = ViewPublishedEventResponse;
type OkResponse = ViewPublishedEventOkResponse;
type ErrResponse = ViewPublishedEventErrResponse;
type UserRole = ViewPublishedEventUserRole;
type Event = ViewPublishedEventEvent;
