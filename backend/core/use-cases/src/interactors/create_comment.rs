use ::axiom::prelude::*;

use crate::boundaries::*;
use crate::gateways::*;

#[derive(::bon::Builder)]
pub struct CreateEventPostCommentInteractor {
    post_repository: ::std::sync::Arc<dyn EventPostRepository + ::core::marker::Send + ::core::marker::Sync>,
    comment_repository: ::std::sync::Arc<dyn EventPostCommentRepository + ::core::marker::Send + ::core::marker::Sync>,
    user_repository: ::std::sync::Arc<dyn UserRepository + ::core::marker::Send + ::core::marker::Sync>,

    uuid_codec: ::std::sync::Arc<dyn UuidCodec + ::core::marker::Send + ::core::marker::Sync>,
    uuid_generator: ::std::sync::Arc<dyn UuidGenerator + ::core::marker::Send + ::core::marker::Sync>,
    auth_token_generator:
        ::std::sync::Arc<dyn AuthenticationTokenGenerator + ::core::marker::Send + ::core::marker::Sync>,
}

#[async_trait]
impl CreateEventPostCommentBoundary for CreateEventPostCommentInteractor {
    async fn apply(
        self: ::std::sync::Arc<Self>, request: CreateEventPostCommentRequest,
    ) -> ::axiom::result::Fallible<CreateEventPostCommentResponse> {
        let actor_id = match ::std::sync::Arc::clone(&self.auth_token_generator).get_payload(request.token).await? {
            ::core::option::Option::None =>
                return ::axiom::err!(CreateEventPostComment @ AuthenticationTokenInvalid),
            ::core::option::Option::Some
            (AuthenticationTokenPayload { user_id, user_role: ::domain::UserRole::Volunteer | ::domain::UserRole::EventManager, expiry_timestamp }) => {
                if expiry_timestamp < ::axiom::time::Timestamp::now() {
                    return ::axiom::err!(CreateEventPostComment @ AuthenticationTokenExpired);
                }

                match ::std::sync::Arc::clone(&self.user_repository).get_by_id(user_id).await? {
                    ::core::option::Option::Some(user) => {
                        if ::core::matches!(user.statuses.last(), ::domain::UserStatus::Suspended { .. }) {
                            return ::axiom::err!(CreateEventPostComment @ UserSuspended);
                        }
                    },
                    ::core::option::Option::None =>
                        return ::axiom::err!(CreateEventPostComment @ UserNotFound),
                }

                user_id
            },
            ::core::option::Option::Some(AuthenticationTokenPayload { user_role, .. }) =>
                return ::axiom::err!(CreateEventPostComment @ UserUnauthorized { user_role: user_role.into() }),
        };

        let mut errors = ::std::vec::Vec::new();

        let comment_content = ::domain::EventPostCommentContent::try_from(request.comment_content)
            .map_err(|error| errors.push(CreateEventPostCommentErrResponse::CommentContentInvalid { comment_content: error.into() }));

        let post_id = ::std::sync::Arc::clone(&self.uuid_codec).parse(request.post_id).await?;

        if !::std::sync::Arc::clone(&self.post_repository).contains_id(post_id).await? {
            errors.push(CreateEventPostCommentErrResponse::PostNotFound);
        }

        let ::core::result::Result::Ok(comment_content) = comment_content else { return ::axiom::errs!(CreateEventPostComment @ errors) };

        if !errors.is_empty() {
            return ::axiom::errs!(CreateEventPostComment @ errors);
        }

        let comment_id = loop {
            let uuid = ::std::sync::Arc::clone(&self.uuid_generator).generate().await?;

            if !::std::sync::Arc::clone(&self.user_repository).contains_id(uuid).await? {
                break uuid;
            }
        };

        let comment = ::domain::EventPostComment::builder()
            .id(comment_id)
            .post_id(post_id)
            .user_id(actor_id)
            .content(comment_content)
            .build();

        ::std::sync::Arc::clone(&self.comment_repository).save(comment).await?;

        ::axiom::ok!(CreateEventPostComment)
    }
}
