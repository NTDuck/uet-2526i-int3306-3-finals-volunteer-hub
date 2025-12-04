use ::axiom::prelude::*;

use crate::boundaries::*;
use crate::gateways::*;

#[derive(::bon::Builder)]
pub struct UpdateEventPostCommentInteractor {
    comment_repository: ::std::sync::Arc<dyn EventPostCommentRepository + ::core::marker::Send + ::core::marker::Sync>,
    user_repository: ::std::sync::Arc<dyn UserRepository + ::core::marker::Send + ::core::marker::Sync>,
    media_repository: ::std::sync::Arc<dyn MediaRepository + ::core::marker::Send + ::core::marker::Sync>,

    uuid_codec: ::std::sync::Arc<dyn UuidCodec + ::core::marker::Send + ::core::marker::Sync>,
    auth_token_generator:
        ::std::sync::Arc<dyn AuthTokenGenerator + ::core::marker::Send + ::core::marker::Sync>,
}

#[async_trait]
impl UpdateEventPostCommentBoundary for UpdateEventPostCommentInteractor {
    async fn apply(self: ::std::sync::Arc<Self>, request: Request) -> ::axiom::result::Fallible<Response> {
        let actor_id = match ::std::sync::Arc::clone(&self.auth_token_generator)
            .get_payload(request.token)
            .await?
        {
            ::core::option::Option::None => return super::err!(AuthenticationTokenInvalid),
            ::core::option::Option::Some(AuthenticationTokenPayload {
                user_id,
                user_role: ::domain::UserRole::Volunteer | ::domain::UserRole::EventManager,
                expiry_timestamp,
            }) => {
                if expiry_timestamp < ::axiom::time::Timestamp::now() {
                    return super::err!(AuthenticationTokenExpired);
                }

                match ::std::sync::Arc::clone(&self.user_repository).get_by_id(user_id).await? {
                    ::core::option::Option::Some(::domain::User { statuses, .. }) => {
                        if ::core::matches!(statuses[..], [.., ::domain::UserStatus::Suspended { .. }]) {
                            return super::err!(UserSuspended);
                        }
                    },
                    ::core::option::Option::None => return super::err!(UserNotFound),
                }

                user_id
            },
            ::core::option::Option::Some(AuthenticationTokenPayload { user_role, .. }) =>
                return super::err!(UserUnauthorized { user_role: user_role.into(), allowed_user_roles: ::std::vec![UserRole::Volunteer, UserRole::EventManager] }),
        };

        let mut errors = ::std::vec::Vec::new();

        let comment_id = ::std::sync::Arc::clone(&self.uuid_codec).parse(request.comment_id).await?;

        let comment = ::std::sync::Arc::clone(&self.comment_repository).get_by_id(comment_id).await?;

        match comment {
            ::core::option::Option::Some(::domain::EventPostComment { author_id, .. }) =>
                if author_id != actor_id {
                    errors.push(ErrResponse::OwnershipMismatch);
                },
            ::core::option::Option::None => {
                errors.push(ErrResponse::CommentNotFound);
            },
        }

        let comment_content = request
            .comment_content
            .map(|comment_content| {
                ::domain::EventPostCommentContent::try_from(comment_content).map_err(|error| {
                    errors.push(ErrResponse::CommentContentInvalid {
                        comment_content: error.into(),
                    })
                })
            })
            .transpose();

        let comment_image_url = request.comment_image
            .map(::core::convert::Into::<::axiom::bytes::Bytes>::into)
            .map_async(|image| {
                let media_repository = ::std::sync::Arc::clone(&self.media_repository);

                async move {
                    let verified = ::std::sync::Arc::clone(&media_repository).verify(image.clone()).await?;

                    (image, verified).into_ok()
                }
            }).await
            .transpose()?
            .map(|(image, verified)| {
                if !verified {
                    errors.push(ErrResponse::CommentImageInvalid);
                }

                image
            })
            .map_async(|image| {
                let media_repository = ::std::sync::Arc::clone(&self.media_repository);

                async move {
                    ::std::sync::Arc::clone(&media_repository)
                        .save(image)
                        .await?
                        .into_ok()
                }
            }).await
            .transpose()?;

        let ::core::result::Result::Ok(comment_content) = comment_content else {
            return super::errs!(errors);
        };

        if !errors.is_empty() {
            return super::errs!(errors);
        }

        let mut comment = unsafe { comment.unwrap_unchecked() };

        comment_content.map(::core::convert::Into::into).map(|comment_content| comment.content = comment_content);

        comment_image_url.map(::core::convert::Into::into).map(|comment_image_url| comment.image_url = comment_image_url);

        ::std::sync::Arc::clone(&self.comment_repository).save(comment).await?;

        super::ok!(())
    }
}

type Request = UpdateEventPostCommentRequest;
type Response = UpdateEventPostCommentResponse;
type ErrResponse = UpdateEventPostCommentErrResponse;
type UserRole = UpdateEventPostCommentUserRole;
