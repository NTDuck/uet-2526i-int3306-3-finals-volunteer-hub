use ::axiom::prelude::*;
use ::futures::prelude::*;

use crate::boundaries::*;
use crate::gateways::*;

#[derive(::bon::Builder)]
pub struct ViewEventChannelInteractor {
    event_repository: ::std::sync::Arc<dyn EventRepository + ::core::marker::Send + ::core::marker::Sync>,
    post_repository: ::std::sync::Arc<dyn EventPostRepository + ::core::marker::Send + ::core::marker::Sync>,
    reaction_repository: ::std::sync::Arc<dyn EventPostReactionRepository + ::core::marker::Send + ::core::marker::Sync>,
    comment_repository: ::std::sync::Arc<dyn EventPostCommentRepository + ::core::marker::Send + ::core::marker::Sync>,
    user_repository: ::std::sync::Arc<dyn UserRepository + ::core::marker::Send + ::core::marker::Sync>,

    uuid_codec: ::std::sync::Arc<dyn UuidCodec + ::core::marker::Send + ::core::marker::Sync>,
    timestamp_codec: ::std::sync::Arc<dyn TimestampCodec + ::core::marker::Send + ::core::marker::Sync>,
    uuid_generator: ::std::sync::Arc<dyn UuidGenerator + ::core::marker::Send + ::core::marker::Sync>,
    auth_token_generator:
        ::std::sync::Arc<dyn AuthenticationTokenGenerator + ::core::marker::Send + ::core::marker::Sync>,
}

#[async_trait]
impl ViewEventChannelBoundary for ViewEventChannelInteractor {
    async fn apply(
        self: ::std::sync::Arc<Self>, request: ViewEventChannelRequest,
    ) -> ::axiom::result::Fallible<ViewEventChannelResponse> {
        let actor_id = match ::std::sync::Arc::clone(&self.auth_token_generator).get_payload(request.token).await? {
            ::core::option::Option::None =>
                return ::axiom::err!(ViewEventChannel @ AuthenticationTokenInvalid),
            ::core::option::Option::Some
            (AuthenticationTokenPayload { user_id, user_role: ::domain::UserRole::Volunteer, expiry_timestamp }) => {
                if expiry_timestamp < ::axiom::time::Timestamp::now() {
                    return ::axiom::err!(ViewEventChannel @ AuthenticationTokenExpired);
                }

                user_id
            },
            ::core::option::Option::Some(AuthenticationTokenPayload { user_role, .. }) =>
                return ::axiom::err!(ViewEventChannel @ UserUnauthorized { user_role: user_role.into() }),
        };

        let ::core::option::Option::Some(actor) = ::std::sync::Arc::clone(&self.user_repository).get_by_id(actor_id).await? else {
            return ::axiom::err!(ViewEventChannel @ UserNotFound);
        };

        let event_id = ::std::sync::Arc::clone(&self.uuid_codec).parse(request.event_id).await?;

        match ::std::sync::Arc::clone(&self.event_repository).get_by_id(event_id).await? {
            ::core::option::Option::Some(event) => {
                if !::core::matches!(*event.statuses.last(), ::domain::EventStatus::Approved { .. }) {
                    return ::axiom::err!(ViewEventChannel @ EventChannelNotFound);
                }
            },
            ::core::option::Option::None => return ::axiom::err!(ViewEventChannel @ EventChannelNotFound),
        }

        let posts = ::std::sync::Arc::clone(&self.post_repository).view_by_event_id(event_id).await?;

        let posts = ::futures::stream::iter(posts)
            .zip(::futures::stream::repeat(actor))
            .then(|(post, actor)| {
                let reaction_repository = ::std::sync::Arc::clone(&self.reaction_repository);
                let comment_repository = ::std::sync::Arc::clone(&self.comment_repository);
                let uuid_generator = ::std::sync::Arc::clone(&self.uuid_generator);
                let uuid_codec = ::std::sync::Arc::clone(&self.uuid_codec);
                let timestamp_codec = ::std::sync::Arc::clone(&self.timestamp_codec);

                async move {
                    ViewEventChannelEventPost::build_from(post, actor)
                        .with_reaction_repository(reaction_repository)
                        .with_comment_repository(comment_repository)
                        .with_uuid_generator(uuid_generator)
                        .with_uuid_codec(uuid_codec)
                        .with_timestamp_codec(timestamp_codec)
                        .try_build().await
                }
            })
            .filter_map(|fallible| async move { fallible.ok() })
            .collect::<::std::vec::Vec<_>>().await;

        let response = ViewEventChannelOkResponse::builder().posts(posts).build();

        ::axiom::ok!(ViewEventChannel @ response)
    }
}
