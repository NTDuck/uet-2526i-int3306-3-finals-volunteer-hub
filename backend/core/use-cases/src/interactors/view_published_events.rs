use ::async_trait::async_trait;

use crate::boundaries::*;
use crate::gateways::*;

#[derive(::bon::Builder)]
pub struct ViewPublishedEventsInteractor {
    event_repository: ::std::sync::Arc<dyn EventRepository + ::core::marker::Send + ::core::marker::Sync>,
    user_repository: ::std::sync::Arc<dyn UserRepository + ::core::marker::Send + ::core::marker::Sync>,

    uuid_codec: ::std::sync::Arc<dyn UuidCodec + ::core::marker::Send + ::core::marker::Sync>,
    auth_token_generator: ::std::sync::Arc<dyn AuthenticationTokenGenerator + ::core::marker::Send + ::core::marker::Sync>,
}

#[async_trait]
impl ViewPublishedEventsBoundary for ViewPublishedEventsInteractor {
    async fn apply(
        self: ::std::sync::Arc<Self>, request: ViewPublishedEventsRequest,
    ) -> ::axiom::result::Fallible<ViewPublishedEventsResponse> {
        use ::axiom::time::TimestampExt as _;

        match ::std::sync::Arc::clone(&self.auth_token_generator).get_payload(request.token).await? {
            ::core::option::Option::None =>
                return ::axiom::result::Fallible::Ok(ViewPublishedEventsResponse::Err(::std::vec![ViewPublishedEventsErrResponse::AuthenticationTokenInvalid])),
            ::core::option::Option::Some
            (AuthenticationTokenPayload { user_id, user_role: ::domain::UserRole::Volunteer, expiry_timestamp }) => {
                if expiry_timestamp < ::axiom::time::Timestamp::now() {
                    return ::axiom::result::Fallible::Ok(ViewPublishedEventsResponse::Err(::std::vec![ViewPublishedEventsErrResponse::AuthenticationTokenExpired]));
                }

                if !::std::sync::Arc::clone(&self.user_repository).contains_id(user_id).await? {
                    return ::axiom::result::Fallible::Ok(ViewPublishedEventsResponse::Err(::std::vec![ViewPublishedEventsErrResponse::UserNotFound]));
                }
            },
            ::core::option::Option::Some(AuthenticationTokenPayload { user_role, .. }) =>
                return ::axiom::result::Fallible::Ok(ViewPublishedEventsResponse::Err(::std::vec![
                    ViewPublishedEventsErrResponse::UserUnauthorized { user_role: user_role.into() },
                ])),
        }

        let events: ::std::vec::Vec<::domain::Event> = ::std::sync::Arc::clone(&self.event_repository).view(request.filter.into()).await?;

        let events = ::futures::future::try_join_all(events.into_iter().map(|event| {
            let uuid_codec = ::std::sync::Arc::clone(&self.uuid_codec);
            
            async move {
                ::futures::future::ok::<_, ::axiom::result::Error>(
                    ViewPublishedEventsEvent::builder()
                        .id(uuid_codec.format(event.id).await?)
                        .status(*event.statuses.last())
                        .name(event.name)
                        .categories(event.categories.into_vec())
                        .location(event.location)
                        .build(),
                ).await
            }
        })).await?;
        
        let response = ViewPublishedEventsOkResponse::builder().events(events).build();
        ::axiom::result::Fallible::Ok(ViewPublishedEventsResponse::Ok(response))
    }
}