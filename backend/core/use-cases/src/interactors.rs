use ::async_trait::async_trait;
use crate::gateways::*;
use crate::boundaries::*;

#[derive(::bon::Builder)]
pub struct SignInInteractor {
	user_repository: ::std::sync::Arc<dyn UserRepository + ::core::marker::Send + ::core::marker::Sync>,

	auth_token_generator: ::std::sync::Arc<dyn AuthenticationTokenGenerator + ::core::marker::Send + ::core::marker::Sync>,
	password_hasher: ::std::sync::Arc<dyn PasswordHasher + ::core::marker::Send + ::core::marker::Sync>,
}

#[async_trait]
impl SignInBoundary for SignInInteractor {
    async fn apply(self: ::std::sync::Arc<Self>, request: SignInRequest) -> ::aliases::result::Fallible<SignInResponse> {
		use ::aliases::time::TimestampExt as _;

		let mut errors = ::std::vec::Vec::new();
		
    	let user = if let ::core::result::Result::Ok(username) = ::domain::Username::builder().value(request.username_or_email.clone()).build() {
			if let ::core::option::Option::Some(user) = ::std::sync::Arc::clone(&self.user_repository).get_by_username(username.clone()).await? {
				::core::option::Option::Some(user)
			} else {
				errors.push(SignInErrResponse::UsernameNotFound { username: (*username).clone() });
				::core::option::Option::None
			}
		} else if let ::core::result::Result::Ok(email) = ::domain::Email::builder().value(request.username_or_email.clone()).build() {
			if let ::core::option::Option::Some(user) = ::std::sync::Arc::clone(&self.user_repository).get_by_email(email.clone()).await? {
				::core::option::Option::Some(user)
			} else {
				errors.push(SignInErrResponse::EmailNotFound { email: (*email).clone() });
				::core::option::Option::None
			}
		} else {
			errors.push(SignInErrResponse::UsernameOrEmailInvalid { username_or_email: request.username_or_email });
			::core::option::Option::None
		};

		let password = if let ::core::result::Result::Ok(password) = ::domain::Password::builder().value(request.password).build() {
			::core::option::Option::Some(password)
		} else {
			errors.push(SignInErrResponse::PasswordInvalid);
			::core::option::Option::None
		};

		let (::core::option::Option::Some(user), ::core::option::Option::Some(password)) = (user, password) else {
			return ::aliases::result::Fallible::Ok(SignInResponse::Err(errors));
		};

		if !::std::sync::Arc::clone(&self.password_hasher).verify(password, user.password).await? {
			errors.push(SignInErrResponse::PasswordMismatch);
			return ::aliases::result::Fallible::Ok(SignInResponse::Err(errors));
		}

		let auth_token_payload = crate::gateways::models::AuthenticationTokenPayload {
			user_id: user.id,
			user_role: user.role,
			expiry_timestamp: ::aliases::time::Timestamp::now(),
		};

		let auth_token = ::std::sync::Arc::clone(&self.auth_token_generator).generate(auth_token_payload).await?;

		::aliases::result::Fallible::Ok(SignInResponse::Ok(SignInOkResponse { token: auth_token }))
    }
}

#[derive(::bon::Builder)]
pub struct SignUpInteractor {
	user_repository: ::std::sync::Arc<dyn UserRepository + ::core::marker::Send + ::core::marker::Sync>,
	password_hasher: ::std::sync::Arc<dyn PasswordHasher + ::core::marker::Send + ::core::marker::Sync>,
}

#[async_trait]
impl SignUpBoundary for SignUpInteractor {
    async fn apply(self: ::std::sync::Arc<Self>, request: SignUpRequest) -> ::aliases::result::Fallible<SignUpResponse> {
		todo!()
	}
}