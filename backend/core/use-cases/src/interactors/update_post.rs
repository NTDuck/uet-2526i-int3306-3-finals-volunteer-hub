use ::axiom::prelude::*;

use crate::boundaries::*;
use crate::gateways::*;

#[derive(::bon::Builder)]
pub struct UpdateEventPostInteractor {
    post_repository: ::std::sync::Arc<dyn EventPostRepository + ::core::marker::Send + ::core::marker::Sync>,
    user_repository: ::std::sync::Arc<dyn UserRepository + ::core::marker::Send + ::core::marker::Sync>,
    media_repository: ::std::sync::Arc<dyn MediaRepository + ::core::marker::Send + ::core::marker::Sync>,

    uuid_codec: ::std::sync::Arc<dyn UuidCodec + ::core::marker::Send + ::core::marker::Sync>,
    auth_token_generator:
        ::std::sync::Arc<dyn AuthTokenGenerator + ::core::marker::Send + ::core::marker::Sync>,
}

#[async_trait]
impl UpdateEventPostBoundary for UpdateEventPostInteractor {
    async fn apply(
        self: ::std::sync::Arc<Self>, request: Request,
    ) -> ::axiom::result::Fallible<Response> {
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

        let post_id = ::std::sync::Arc::clone(&self.uuid_codec).parse(request.post_id).await?;

        let post = ::std::sync::Arc::clone(&self.post_repository).get_by_id(post_id).await?;

        match post {
            ::core::option::Option::Some(::domain::EventPost { author_id, .. }) =>
                if author_id != actor_id {
                    errors.push(ErrResponse::OwnershipMismatch);
                },
            ::core::option::Option::None => {
                errors.push(ErrResponse::PostNotFound);
            },
        }

        let post_title = request
            .post_title
            .map(|post_title| {
                ::domain::EventPostTitle::try_from(post_title).map_err(|error| {
                    errors.push(ErrResponse::PostTitleInvalid { post_title: error.into() })
                })
            })
            .transpose();

        let post_content = request
            .post_content
            .map(|post_content| {
                ::domain::EventPostContent::try_from(post_content).map_err(|error| {
                    errors.push(ErrResponse::PostContentInvalid { post_content: error.into() })
                })
            })
            .transpose();

        let post_image_url = request.post_image
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
                    errors.push(ErrResponse::PostImageInvalid);
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

        let (::core::result::Result::Ok(post_title), ::core::result::Result::Ok(post_content)) =
            (post_title, post_content)
        else {
            return super::errs!(errors);
        };

        if !errors.is_empty() {
            return super::errs!(errors);
        }

        let mut post = unsafe { post.unwrap_unchecked() };

        post_title.map(|post_title| post.title = post_title);
        post_content.map(|post_content| post.content = post_content);
        post_image_url.map(::core::convert::Into::into).map(|post_image_url| post.image_url = post_image_url);

        ::std::sync::Arc::clone(&self.post_repository).save(post).await?;

        super::ok!(())
    }
}

type Request = UpdateEventPostRequest;
type Response = UpdateEventPostResponse;
type ErrResponse = UpdateEventPostErrResponse;
type UserRole = UpdateEventPostUserRole;
