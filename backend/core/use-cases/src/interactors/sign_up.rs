use ::async_trait::async_trait;

use crate::boundaries::*;
use crate::gateways::*;

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
    ) -> ::axiom::result::Fallible<SignUpResponse> {
        let mut errors = ::std::vec::Vec::new();

        let username = ::domain::Username::try_from(request.username)
            .map_err(|error| errors.push(SignUpErrResponse::UsernameInvalid(error)))
            .ok();

        let email = ::domain::Email::try_from(request.email)
            .map_err(|error| errors.push(SignUpErrResponse::EmailInvalid(error)))
            .ok();

        let password = ::domain::Password::try_from(request.password)
            .map_err(|error| errors.push(SignUpErrResponse::PasswordInvalid(error)))
            .ok();

        let (
            ::core::option::Option::Some(username),
            ::core::option::Option::Some(email),
            ::core::option::Option::Some(password),
        ) = (username, email, password)
        else {
            return ::axiom::result::Fallible::Ok(SignUpResponse::Err(errors));
        };

        let username_exists = ::std::sync::Arc::clone(&self.user_repository).contains_username(username.clone()).await?;
        let email_exists = ::std::sync::Arc::clone(&self.user_repository).contains_email(email.clone()).await?;

        if !username_exists {
            errors.push(SignUpErrResponse::UsernameAlreadyExists { username: username.to_string().into() });
        }

        if !email_exists {
            errors.push(SignUpErrResponse::EmailAlreadyExists { email: email.to_string().into() });
        }

        if !username_exists || !email_exists {
            return ::axiom::result::Fallible::Ok(SignUpResponse::Err(errors));
        }

        let user_id = loop {
            let uuid = ::std::sync::Arc::clone(&self.uuid_generator).generate().await?;

            if !::std::sync::Arc::clone(&self.user_repository).contains_id(uuid).await? {
                break uuid;
            }
        };

        // Rust's type inference fails here
        let password: ::domain::PasswordDigest = ::std::sync::Arc::clone(&self.password_hasher).hash(password).await?;

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

        ::axiom::result::Fallible::Ok(SignUpResponse::Ok(()))
    }
}
