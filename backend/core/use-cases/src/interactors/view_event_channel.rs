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
    uuid_generator: ::std::sync::Arc<dyn UuidGenerator + ::core::marker::Send + ::core::marker::Sync>,
    auth_token_generator:
        ::std::sync::Arc<dyn AuthenticationTokenGenerator + ::core::marker::Send + ::core::marker::Sync>,
}

#[async_trait]
impl ViewEventChannelBoundary for ViewEventChannelInteractor {
    async fn apply(
        self: ::std::sync::Arc<Self>, request: ViewEventChannelRequest,
    ) -> ::axiom::result::Fallible<ViewEventChannelResponse> {
        let actor_id = match ::std::sync::Arc::clone(&self.auth_token_generator)
            .get_payload(request.token)
            .await?
        {
            ::core::option::Option::None => return ::axiom::err!(ViewEventChannel @ AuthenticationTokenInvalid),
            ::core::option::Option::Some(AuthenticationTokenPayload {
                user_id,
                user_role: ::domain::UserRole::Volunteer,
                expiry_timestamp,
            }) => {
                if expiry_timestamp < ::axiom::time::Timestamp::now() {
                    return ::axiom::err!(ViewEventChannel @ AuthenticationTokenExpired);
                }

                if !::std::sync::Arc::clone(&self.user_repository).contains_id(user_id).await? {
                    return ::axiom::err!(ViewEventChannel @ UserNotFound);
                }

                user_id
            },
            ::core::option::Option::Some(AuthenticationTokenPayload { user_role, .. }) =>
                return ::axiom::err!(ViewEventChannel @ UserUnauthorized { user_role: user_role.into(), allowed_user_roles: ::std::vec![ViewEventChannelUserRole::Volunteer] }),
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

        let posts = ::std::sync::Arc::clone(&self.post_repository)
            .view_by_event_id(event_id)
            .await?
            .into_stream()
            .zip(::futures::stream::repeat(::std::sync::Arc::clone(&self.user_repository).get_by_id(actor_id).await?))
            .then(|(post, actor)| {
                let reaction_repository = ::std::sync::Arc::clone(&self.reaction_repository);
                let comment_repository = ::std::sync::Arc::clone(&self.comment_repository);
                let user_repository = ::std::sync::Arc::clone(&self.user_repository);
                let uuid_generator = ::std::sync::Arc::clone(&self.uuid_generator);
                let uuid_codec = ::std::sync::Arc::clone(&self.uuid_codec);
                let timestamp_codec = ::std::sync::Arc::clone(&self.timestamp_codec);

                async move {
                    ViewEventChannelEventPost::builder()
                        .id(::std::sync::Arc::clone(&uuid_codec).format(post.id).await?)
                        .created_at(
                            ::std::sync::Arc::clone(&timestamp_codec)
                                .format(::std::sync::Arc::clone(&uuid_generator).get_timestamp(post.id).await?)
                                .await?,
                        )
                        .title(post.title)
                        .content(post.content)
                        .reaction_count(::std::sync::Arc::clone(&reaction_repository).count_by_post_id(post.id).await?)
                        .comment_count(::std::sync::Arc::clone(&comment_repository).count_by_post_id(post.id).await?)
                        .maybe_author(
                            ::std::sync::Arc::clone(&user_repository)
                                .get_by_id(post.author_id)
                                .await?
                                .map_async(|author| async {
                                    ViewEventChannelUser::build_from(author)
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
                        .comments_by_actor(
                            ::futures::stream::iter(
                                ::std::sync::Arc::clone(&comment_repository)
                                    .view_by_post_and_user_id(post.id, actor_id)
                                    .await?,
                            )
                            .zip(::futures::stream::repeat(actor))
                            .then(|(comment, actor)| async {
                                ViewEventChannelEventPostComment::build_from(comment, actor)
                                    .with_uuid_generator(::std::sync::Arc::clone(&uuid_generator))
                                    .with_uuid_codec(::std::sync::Arc::clone(&uuid_codec))
                                    .with_timestamp_codec(::std::sync::Arc::clone(&timestamp_codec))
                                    .try_build()
                                    .await
                            })
                            .filter_map(|fallible| async move { fallible.ok() })
                            .collect::<::std::vec::Vec<_>>()
                            .await,
                        )
                        .build()
                        .into_ok()
                }
            })
            .try_collect::<::std::vec::Vec<_>>()
            .await?;

        let response = ViewEventChannelOkResponse::builder().posts(posts).build();

        ::axiom::ok!(ViewEventChannel @ response)
    }
}
