use ::async_trait::async_trait;

use crate::_boundaries::*;
use crate::gateways::*;

#[derive(::bon::Builder)]
pub struct SignInInteractor {
    user_repository: ::std::sync::Arc<dyn UserRepository + ::core::marker::Send + ::core::marker::Sync>,

    auth_token_generator:
        ::std::sync::Arc<dyn AuthenticationTokenGenerator + ::core::marker::Send + ::core::marker::Sync>,
    password_hasher: ::std::sync::Arc<dyn PasswordHasher + ::core::marker::Send + ::core::marker::Sync>,
}

#[async_trait]
impl SignInBoundary for SignInInteractor {
    async fn apply(
        self: ::std::sync::Arc<Self>, request: SignInRequest,
    ) -> ::aliases::result::Fallible<SignInResponse> {
        use ::aliases::time::TimestampExt as _;

        let mut errors = ::std::vec::Vec::new();

        let user = if let ::core::result::Result::Ok(username) =
            ::domain::Username::builder().value(request.username_or_email.clone()).build()
        {
            if let ::core::option::Option::Some(user) =
                ::std::sync::Arc::clone(&self.user_repository).get_by_username(username).await?
            {
                ::core::option::Option::Some(user)
            } else {
                errors.push(SignInErrResponse::UsernameNotFound { username: request.username_or_email.clone() });
                ::core::option::Option::None
            }
        } else if let ::core::result::Result::Ok(email) =
            ::domain::Email::builder().value(request.username_or_email.clone()).build()
        {
            if let ::core::option::Option::Some(user) =
                ::std::sync::Arc::clone(&self.user_repository).get_by_email(email).await?
            {
                ::core::option::Option::Some(user)
            } else {
                errors.push(SignInErrResponse::EmailNotFound { email: request.username_or_email.clone() });
                ::core::option::Option::None
            }
        } else {
            errors.push(SignInErrResponse::UsernameOrEmailInvalid(::core::default::Default::default()));
            ::core::option::Option::None
        };

        let password = if let ::core::result::Result::Ok(password) =
            ::domain::Password::builder().value(request.password).build()
        {
            ::core::option::Option::Some(password)
        } else {
            errors.push(SignInErrResponse::PasswordInvalid(::core::default::Default::default()));
            ::core::option::Option::None
        };

        let (::core::option::Option::Some(user), ::core::option::Option::Some(password)) = (user, password) else {
            return ::aliases::result::Fallible::Ok(SignInResponse::Err(errors));
        };

        if !::std::sync::Arc::clone(&self.password_hasher)
            .verify(password, user.password)
            .await?
        {
            errors.push(SignInErrResponse::PasswordMismatch);
            return ::aliases::result::Fallible::Ok(SignInResponse::Err(errors));
        }

        let auth_token_payload = crate::gateways::models::AuthenticationTokenPayload::builder()
            .user_id(user.id)
            .user_role(user.role)
            .expiry_timestamp(::aliases::time::Timestamp::now() + Self::AUTH_TOKEN_LIFETIME)
            .build();

        let auth_token = ::std::sync::Arc::clone(&self.auth_token_generator)
            .generate(auth_token_payload)
            .await?;

        let response = SignInOkResponse::builder().token(auth_token).user_role(user.role).build();

        ::aliases::result::Fallible::Ok(SignInResponse::Ok(response))
    }
}

impl SignInInteractor {
    const AUTH_TOKEN_LIFETIME: ::aliases::time::Interval = ::aliases::time::Interval::hours(1);
}

#[derive(::bon::Builder)]
pub struct SignUpInteractor {
    user_repository: ::std::sync::Arc<dyn UserRepository + ::core::marker::Send + ::core::marker::Sync>,

    uuid_generator: ::std::sync::Arc<dyn UuidGenerator + ::core::marker::Send + ::core::marker::Sync>,
    password_hasher: ::std::sync::Arc<dyn PasswordHasher + ::core::marker::Send + ::core::marker::Sync>,
}

#[async_trait]
impl SignUpBoundary for SignUpInteractor {
    async fn apply(
        self: ::std::sync::Arc<Self>, request: SignUpRequest,
    ) -> ::aliases::result::Fallible<SignUpResponse> {
        let mut errors = ::std::vec::Vec::new();

        let username = if let ::core::result::Result::Ok(username) =
            ::domain::Username::builder().value(request.username).build()
        {
            ::core::option::Option::Some(username)
        } else {
            errors.push(SignUpErrResponse::UsernameInvalid(::core::default::Default::default()));
            ::core::option::Option::None
        };

        let email = if let ::core::result::Result::Ok(email) = ::domain::Email::builder().value(request.email).build() {
            ::core::option::Option::Some(email)
        } else {
            errors.push(SignUpErrResponse::EmailInvalid(::core::default::Default::default()));
            ::core::option::Option::None
        };

        let password = if let ::core::result::Result::Ok(password) =
            ::domain::Password::builder().value(request.password).build()
        {
            ::core::option::Option::Some(password)
        } else {
            errors.push(SignUpErrResponse::PasswordInvalid(::core::default::Default::default()));
            ::core::option::Option::None
        };

        let (
            ::core::option::Option::Some(username),
            ::core::option::Option::Some(email),
            ::core::option::Option::Some(password),
        ) = (username, email, password)
        else {
            return ::aliases::result::Fallible::Ok(SignUpResponse::Err(errors));
        };

        if ::std::sync::Arc::clone(&self.user_repository)
            .contains_username(username.clone())
            .await?
        {
            errors.push(SignUpErrResponse::UsernameAlreadyExists { username: username.to_string().into() });
        }

        if ::std::sync::Arc::clone(&self.user_repository)
            .contains_email(email.clone())
            .await?
        {
            errors.push(SignUpErrResponse::EmailAlreadyExists { email: email.to_string().into() });
        }

        if !errors.is_empty() {
            return ::aliases::result::Fallible::Ok(SignUpResponse::Err(errors));
        }

        let user_id = loop {
            let uuid = ::std::sync::Arc::clone(&self.uuid_generator).generate().await?;

            if !::std::sync::Arc::clone(&self.user_repository).contains_id(uuid).await? {
                break uuid;
            }
        };

        let password = ::std::sync::Arc::clone(&self.password_hasher).hash(password).await?;

        let user = ::domain::User::builder()
            .id(user_id)
            .role(request.user_role)
            .username(username)
            .email(email)
            .password(password)
            .first_name(request.first_name)
            .last_name(request.last_name)
            .build();

        ::std::sync::Arc::clone(&self.user_repository).save(user).await?;

        ::aliases::result::Fallible::Ok(SignUpResponse::Ok(()))
    }
}

#[derive(::bon::Builder)]
pub struct ViewEventRecommendationInteractor {
    event_repository: ::std::sync::Arc<dyn EventRepository + ::core::marker::Send + ::core::marker::Sync>,

    uuid_codec: ::std::sync::Arc<dyn UuidCodec + ::core::marker::Send + ::core::marker::Sync>,
    auth_token_generator: ::std::sync::Arc<dyn AuthenticationTokenGenerator + ::core::marker::Send + ::core::marker::Sync>,
}

#[async_trait]
impl ViewEventRecommendationBoundary for ViewEventRecommendationInteractor {
    async fn apply(self: ::std::sync::Arc<Self>, request: ViewEventRecommendationRequest)
    -> ::aliases::result::Fallible<ViewEventRecommendationResponse> {
        if !::std::sync::Arc::clone(&self.auth_token_generator).verify(request.token).await? {
            return ::aliases::result::Fallible::Ok(ViewEventRecommendationResponse::Err(::std::vec![
                ViewEventRecommendationErrResponse::AuthenticationTokenInvalid,
            ]));
        }

        // Rust's type inference fails here
        let events: ::std::vec::Vec<::domain::Event> = match request.r#type {
            crate::_boundaries::EventRecommendationType::RecentlyPublished =>
                ::std::sync::Arc::clone(&self.event_repository).view_recently_approved(request.limit).await?,
            crate::_boundaries::EventRecommendationType::RecentlyPosted =>
                ::std::sync::Arc::clone(&self.event_repository).view_recently_posted(request.limit).await?,
            crate::_boundaries::EventRecommendationType::Trending =>
                ::std::sync::Arc::clone(&self.event_repository).view_trending(request.limit).await?,
        };

        let events = ::futures::future::try_join_all(events.into_iter().map(|event| {
            let uuid_codec = ::std::sync::Arc::clone(&self.uuid_codec);
            async move {
                ::futures::future::ok::<_, ::aliases::result::Error>(crate::_boundaries::models::EventPreview::builder()
                    .id(uuid_codec.format(event.id).await?)
                    .statuses(event.statuses)
                    .name(event.name)
                    .categories(event.categories)
                    .build())
                .await
            }
        }))
            .await?;

        let response = ViewEventRecommendationOkResponse::builder().events(events).build();

        ::aliases::result::Fallible::Ok(ViewEventRecommendationResponse::Ok(response))
    }
}
