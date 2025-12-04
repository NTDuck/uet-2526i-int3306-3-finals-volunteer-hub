use ::axiom::prelude::*;

use crate::boundaries::*;
use crate::gateways::*;

#[derive(::bon::Builder)]
pub struct SignUpInteractor {
    user_repository: ::std::sync::Arc<dyn UserRepository + ::core::marker::Send + ::core::marker::Sync>,
    media_repository: ::std::sync::Arc<dyn MediaRepository + ::core::marker::Send + ::core::marker::Sync>,

    uuid_generator: ::std::sync::Arc<dyn UuidGenerator + ::core::marker::Send + ::core::marker::Sync>,
    password_hasher: ::std::sync::Arc<dyn PasswordHasher + ::core::marker::Send + ::core::marker::Sync>,
}

#[async_trait]
impl SignUpBoundary for SignUpInteractor {
    async fn apply(self: ::std::sync::Arc<Self>, request: Request) -> ::axiom::result::Fallible<Response> {
        let mut errors = ::std::vec::Vec::new();

        let username = ::domain::Username::try_from(request.username)
            .map_err(|error| errors.push(ErrResponse::UsernameInvalid { username: error.into() }));

        let email = ::domain::Email::try_from(request.email)
            .map_err(|error| errors.push(ErrResponse::EmailInvalid { email: error.into() }));

        let password =
            ::domain::Password::try_from(request.password).map_err(|_| errors.push(ErrResponse::PasswordInvalid));

        let full_name = ::domain::FullName::try_from(request.full_name)
            .map_err(|error| errors.push(ErrResponse::FullNameInvalid { full_name: error.into() }));

        let avatar_url = request.avatar
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
                    errors.push(ErrResponse::AvatarInvalid);
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

        let (
            ::core::result::Result::Ok(username),
            ::core::result::Result::Ok(email),
            ::core::result::Result::Ok(password),
            ::core::result::Result::Ok(full_name),
        ) = (username, email, password, full_name)
        else {
            return super::errs!(errors);
        };

        if ::std::sync::Arc::clone(&self.user_repository)
            .contains_username(username.clone())
            .await?
        {
            errors.push(ErrResponse::UsernameAlreadyExists { username: username.to_string().into() });
        }

        if ::std::sync::Arc::clone(&self.user_repository)
            .contains_email(email.clone())
            .await?
        {
            errors.push(ErrResponse::EmailAlreadyExists { email: email.to_string().into() });
        }

        if !errors.is_empty() {
            return super::errs!(errors);
        }

        let user_id = loop {
            let uuid = ::std::sync::Arc::clone(&self.uuid_generator).generate().await?;
            if !::std::sync::Arc::clone(&self.user_repository).contains_id(uuid).await? { break uuid; }
        };

        let password = ::std::sync::Arc::clone(&self.password_hasher).hash(password).await?;

        let user = ::domain::User::builder()
            .id(user_id)
            .statuses(::vec1::Vec1::new(::domain::UserStatus::Created { created_at: ::axiom::time::Timestamp::now() }))
            .role(request.user_role)
            .username(username)
            .email(email)
            .password(password)
            .full_name(full_name)
            .avatar_url(avatar_url)
            .build();

        ::std::sync::Arc::clone(&self.user_repository).save(user).await?;

        super::ok!(())
    }
}

type Request = SignUpRequest;
type Response = SignUpResponse;
type ErrResponse = SignUpErrResponse;
