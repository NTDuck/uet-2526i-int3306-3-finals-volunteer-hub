use ::axiom::prelude::*;

use crate::boundaries::*;
use crate::gateways::*;

#[derive(::bon::Builder)]
pub struct UpdateSelfProfileInteractor {
    user_repository: ::std::sync::Arc<dyn UserRepository + ::core::marker::Send + ::core::marker::Sync>,
    media_repository: ::std::sync::Arc<dyn MediaRepository + ::core::marker::Send + ::core::marker::Sync>,

    password_hasher: ::std::sync::Arc<dyn PasswordHasher + ::core::marker::Send + ::core::marker::Sync>,
    auth_token_generator: ::std::sync::Arc<dyn AuthTokenGenerator + ::core::marker::Send + ::core::marker::Sync>,
}

#[async_trait]
impl UpdateSelfProfileBoundary for UpdateSelfProfileInteractor {
    async fn apply(self: ::std::sync::Arc<Self>, request: Request) -> ::axiom::result::Fallible<Response> {
        let actor_id = match ::std::sync::Arc::clone(&self.auth_token_generator)
            .get_payload(request.token)
            .await?
        {
            ::core::option::Option::None => return super::err!(AuthenticationTokenInvalid),
            ::core::option::Option::Some(AuthTokenPayload { user_id, expiry_timestamp, .. }) => {
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
        };

        let mut errors = ::std::vec::Vec::new();

        let password = request
            .password
            .map(|password| {
                ::domain::Password::try_from(password).map_err(|_| errors.push(ErrResponse::PasswordInvalid))
            })
            .transpose();

        let new_password = request
            .new_password
            .map(|new_password| {
                ::domain::Password::try_from(new_password).map_err(|_| errors.push(ErrResponse::NewPasswordInvalid))
            })
            .transpose();

        let full_name = request
            .full_name
            .map(|full_name| {
                ::domain::FullName::try_from(full_name)
                    .map_err(|error| errors.push(ErrResponse::FullNameInvalid { full_name: error.into() }))
            })
            .transpose();

        let avatar_url = request
            .avatar
            .map(::core::convert::Into::<::axiom::bytes::Bytes>::into)
            .map_async(|image| {
                let media_repository = ::std::sync::Arc::clone(&self.media_repository);

                async move {
                    let verified = ::std::sync::Arc::clone(&media_repository).verify(image.clone()).await?;

                    (image, verified).into_ok()
                }
            })
            .await
            .transpose()?
            .map(|(image, verified)| {
                if !verified {
                    errors.push(ErrResponse::AvatarInvalid);
                }

                image
            })
            .map_async(|image| {
                let media_repository = ::std::sync::Arc::clone(&self.media_repository);

                async move { ::std::sync::Arc::clone(&media_repository).save(image).await?.into_ok() }
            })
            .await
            .transpose()?;

        let (
            ::core::result::Result::Ok(password),
            ::core::result::Result::Ok(new_password),
            ::core::result::Result::Ok(full_name),
        ) = (password, new_password, full_name)
        else {
            return super::errs!(errors);
        };

        if !errors.is_empty() {
            return super::errs!(errors);
        }

        let mut user = unsafe {
            ::std::sync::Arc::clone(&self.user_repository)
                .get_by_id(actor_id)
                .await?
                .unwrap_unchecked()
        };

        if let (::core::option::Option::Some(password), ::core::option::Option::Some(new_password)) =
            (password, new_password)
        {
            if !::std::sync::Arc::clone(&self.password_hasher)
                .verify(password.clone(), user.password)
                .await?
            {
                errors.push(ErrResponse::PasswordMismatch);
                return super::errs!(errors);
            }

            user.password = ::std::sync::Arc::clone(&self.password_hasher).hash(new_password).await?;
        }

        full_name.map(|full_name| user.full_name = full_name);
        avatar_url.map(|avatar_url| user.avatar_url = ::core::option::Option::Some(avatar_url));

        ::std::sync::Arc::clone(&self.user_repository).save(user).await?;

        super::ok!(())
    }
}

type Request = UpdateSelfProfileRequest;
type Response = UpdateSelfProfileResponse;
type ErrResponse = UpdateSelfProfileErrResponse;
