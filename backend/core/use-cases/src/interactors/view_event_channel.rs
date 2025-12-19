use ::axiom::prelude::*;
use ::futures::prelude::*;

use crate::boundaries::*;
use crate::gateways::*;

#[derive(::bon::Builder)]
pub struct ViewEventChannelInteractor {
    event_repository: ::std::sync::Arc<dyn EventRepository + ::core::marker::Send + ::core::marker::Sync>,
    post_repository: ::std::sync::Arc<dyn EventPostRepository + ::core::marker::Send + ::core::marker::Sync>,
    reaction_repository:
        ::std::sync::Arc<dyn EventPostReactionRepository + ::core::marker::Send + ::core::marker::Sync>,
    comment_repository: ::std::sync::Arc<dyn EventPostCommentRepository + ::core::marker::Send + ::core::marker::Sync>,
    user_repository: ::std::sync::Arc<dyn UserRepository + ::core::marker::Send + ::core::marker::Sync>,

    uuid_codec: ::std::sync::Arc<dyn UuidCodec + ::core::marker::Send + ::core::marker::Sync>,
    timestamp_codec: ::std::sync::Arc<dyn TimestampCodec + ::core::marker::Send + ::core::marker::Sync>,
    auth_token_generator: ::std::sync::Arc<dyn AuthTokenGenerator + ::core::marker::Send + ::core::marker::Sync>,
}

#[async_trait]
impl ViewEventChannelBoundary for ViewEventChannelInteractor {
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

                if !::std::sync::Arc::clone(&self.user_repository).contains_id(user_id).await? {
                    return super::err!(UserNotFound);
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

        match ::std::sync::Arc::clone(&self.event_repository).get_by_id(event_id).await? {
            ::core::option::Option::Some(::domain::Event { statuses, .. }) => {
                if !::core::matches!(statuses[..], [.., ::domain::EventStatus::Approved { .. }]) {
                    return super::err!(EventChannelNotFound);
                }
            },
            ::core::option::Option::None => return super::err!(EventChannelNotFound),
            // _ => {},
        }

        let posts = ::std::sync::Arc::clone(&self.post_repository)
            .view_by_event_id(event_id)
            .await?
            .into_stream()
            .then(|post| {
                let reaction_repository = ::std::sync::Arc::clone(&self.reaction_repository);
                let comment_repository = ::std::sync::Arc::clone(&self.comment_repository);
                let user_repository = ::std::sync::Arc::clone(&self.user_repository);
                let uuid_codec = ::std::sync::Arc::clone(&self.uuid_codec);
                let timestamp_codec = ::std::sync::Arc::clone(&self.timestamp_codec);

                async move {
                    EventPost::builder()
                        .id(::std::sync::Arc::clone(&uuid_codec).format(post.id).await?)
                        .last_updated_at(::std::sync::Arc::clone(&timestamp_codec).format(post.last_updated_at).await?)
                        .title(post.title)
                        .content(post.content)
                        .image_url(post.image_url)
                        .reaction_count(::std::sync::Arc::clone(&reaction_repository).count_by_post_id(post.id).await?)
                        .comment_count(::std::sync::Arc::clone(&comment_repository).count_by_post_id(post.id).await?)
                        .maybe_author(
                            ::std::sync::Arc::clone(&user_repository)
                                .get_by_id(post.author_id)
                                .await?
                                .map_async(|author| async {
                                    User::build_from(author)
                                        .with_uuid_codec(::std::sync::Arc::clone(&uuid_codec))
                                        .try_build()
                                        .await
                                })
                                .await
                                .transpose()?,
                        )
                        .is_reacted_by_actor(
                            ::std::sync::Arc::clone(&reaction_repository)
                                .contains_post_and_user_id(post.id, actor_id)
                                .await?,
                        )
                        .build()
                        .into_ok()
                }
            })
            .try_collect::<::std::vec::Vec<_>>()
            .await?;

        let response = OkResponse::builder().posts(posts).build();
        super::ok!(response)
    }
}

type Request = ViewEventChannelRequest;
type Response = ViewEventChannelResponse;
type OkResponse = ViewEventChannelOkResponse;
type ErrResponse = ViewEventChannelErrResponse;
type UserRole = ViewEventChannelUserRole;
type User = ViewEventChannelUser;
type EventPost = ViewEventChannelEventPost;
