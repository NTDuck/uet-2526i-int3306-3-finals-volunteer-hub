use ::async_trait::async_trait;
use crate::gateways::*;
use crate::boundaries::*;

#[derive(::bon::Builder)]
pub struct SignInInteractor {
	user_repository: ::std::sync::Arc<dyn UserRepository + ::core::marker::Send + ::core::marker::Sync>,

	password_hasher: ::std::sync::Arc<dyn PasswordHasher + ::core::marker::Send + ::core::marker::Sync>,
}

#[async_trait]
impl SignInBoundary for SignInInteractor {
    async fn apply(self: ::std::sync::Arc<Self>, request: SignInRequest) -> ::aliases::result::Fallible<SignInResponse> {
    	let user = if let ::core::result::Result::Ok(username) = ::domain::Username::builder().value(request.username_or_email.clone()).build() {
			if let ::core::option::Option::Some(user) = ::std::sync::Arc::clone(&self.user_repository).get_by_username(username.clone()).await? {
				user
			} else {
				return ::aliases::result::Fallible::Ok(SignInResponse::Err(::std::vec![
					SignInErrResponse::UsernameNotFound { username: (*username).clone(), },
				]));
			}
		} else if let ::core::result::Result::Ok(email) = ::domain::Email::builder().value(request.username_or_email.clone()).build() {
			if let ::core::option::Option::Some(user) = ::std::sync::Arc::clone(&self.user_repository).get_by_email(email.clone()).await? {
				user
			} else {
				return ::aliases::result::Fallible::Ok(SignInResponse::Err(::std::vec![
					SignInErrResponse::EmailNotFound { email: (*email).clone(), },
				]));
			}
		} else {
			return ::aliases::result::Fallible::Ok(SignInResponse::Err(::std::vec![
				SignInErrResponse::UsernameOrEmailInvalid { username_or_email: request.username_or_email.clone(), },
			]));
		};

		let password = ::domain::Password::builder().value(request.password).build();

		// if ::std::sync::Arc::clone(&self.password_hasher).verify(password, digest)
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