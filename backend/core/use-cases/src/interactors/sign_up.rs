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
    async fn apply(self: ::std::sync::Arc<Self>, request: SignUpRequest) -> ::axiom::result::Fallible<SignUpResponse> {
        let mut errors = ::std::vec::Vec::new();

        let username = ::domain::Username::try_from(request.username)
            .map_err(|error| errors.push(SignUpErrResponse::UsernameInvalid { username: error.into() }))
            .ok();

        let email = ::domain::Email::try_from(request.email)
            .map_err(|error| errors.push(SignUpErrResponse::EmailInvalid { email: error.into() }))
            .ok();

        let password = ::domain::Password::try_from(request.password)
            .map_err(|_| errors.push(SignUpErrResponse::PasswordInvalid))
            .ok();

        let full_name = ::domain::FullName::try_from(request.full_name)
            .map_err(|error| errors.push(SignUpErrResponse::FullNameInvalid { full_name: error.into() }))
            .ok();

        let (
            ::core::option::Option::Some(username),
            ::core::option::Option::Some(email),
            ::core::option::Option::Some(password),
            ::core::option::Option::Some(full_name),
        ) = (username, email, password, full_name)
        else {
            return ::axiom::errs!(SignUp @ errors);
        };

        let username_exists = ::std::sync::Arc::clone(&self.user_repository)
            .contains_username(username.clone())
            .await?;
        let email_exists = ::std::sync::Arc::clone(&self.user_repository)
            .contains_email(email.clone())
            .await?;

        if username_exists {
            errors.push(SignUpErrResponse::UsernameAlreadyExists { username: username.to_string().into() });
        }

        if email_exists {
            errors.push(SignUpErrResponse::EmailAlreadyExists { email: email.to_string().into() });
        }

        if username_exists || email_exists {
            return ::axiom::errs!(SignUp @ errors);
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
            .statuses(::vec1::Vec1::new(::domain::UserStatus::Created))
            .role(request.user_role)
            .username(username)
            .email(email)
            .password(password)
            .full_name(full_name)
            .build();

        ::std::sync::Arc::clone(&self.user_repository).save(user).await?;

        ::axiom::ok!(SignUp)
    }
}
