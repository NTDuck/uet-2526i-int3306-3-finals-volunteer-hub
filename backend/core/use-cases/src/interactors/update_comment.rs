use ::axiom::prelude::*;

use crate::boundaries::*;
use crate::gateways::*;

#[derive(::bon::Builder)]
pub struct UpdateEventPostCommentInteractor {
    comment_repository: ::std::sync::Arc<dyn EventPostCommentRepository + ::core::marker::Send + ::core::marker::Sync>,
    user_repository: ::std::sync::Arc<dyn UserRepository + ::core::marker::Send + ::core::marker::Sync>,

    uuid_codec: ::std::sync::Arc<dyn UuidCodec + ::core::marker::Send + ::core::marker::Sync>,
    auth_token_generator:
        ::std::sync::Arc<dyn AuthenticationTokenGenerator + ::core::marker::Send + ::core::marker::Sync>,
}

#[async_trait]
impl UpdateEventPostCommentBoundary for UpdateEventPostCommentInteractor {
    async fn apply(
        self: ::std::sync::Arc<Self>, request: UpdateEventPostCommentRequest,
    ) -> ::axiom::result::Fallible<UpdateEventPostCommentResponse> {
        let actor_id = match ::std::sync::Arc::clone(&self.auth_token_generator).get_payload(request.token).await? {
            ::core::option::Option::None =>
                return ::axiom::err!(UpdateEventPostComment @ AuthenticationTokenInvalid),
            ::core::option::Option::Some
            (AuthenticationTokenPayload { user_id, user_role: ::domain::UserRole::Volunteer | ::domain::UserRole::EventManager, expiry_timestamp }) => {
                if expiry_timestamp < ::axiom::time::Timestamp::now() {
                    return ::axiom::err!(UpdateEventPostComment @ AuthenticationTokenExpired);
                }

                match ::std::sync::Arc::clone(&self.user_repository).get_by_id(user_id).await? {
                    ::core::option::Option::Some(user) => {
                        if ::core::matches!(user.statuses.last(), ::domain::UserStatus::Suspended { .. }) {
                            return ::axiom::err!(UpdateEventPostComment @ UserSuspended);
                        }
                    },
                    ::core::option::Option::None =>
                        return ::axiom::err!(UpdateEventPostComment @ UserNotFound),
                }

                user_id
            },
            ::core::option::Option::Some(AuthenticationTokenPayload { user_role, .. }) =>
                return ::axiom::err!(UpdateEventPostComment @ UserUnauthorized { user_role: user_role.into(), allowed_user_roles: ::std::vec![UpdateEventPostCommentUserRole::Volunteer, UpdateEventPostCommentUserRole::EventManager] }),
        };

        let mut errors = ::std::vec::Vec::new();
    
        let comment_id = ::std::sync::Arc::clone(&self.uuid_codec).parse(request.comment_id).await?;

        let comment = ::std::sync::Arc::clone(&self.comment_repository).get_by_id(comment_id).await?;

        match comment {
            ::core::option::Option::Some(::domain::EventPostComment { author_id, .. }) => {
                if author_id != actor_id {
                    errors.push(UpdateEventPostCommentErrResponse::OwnershipMismatch);
                }
            },
            ::core::option::Option::None => {
                errors.push(UpdateEventPostCommentErrResponse::CommentNotFound);
            },
        }

        let comment_content = request.comment_content.map(|
        comment_content| ::domain::EventPostCommentContent::try_from(comment_content)
            .map_err(|error| errors.push(UpdateEventPostCommentErrResponse::CommentContentInvalid { comment_content: error.into() })))
            .transpose();

        let ::core::result::Result::Ok(comment_content) = comment_content else { return ::axiom::errs!(UpdateEventPostComment @ errors) };

        if !errors.is_empty() {
            return ::axiom::errs!(UpdateEventPostComment @ errors);
        }

        let mut comment = unsafe { comment.unwrap_unchecked() };

        comment_content.map(|comment_content| comment.content = comment_content);

        ::std::sync::Arc::clone(&self.comment_repository).save(comment).await?;

        ::axiom::ok!(UpdateEventPostComment)
    }
}
