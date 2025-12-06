use ::axiom::prelude::*;

use crate::boundaries::*;
use crate::gateways::*;

#[derive(::bon::Builder)]
pub struct CreateEventPostCommentInteractor {
    event_recommender: ::std::sync::Arc<dyn EventRecommender + ::core::marker::Send + ::core::marker::Sync>,
    post_repository: ::std::sync::Arc<dyn EventPostRepository + ::core::marker::Send + ::core::marker::Sync>,
    comment_repository: ::std::sync::Arc<dyn EventPostCommentRepository + ::core::marker::Send + ::core::marker::Sync>,
    user_repository: ::std::sync::Arc<dyn UserRepository + ::core::marker::Send + ::core::marker::Sync>,
    media_repository: ::std::sync::Arc<dyn MediaRepository + ::core::marker::Send + ::core::marker::Sync>,

    uuid_generator: ::std::sync::Arc<dyn UuidGenerator + ::core::marker::Send + ::core::marker::Sync>,
    uuid_codec: ::std::sync::Arc<dyn UuidCodec + ::core::marker::Send + ::core::marker::Sync>,
    auth_token_generator:
        ::std::sync::Arc<dyn AuthTokenGenerator + ::core::marker::Send + ::core::marker::Sync>,
}

#[async_trait]
impl CreateEventPostCommentBoundary for CreateEventPostCommentInteractor {
    async fn apply(
        self: ::std::sync::Arc<Self>, request: Request,
    ) -> ::axiom::result::Fallible<Response> {
        let actor_id = match ::std::sync::Arc::clone(&self.auth_token_generator)
            .get_payload(request.token)
            .await?
        {
            ::core::option::Option::None => return super::err!( AuthenticationTokenInvalid),
            ::core::option::Option::Some(AuthTokenPayload {
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
            ::core::option::Option::Some(AuthTokenPayload { user_role, .. }) =>
                return super::err!(UserUnauthorized { user_role: user_role.into(), allowed_user_roles: ::std::vec![UserRole::Volunteer, UserRole::EventManager, ], }),
        };

        let mut errors = ::std::vec::Vec::new();

        if ::core::matches!(request, Request { comment_content: ::core::option::Option::None, comment_image: ::core::option::Option::None, .. }) {
            errors.push(ErrResponse::MissingRequiredFields);
        }

        let comment_content = request.comment_content
            .map(|comment_content| ::domain::EventPostCommentContent::try_from(comment_content)
                .map_err(|error| {
                errors.push(ErrResponse::CommentContentInvalid { comment_content: error.into() })
            }))
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

        let post_id = ::std::sync::Arc::clone(&self.uuid_codec).parse(request.post_id).await?;
        let post = ::std::sync::Arc::clone(&self.post_repository).get_by_id(post_id).await?;

        if ::core::matches!(&post, ::core::option::Option::None) {
            errors.push(ErrResponse::PostNotFound);
        }

        let ::core::result::Result::Ok(comment_content) = comment_content else {
            return super::errs!(errors);
        };

        if !errors.is_empty() {
            return super::errs!(errors);
        }

        let comment_id = loop {
            let uuid = ::std::sync::Arc::clone(&self.uuid_generator).generate().await?;
            if !::std::sync::Arc::clone(&self.comment_repository).contains_id(uuid).await? { break uuid; }
        };

        let comment = ::domain::EventPostComment::builder()
            .id(comment_id)
            .post_id(post_id)
            .author_id(actor_id)
            .last_updated_at(::axiom::time::Timestamp::now())
            .content(comment_content)
            .image_url(comment_image_url)
            .build();

        ::std::sync::Arc::clone(&self.comment_repository).save(comment).await?;

        ::std::sync::Arc::clone(&self.event_recommender)
            .track_commented(unsafe { post.unwrap_unchecked() }.event_id, actor_id)
            .await?;

        super::ok!(())
    }
}

type Request = CreateEventPostCommentRequest;
type Response = CreateEventPostCommentResponse;
type ErrResponse = CreateEventPostCommentErrResponse;
type UserRole = CreateEventPostCommentUserRole;