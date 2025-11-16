use ::async_trait::async_trait;

use crate::boundaries::*;
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
    ) -> ::axiom::result::Fallible<SignInResponse> {
        use ::axiom::time::TimestampExt as _;

        let mut errors = ::std::vec::Vec::new();

        // Rust's type inference fails here
        let user: ::core::option::Option<::domain::User> = if let ::core::result::Result::Ok(username) =
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
            return ::axiom::result::Fallible::Ok(SignInResponse::Err(errors));
        };

        if !::std::sync::Arc::clone(&self.password_hasher)
            .verify(password, user.password)
            .await?
        {
            errors.push(SignInErrResponse::PasswordMismatch);
            return ::axiom::result::Fallible::Ok(SignInResponse::Err(errors));
        }

        let auth_token_payload = crate::gateways::AuthenticationTokenPayload::builder()
            .user_id(user.id)
            .user_role(user.role)
            .expiry_timestamp(::axiom::time::Timestamp::now() + Self::AUTH_TOKEN_LIFETIME)
            .build();

        // Rust's type inference fails here
        let auth_token: ::axiom::string::String = ::std::sync::Arc::clone(&self.auth_token_generator)
            .generate(auth_token_payload)
            .await?;

        let response = SignInOkResponse::builder().token(auth_token).user_role(user.role).build();

        ::axiom::result::Fallible::Ok(SignInResponse::Ok(response))
    }
}

impl SignInInteractor {
    const AUTH_TOKEN_LIFETIME: ::axiom::time::Interval = ::axiom::time::Interval::hours(1);
}
