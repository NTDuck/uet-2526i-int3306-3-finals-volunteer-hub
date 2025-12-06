use ::axiom::prelude::*;

use crate::boundaries::*;
use crate::gateways::*;

#[derive(::bon::Builder)]
pub struct CreateEventPostInteractor {
    event_repository: ::std::sync::Arc<dyn EventRepository + ::core::marker::Send + ::core::marker::Sync>,
    event_recommender: ::std::sync::Arc<dyn EventRecommender + ::core::marker::Send + ::core::marker::Sync>,
    post_repository: ::std::sync::Arc<dyn EventPostRepository + ::core::marker::Send + ::core::marker::Sync>,
    user_repository: ::std::sync::Arc<dyn UserRepository + ::core::marker::Send + ::core::marker::Sync>,
    media_repository: ::std::sync::Arc<dyn MediaRepository + ::core::marker::Send + ::core::marker::Sync>,

    uuid_codec: ::std::sync::Arc<dyn UuidCodec + ::core::marker::Send + ::core::marker::Sync>,
    uuid_generator: ::std::sync::Arc<dyn UuidGenerator + ::core::marker::Send + ::core::marker::Sync>,
    auth_token_generator:
        ::std::sync::Arc<dyn AuthTokenGenerator + ::core::marker::Send + ::core::marker::Sync>,
}

#[async_trait]
impl CreateEventPostBoundary for CreateEventPostInteractor {
    async fn apply(
        self: ::std::sync::Arc<Self>, request: Request,
    ) -> ::axiom::result::Fallible<Response> {
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
                return super::err!(UserUnauthorized { user_role: user_role.into(), allowed_user_roles: ::std::vec![UserRole::Volunteer, UserRole::EventManager] }),
        };

        let mut errors = ::std::vec::Vec::new();

        let post_title = ::domain::EventPostTitle::try_from(request.post_title)
            .map_err(|error| errors.push(ErrResponse::PostTitleInvalid { post_title: error.into() }));

        let post_content = ::domain::EventPostContent::try_from(request.post_content).map_err(|error| {
            errors.push(ErrResponse::PostContentInvalid { post_content: error.into() })
        });

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

        let event_id = ::std::sync::Arc::clone(&self.uuid_codec).parse(request.event_id).await?;

        match ::std::sync::Arc::clone(&self.event_repository).get_by_id(event_id).await? {
            ::core::option::Option::Some(::domain::Event { statuses, .. }) => {
                if !::core::matches!(*statuses, [.., ::domain::EventStatus::Approved { .. }]) {
                    errors.push(ErrResponse::EventChannelNotFound);
                }
            },
            ::core::option::Option::None => {
                errors.push(ErrResponse::EventChannelNotFound);
            },
        }

        let (::core::result::Result::Ok(post_title), ::core::result::Result::Ok(post_content)) =
            (post_title, post_content)
        else {
            return super::errs!(errors);
        };

        if !errors.is_empty() {
            return super::errs!(errors);
        }

        let post_id = loop {
            let uuid = ::std::sync::Arc::clone(&self.uuid_generator).generate().await?;
            if !::std::sync::Arc::clone(&self.user_repository).contains_id(uuid).await? { break uuid; }
        };

        let post = ::domain::EventPost::builder()
            .id(post_id)
            .event_id(event_id)
            .author_id(actor_id)
            .last_updated_at(::axiom::time::Timestamp::now())
            .title(post_title)
            .content(post_content)
            .image_url(post_image_url)
            .build();

        ::std::sync::Arc::clone(&self.post_repository).save(post).await?;

        ::std::sync::Arc::clone(&self.event_recommender)
            .track_posted(event_id, actor_id)
            .await?;

        super::ok!(())
    }
}

type Request = CreateEventPostRequest;
type Response = CreateEventPostResponse;
type ErrResponse = CreateEventPostErrResponse;
type UserRole = CreateEventPostUserRole;
