use ::axiom::prelude::*;

use crate::boundaries::*;
use crate::gateways::*;

#[derive(::bon::Builder)]
pub struct RemoveEventPostCommentInteractor {
    event_recommender: ::std::sync::Arc<dyn EventRecommender + ::core::marker::Send + ::core::marker::Sync>,
    post_repository: ::std::sync::Arc<dyn EventPostRepository + ::core::marker::Send + ::core::marker::Sync>,
    comment_repository: ::std::sync::Arc<dyn EventPostCommentRepository + ::core::marker::Send + ::core::marker::Sync>,
    user_repository: ::std::sync::Arc<dyn UserRepository + ::core::marker::Send + ::core::marker::Sync>,

    uuid_codec: ::std::sync::Arc<dyn UuidCodec + ::core::marker::Send + ::core::marker::Sync>,
    auth_token_generator:
        ::std::sync::Arc<dyn AuthenticationTokenGenerator + ::core::marker::Send + ::core::marker::Sync>,
}

#[async_trait]
impl RemoveEventPostCommentBoundary for RemoveEventPostCommentInteractor {
    async fn apply(
        self: ::std::sync::Arc<Self>, request: RemoveEventPostCommentRequest,
    ) -> ::axiom::result::Fallible<RemoveEventPostCommentResponse> {
        let actor_id = match ::std::sync::Arc::clone(&self.auth_token_generator)
            .get_payload(request.token)
            .await?
        {
            ::core::option::Option::None => return ::axiom::err!(RemoveEventPostComment @ AuthenticationTokenInvalid),
            ::core::option::Option::Some(AuthenticationTokenPayload {
                user_id,
                user_role: ::domain::UserRole::Volunteer | ::domain::UserRole::EventManager,
                expiry_timestamp,
            }) => {
                if expiry_timestamp < ::axiom::time::Timestamp::now() {
                    return ::axiom::err!(RemoveEventPostComment @ AuthenticationTokenExpired);
                }

                match ::std::sync::Arc::clone(&self.user_repository).get_by_id(user_id).await? {
                    ::core::option::Option::Some(::domain::User { ref statuses, .. }) => {
                        if ::core::matches!(statuses.last(), ::domain::UserStatus::Suspended { .. }) {
                            return ::axiom::err!(RemoveEventPostComment @ UserSuspended);
                        }
                    },
                    ::core::option::Option::None => return ::axiom::err!(RemoveEventPostComment @ UserNotFound),
                }

                user_id
            },
            ::core::option::Option::Some(AuthenticationTokenPayload { user_role, .. }) =>
                return ::axiom::err!(RemoveEventPostComment @ UserUnauthorized { user_role: user_role.into(), allowed_user_roles: ::std::vec![RemoveEventPostCommentUserRole::Volunteer, RemoveEventPostCommentUserRole::EventManager] }),
        };

        let comment_id = ::std::sync::Arc::clone(&self.uuid_codec).parse(request.comment_id).await?;

        let ::core::option::Option::Some(comment) =
            ::std::sync::Arc::clone(&self.comment_repository).get_by_id(comment_id).await?
        else {
            return ::axiom::err!(RemoveEventPostComment @ CommentNotFound);
        };

        if comment.author_id != actor_id {
            return ::axiom::err!(RemoveEventPostComment @ OwnershipMismatch);
        }

        ::std::sync::Arc::clone(&self.comment_repository).remove(comment_id).await?;

        let ::domain::EventPost { event_id, .. } = unsafe {
            ::std::sync::Arc::clone(&self.post_repository)
                .get_by_id(comment.post_id)
                .await?
                .unwrap_unchecked()
        };

        ::std::sync::Arc::clone(&self.event_recommender)
            .untrack_reacted(event_id, actor_id)
            .await?;

        ::axiom::ok!(RemoveEventPostComment)
    }
}
