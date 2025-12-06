use ::axiom::prelude::*;
use ::futures::prelude::*;

use crate::boundaries::*;
use crate::gateways::*;

#[derive(::bon::Builder)]
pub struct ViewEventPostInteractor {
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
impl ViewEventPostBoundary for ViewEventPostInteractor {
    async fn apply(self: ::std::sync::Arc<Self>, request: Request) -> ::axiom::result::Fallible<Response> {
        let actor_id = match ::std::sync::Arc::clone(&self.auth_token_generator)
            .get_payload(request.token)
            .await?
        {
            ::core::option::Option::None => return super::err!(AuthenticationTokenInvalid),
            ::core::option::Option::Some(AuthTokenPayload {
                user_id,
                user_role: ::domain::UserRole::Volunteer | ::domain::UserRole::EventManager,
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
                    allowed_user_roles: ::std::vec![UserRole::Volunteer, UserRole::EventManager]
                }),
        };

        let post_id = ::std::sync::Arc::clone(&self.uuid_codec).parse(request.post_id).await?;

        let ::core::option::Option::Some(post) =
            ::std::sync::Arc::clone(&self.post_repository).get_by_id(post_id).await?
        else {
            return super::err!(PostNotFound);
        };

        let response = OkResponse::builder()
            .id(::std::sync::Arc::clone(&self.uuid_codec).format(post.id).await?)
            .last_updated_at(
                ::std::sync::Arc::clone(&self.timestamp_codec)
                    .format(post.last_updated_at)
                    .await?,
            )
            .title(post.title)
            .content(post.content)
            .image_url(post.image_url)
            .reactions(
                ::std::sync::Arc::clone(&self.reaction_repository)
                    .view_by_post_id(post.id)
                    .await?
                    .into_stream()
                    .then(|reaction| {
                        let user_repository = ::std::sync::Arc::clone(&self.user_repository);

                        async move {
                            ::std::sync::Arc::clone(&user_repository)
                                .get_by_id(reaction.author_id)
                                .await
                                .map(|author| (reaction, author))
                        }
                    })
                    .and_then(|(reaction, author)| {
                        let uuid_codec = ::std::sync::Arc::clone(&self.uuid_codec);

                        async move {
                            EventPostReaction::build_from(reaction, author)
                                .with_uuid_codec(::std::sync::Arc::clone(&uuid_codec))
                                .try_build()
                                .await
                        }
                    })
                    .try_collect::<::std::vec::Vec<_>>()
                    .await?,
            )
            .comments(
                ::std::sync::Arc::clone(&self.comment_repository)
                    .view_by_post_id(post.id)
                    .await?
                    .into_stream()
                    .then(|comment| {
                        let user_repository = ::std::sync::Arc::clone(&self.user_repository);

                        async move {
                            ::std::sync::Arc::clone(&user_repository)
                                .get_by_id(comment.author_id)
                                .await
                                .map(|author| (comment, author))
                        }
                    })
                    .and_then(|(comment, author)| {
                        let uuid_codec = ::std::sync::Arc::clone(&self.uuid_codec);
                        let timestamp_codec = ::std::sync::Arc::clone(&self.timestamp_codec);

                        async move {
                            EventPostComment::build_from(comment, author)
                                .with_uuid_codec(::std::sync::Arc::clone(&uuid_codec))
                                .with_timestamp_codec(::std::sync::Arc::clone(&timestamp_codec))
                                .try_build()
                                .await
                        }
                    })
                    .try_collect::<::std::vec::Vec<_>>()
                    .await?,
            )
            .maybe_author(
                ::std::sync::Arc::clone(&self.user_repository)
                    .get_by_id(post.author_id)
                    .await?
                    .map_async(|author| {
                        let uuid_codec = ::std::sync::Arc::clone(&self.uuid_codec);

                        async move {
                            User::build_from(author)
                                .with_uuid_codec(::std::sync::Arc::clone(&uuid_codec))
                                .try_build()
                                .await
                        }
                    })
                    .await
                    .transpose()?,
            )
            .is_reacted_by_actor(
                ::std::sync::Arc::clone(&self.reaction_repository)
                    .contains_post_and_user_id(post.id, actor_id)
                    .await?,
            )
            .build();

        super::ok!(response)
    }
}

type Request = ViewEventPostRequest;
type Response = ViewEventPostResponse;
type OkResponse = ViewEventPostOkResponse;
type ErrResponse = ViewEventPostErrResponse;
type UserRole = ViewEventPostUserRole;
type User = ViewEventPostUser;
type EventPostReaction = ViewEventPostEventPostReaction;
type EventPostComment = ViewEventPostEventPostComment;
