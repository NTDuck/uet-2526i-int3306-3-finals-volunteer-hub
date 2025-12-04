use ::axiom::prelude::*;

use crate::boundaries::*;
use crate::gateways::*;

#[derive(::bon::Builder)]
pub struct SignInInteractor {
    #[builder(default = ::axiom::time::Interval::hours(1))]
    auth_token_lifetime: ::axiom::time::Interval,

    user_repository: ::std::sync::Arc<dyn UserRepository + ::core::marker::Send + ::core::marker::Sync>,

    auth_token_generator:
        ::std::sync::Arc<dyn AuthTokenGenerator + ::core::marker::Send + ::core::marker::Sync>,
    password_hasher: ::std::sync::Arc<dyn PasswordHasher + ::core::marker::Send + ::core::marker::Sync>,
}

#[async_trait]
impl SignInBoundary for SignInInteractor {
    async fn apply(self: ::std::sync::Arc<Self>, request: Request) -> ::axiom::result::Fallible<Response> {
        let mut errors = ::std::vec::Vec::new();

        let user = if let ::core::result::Result::Ok(username) =
            ::domain::Username::try_from(request.username_or_email.clone())
        {
            let user = ::std::sync::Arc::clone(&self.user_repository)
                .get_by_username(username)
                .await?;

            if ::core::matches!(user, ::core::option::Option::None) {
                errors.push(ErrResponse::UsernameNotFound {
                    username: request.username_or_email.clone(),
                });
            }

            user

        } else if let ::core::result::Result::Ok(email) = ::domain::Email::try_from(request.username_or_email.clone()) {
            let user = ::std::sync::Arc::clone(&self.user_repository)
                .get_by_email(email)
                .await?;

            if ::core::matches!(user, ::core::option::Option::None) {
                errors.push(ErrResponse::EmailNotFound {
                    email: request.username_or_email,
                });
            }
            
            user

        } else {
            errors.push(ErrResponse::UsernameOrEmailInvalid {
                username_or_email: request.username_or_email.clone(),
            });

            ::core::option::Option::None
        };

        let password = ::domain::Password::try_from(request.password.clone())
            .map_err(|_| errors.push(ErrResponse::PasswordInvalid));

        let (::core::option::Option::Some(user), ::core::result::Result::Ok(password)) = (user, password) else {
            return super::errs!(errors);
        };

        if !::std::sync::Arc::clone(&self.password_hasher)
            .verify(password, user.password)
            .await?
        {
            errors.push(ErrResponse::PasswordMismatch);
            return super::errs!(errors);
        }

        let auth_token_payload = AuthenticationTokenPayload::builder()
            .user_id(user.id)
            .user_role(user.role)
            .expiry_timestamp(::axiom::time::Timestamp::now() + self.auth_token_lifetime)
            .build();

        let auth_token = ::std::sync::Arc::clone(&self.auth_token_generator)
            .generate(auth_token_payload)
            .await?;

        let response = OkResponse::builder().token(auth_token).user_role(user.role).build();
        super::ok!(response)
    }
}

type Request = SignInRequest;
type Response = SignInResponse;
type OkResponse = SignInOkResponse;
type ErrResponse = SignInErrResponse;
