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
    uuid_generator: ::std::sync::Arc<dyn UuidGenerator + ::core::marker::Send + ::core::marker::Sync>,
    auth_token_generator:
        ::std::sync::Arc<dyn AuthenticationTokenGenerator + ::core::marker::Send + ::core::marker::Sync>,
}

#[async_trait]
impl ViewEventPostBoundary for ViewEventPostInteractor {
    async fn apply(
        self: ::std::sync::Arc<Self>, request: ViewEventPostRequest,
    ) -> ::axiom::result::Fallible<ViewEventPostResponse> {
        let actor_id = match ::std::sync::Arc::clone(&self.auth_token_generator)
            .get_payload(request.token)
            .await?
        {
            ::core::option::Option::None => return ::axiom::err!(ViewEventPost @ AuthenticationTokenInvalid),
            ::core::option::Option::Some(AuthenticationTokenPayload {
                user_id,
                user_role: ::domain::UserRole::Volunteer | ::domain::UserRole::EventManager,
                expiry_timestamp,
            }) => {
                if expiry_timestamp < ::axiom::time::Timestamp::now() {
                    return ::axiom::err!(ViewEventPost @ AuthenticationTokenExpired);
                }

                if !::std::sync::Arc::clone(&self.user_repository).contains_id(user_id).await? {
                    return ::axiom::err!(ViewEventPost @ UserNotFound);
                }

                user_id
            },
            ::core::option::Option::Some(AuthenticationTokenPayload { user_role, .. }) =>
                return ::axiom::err!(ViewEventPost @ UserUnauthorized { user_role: user_role.into(), allowed_user_roles: ::std::vec![ViewEventPostUserRole::Volunteer, ViewEventPostUserRole::EventManager] }),
        };

        let post_id = ::std::sync::Arc::clone(&self.uuid_codec).parse(request.post_id).await?;

        let ::core::option::Option::Some(post) =
            ::std::sync::Arc::clone(&self.post_repository).get_by_id(post_id).await?
        else {
            return ::axiom::err!(ViewEventPost @ PostNotFound);
        };

        let response = ViewEventPostOkResponse::builder()
            .id(::std::sync::Arc::clone(&self.uuid_codec).format(post.id).await?)
            .created_at(
                ::std::sync::Arc::clone(&self.timestamp_codec)
                    .format(::std::sync::Arc::clone(&self.uuid_generator).get_timestamp(post.id).await?)
                    .await?,
            )
            .title(post.title)
            .content(post.content)
            .reactions(
                ::std::sync::Arc::clone(&self.reaction_repository)
                    .view_by_post_id(post.id)
                    .await?
                    .into_stream()
                    .then(|reaction| async {
                        ::std::sync::Arc::clone(&self.user_repository)
                            .get_by_id(reaction.author_id)
                            .await
                            .map(|author| (reaction, author))
                    })
                    .and_then(|(reaction, author)| async {
                        ViewEventPostEventPostReaction::build_from(reaction, author)
                            .with_uuid_generator(::std::sync::Arc::clone(&self.uuid_generator))
                            .with_uuid_codec(::std::sync::Arc::clone(&self.uuid_codec))
                            .with_timestamp_codec(::std::sync::Arc::clone(&self.timestamp_codec))
                            .try_build()
                            .await
                    })
                    .try_collect::<::std::vec::Vec<_>>()
                    .await?,
            )
            .comments(
                ::std::sync::Arc::clone(&self.comment_repository)
                    .view_by_post_id(post.id)
                    .await?
                    .into_stream()
                    .then(|comment| async {
                        ::std::sync::Arc::clone(&self.user_repository)
                            .get_by_id(comment.author_id)
                            .await
                            .map(|author| (comment, author))
                    })
                    .and_then(|(comment, author)| async {
                        ViewEventPostEventPostComment::build_from(comment, author)
                            .with_uuid_generator(::std::sync::Arc::clone(&self.uuid_generator))
                            .with_uuid_codec(::std::sync::Arc::clone(&self.uuid_codec))
                            .with_timestamp_codec(::std::sync::Arc::clone(&self.timestamp_codec))
                            .try_build()
                            .await
                    })
                    .try_collect::<::std::vec::Vec<_>>()
                    .await?,
            )
            .maybe_author(
                ::std::sync::Arc::clone(&self.user_repository)
                    .get_by_id(post.author_id)
                    .await?
                    .map_async(|author| async {
                        ViewEventPostUser::build_from(author)
                            .with_uuid_codec(::std::sync::Arc::clone(&self.uuid_codec))
                            .try_build()
                            .await
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

        ::axiom::ok!(ViewEventPost @ response)
    }
}
