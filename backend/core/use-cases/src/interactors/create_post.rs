use ::axiom::prelude::*;

use crate::boundaries::*;
use crate::gateways::*;

#[derive(::bon::Builder)]
pub struct CreateEventPostInteractor {
    event_repository: ::std::sync::Arc<dyn EventRepository + ::core::marker::Send + ::core::marker::Sync>,
    post_repository: ::std::sync::Arc<dyn EventPostRepository + ::core::marker::Send + ::core::marker::Sync>,
    user_repository: ::std::sync::Arc<dyn UserRepository + ::core::marker::Send + ::core::marker::Sync>,

    uuid_codec: ::std::sync::Arc<dyn UuidCodec + ::core::marker::Send + ::core::marker::Sync>,
    uuid_generator: ::std::sync::Arc<dyn UuidGenerator + ::core::marker::Send + ::core::marker::Sync>,
    auth_token_generator:
        ::std::sync::Arc<dyn AuthenticationTokenGenerator + ::core::marker::Send + ::core::marker::Sync>,
}

#[async_trait]
impl CreateEventPostBoundary for CreateEventPostInteractor {
    async fn apply(
        self: ::std::sync::Arc<Self>, request: CreateEventPostRequest,
    ) -> ::axiom::result::Fallible<CreateEventPostResponse> {
        let actor_id = match ::std::sync::Arc::clone(&self.auth_token_generator).get_payload(request.token).await? {
            ::core::option::Option::None =>
                return ::axiom::err!(CreateEventPost @ AuthenticationTokenInvalid),
            ::core::option::Option::Some
            (AuthenticationTokenPayload { user_id, user_role: ::domain::UserRole::Volunteer | ::domain::UserRole::EventManager, expiry_timestamp }) => {
                if expiry_timestamp < ::axiom::time::Timestamp::now() {
                    return ::axiom::err!(CreateEventPost @ AuthenticationTokenExpired);
                }

                match ::std::sync::Arc::clone(&self.user_repository).get_by_id(user_id).await? {
                    ::core::option::Option::Some(user) => {
                        if ::core::matches!(user.statuses.last(), ::domain::UserStatus::Suspended { .. }) {
                            return ::axiom::err!(CreateEventPost @ UserSuspended);
                        }
                    },
                    ::core::option::Option::None =>
                        return ::axiom::err!(CreateEventPost @ UserNotFound),
                }

                user_id
            },
            ::core::option::Option::Some(AuthenticationTokenPayload { user_role, .. }) =>
                return ::axiom::err!(CreateEventPost @ UserUnauthorized { user_role: user_role.into() }),
        };

        let mut errors = ::std::vec::Vec::new();

        let post_title = ::domain::EventPostTitle::try_from(request.post_title)
            .map_err(|error| errors.push(CreateEventPostErrResponse::PostTitleInvalid { post_title: error.into() }));

        let post_content = ::domain::EventPostContent::try_from(request.post_content)
            .map_err(|error| errors.push(CreateEventPostErrResponse::PostContentInvalid { post_content: error.into() }));

        let event_id = ::std::sync::Arc::clone(&self.uuid_codec).parse(request.event_id).await?;

        match ::std::sync::Arc::clone(&self.event_repository).get_by_id(event_id).await? {
            ::core::option::Option::Some(event) => {
                if !::core::matches!(&event.statuses.last(), ::domain::EventStatus::Approved { .. }) {
                    errors.push(CreateEventPostErrResponse::EventChannelNotFound);
                }
            },
            ::core::option::Option::None => {
                errors.push(CreateEventPostErrResponse::EventChannelNotFound);
            },
        }

        let (
            ::core::result::Result::Ok(post_title),
            ::core::result::Result::Ok(post_content),
        ) = (post_title, post_content) else { return ::axiom::errs!(CreateEventPost @ errors) };

        if !errors.is_empty() {
            return ::axiom::errs!(CreateEventPost @ errors);
        }

        let post_id = loop {
            let uuid = ::std::sync::Arc::clone(&self.uuid_generator).generate().await?;

            if !::std::sync::Arc::clone(&self.user_repository).contains_id(uuid).await? {
                break uuid;
            }
        };

        let post = ::domain::EventPost::builder()
            .id(post_id)
            .event_id(event_id)
            .user_id(actor_id)
            .title(post_title)
            .content(post_content)
            .build();

        ::std::sync::Arc::clone(&self.post_repository).save(post).await?;

        ::axiom::ok!(CreateEventPost)
    }
}
