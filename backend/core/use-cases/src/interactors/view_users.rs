use ::async_trait::async_trait;
use ::futures::prelude::*;

use crate::boundaries::*;
use crate::gateways::*;

#[derive(::bon::Builder)]
pub struct ViewUsersInteractor {
    user_repository: ::std::sync::Arc<dyn UserRepository + ::core::marker::Send + ::core::marker::Sync>,

    uuid_codec: ::std::sync::Arc<dyn UuidCodec + ::core::marker::Send + ::core::marker::Sync>,
    auth_token_generator: ::std::sync::Arc<dyn AuthenticationTokenGenerator + ::core::marker::Send + ::core::marker::Sync>,
}

#[async_trait]
impl ViewUsersBoundary for ViewUsersInteractor {
    async fn apply(
        self: ::std::sync::Arc<Self>, request: ViewUsersRequest,
    ) -> ::axiom::result::Fallible<ViewUsersResponse> {
        use ::axiom::time::TimestampExt as _;
        use ::axiom::option::IntoOptionExt as _;

        match ::std::sync::Arc::clone(&self.auth_token_generator).get_payload(request.token).await? {
            ::core::option::Option::None =>
                return ::axiom::err!(ViewUsers @ AuthenticationTokenInvalid),
            ::core::option::Option::Some
            (AuthenticationTokenPayload { user_id, user_role: ::domain::UserRole::Volunteer, expiry_timestamp }) => {
                if expiry_timestamp < ::axiom::time::Timestamp::now() {
                    return ::axiom::err!(ViewUsers @ AuthenticationTokenExpired);
                }

                if !::std::sync::Arc::clone(&self.user_repository).contains_id(user_id).await? {
                    return ::axiom::err!(ViewUsers @ UserNotFound);
                }
            },
            ::core::option::Option::Some(AuthenticationTokenPayload { user_role, .. }) =>
                return ::axiom::err!(ViewUsers @ UserUnauthorized { user_role: user_role.into() }),
        }

        let events: ::std::vec::Vec<::domain::Event> = ::std::sync::Arc::clone(&self.event_repository).view(request.filter.into()).await?;

        let events = ::futures::stream::iter(events)
            .filter_map(|event| {
                let uuid_codec = ::std::sync::Arc::clone(&self.uuid_codec);

                async move {
                    ViewUsersEvent::builder()
                        .id(uuid_codec.format(event.id).await.ok()?)
                        .status(*event.statuses.last())
                        .name(event.name)
                        .categories(event.categories)
                        .location(event.location)
                        .build()
                        .into_some()
                }
            })
            .collect::<::std::vec::Vec<_>>().await;
        
        let response = ViewUsersOkResponse::builder().events(events).build();
        ::axiom::ok!(ViewUsers @ response)
    }
}
