use ::axiom::prelude::*;

use crate::boundaries::*;
use crate::gateways::*;

#[derive(::bon::Builder)]
pub struct UpdateEventPostInteractor {
    post_repository: ::std::sync::Arc<dyn EventPostRepository + ::core::marker::Send + ::core::marker::Sync>,
    user_repository: ::std::sync::Arc<dyn UserRepository + ::core::marker::Send + ::core::marker::Sync>,

    uuid_codec: ::std::sync::Arc<dyn UuidCodec + ::core::marker::Send + ::core::marker::Sync>,
    auth_token_generator:
        ::std::sync::Arc<dyn AuthenticationTokenGenerator + ::core::marker::Send + ::core::marker::Sync>,
}

#[async_trait]
impl UpdateEventPostBoundary for UpdateEventPostInteractor {
    async fn apply(
        self: ::std::sync::Arc<Self>, request: UpdateEventPostRequest,
    ) -> ::axiom::result::Fallible<UpdateEventPostResponse> {
        let actor_id = match ::std::sync::Arc::clone(&self.auth_token_generator).get_payload(request.token).await? {
            ::core::option::Option::None =>
                return ::axiom::err!(UpdateEventPost @ AuthenticationTokenInvalid),
            ::core::option::Option::Some
            (AuthenticationTokenPayload { user_id, user_role: ::domain::UserRole::Volunteer | ::domain::UserRole::EventManager, expiry_timestamp }) => {
                if expiry_timestamp < ::axiom::time::Timestamp::now() {
                    return ::axiom::err!(UpdateEventPost @ AuthenticationTokenExpired);
                }

                match ::std::sync::Arc::clone(&self.user_repository).get_by_id(user_id).await? {
                    ::core::option::Option::Some(user) => {
                        if ::core::matches!(user.statuses.last(), ::domain::UserStatus::Suspended { .. }) {
                            return ::axiom::err!(UpdateEventPost @ UserSuspended);
                        }
                    },
                    ::core::option::Option::None =>
                        return ::axiom::err!(UpdateEventPost @ UserNotFound),
                }

                user_id
            },
            ::core::option::Option::Some(AuthenticationTokenPayload { user_role, .. }) =>
                return ::axiom::err!(UpdateEventPost @ UserUnauthorized { user_role: user_role.into(), allowed_user_roles: ::std::vec![UpdateEventPostUserRole::Volunteer, UpdateEventPostUserRole::EventManager] }),
        };

        let mut errors = ::std::vec::Vec::new();

        let post_id = ::std::sync::Arc::clone(&self.uuid_codec).parse(request.post_id).await?;

        let post = ::std::sync::Arc::clone(&self.post_repository).get_by_id(post_id).await?;

        match post {
            ::core::option::Option::Some(::domain::EventPost { author_id, .. }) => {
                if author_id != actor_id {
                    errors.push(UpdateEventPostErrResponse::OwnershipMismatch);
                }
            },
            ::core::option::Option::None => {
                errors.push(UpdateEventPostErrResponse::PostNotFound);
            },
        }

        let post_title = request.post_title.map(|post_title| ::domain::EventPostTitle::try_from(post_title)
            .map_err(|error| errors.push(UpdateEventPostErrResponse::PostTitleInvalid { post_title: error.into() })))
            .transpose();

        let post_content = request.post_content.map(|post_content| ::domain::EventPostContent::try_from(post_content)
            .map_err(|error| errors.push(UpdateEventPostErrResponse::PostContentInvalid { post_content: error.into() })))
            .transpose();

        let (
            ::core::result::Result::Ok(post_title),
            ::core::result::Result::Ok(post_content),
        ) = (post_title, post_content) else { return ::axiom::errs!(UpdateEventPost @ errors) };

        if !errors.is_empty() {
            return ::axiom::errs!(UpdateEventPost @ errors);
        }

        let mut post = unsafe { post.unwrap_unchecked() };

        post_title.map(|post_title| post.title = post_title);
        post_content.map(|post_content| post.content = post_content);

        ::std::sync::Arc::clone(&self.post_repository).save(post).await?;

        ::axiom::ok!(UpdateEventPost)
    }
}
