use ::axiom::prelude::*;
use ::futures::prelude::*;

use crate::boundaries::*;
use crate::gateways::*;

#[derive(::bon::Builder)]
pub struct ViewEventsInteractor {
    event_repository: ::std::sync::Arc<dyn EventRepository + ::core::marker::Send + ::core::marker::Sync>,
    user_repository: ::std::sync::Arc<dyn UserRepository + ::core::marker::Send + ::core::marker::Sync>,

    uuid_codec: ::std::sync::Arc<dyn UuidCodec + ::core::marker::Send + ::core::marker::Sync>,
    timestamp_codec: ::std::sync::Arc<dyn TimestampCodec + ::core::marker::Send + ::core::marker::Sync>,
    auth_token_generator:
        ::std::sync::Arc<dyn AuthenticationTokenGenerator + ::core::marker::Send + ::core::marker::Sync>,
}

#[async_trait]
impl ViewEventsBoundary for ViewEventsInteractor {
    async fn apply(
        self: ::std::sync::Arc<Self>, request: ViewEventsRequest,
    ) -> ::axiom::result::Fallible<ViewEventsResponse> {
        match ::std::sync::Arc::clone(&self.auth_token_generator)
            .get_payload(request.token)
            .await?
        {
            ::core::option::Option::None => return ::axiom::err!(ViewEvents @ AuthenticationTokenInvalid),
            ::core::option::Option::Some(AuthenticationTokenPayload {
                user_id,
                user_role: ::domain::UserRole::EventManager | ::domain::UserRole::Administrator,
                expiry_timestamp,
            }) => {
                if expiry_timestamp < ::axiom::time::Timestamp::now() {
                    return ::axiom::err!(ViewEvents @ AuthenticationTokenExpired);
                }

                if !::std::sync::Arc::clone(&self.user_repository).contains_id(user_id).await? {
                    return ::axiom::err!(ViewEvents @ UserNotFound);
                }
            },
            ::core::option::Option::Some(AuthenticationTokenPayload { user_role, .. }) =>
                return ::axiom::err!(ViewEvents @ UserUnauthorized { user_role: user_role.into(), allowed_user_roles: ::std::vec![ViewEventsUserRole::EventManager, ViewEventsUserRole::Administrator] }),
        }

        let events = match request.filter {
            ::core::option::Option::None => ::std::sync::Arc::clone(&self.event_repository).view().await?,
            ::core::option::Option::Some(filter) => {
                let filter = filter
                    .build_into()
                    .with_timestamp_codec(::std::sync::Arc::clone(&self.timestamp_codec))
                    .try_build()
                    .await?;

                ::std::sync::Arc::clone(&self.event_repository).search(filter).await?
            },
        };

        let events = events
            .into_stream()
            .then(|event| {
                let uuid_codec = ::std::sync::Arc::clone(&self.uuid_codec);

                async move { ViewEventsEvent::build_from(event).with_uuid_codec(uuid_codec).try_build().await }
            })
            .try_collect::<::std::vec::Vec<_>>()
            .await?;

        let response = ViewEventsOkResponse::builder().events(events).build();
        ::axiom::ok!(ViewEvents @ response)
    }
}
