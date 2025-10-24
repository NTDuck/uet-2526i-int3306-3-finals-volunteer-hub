use ::async_trait::async_trait;
use crate::gateways::*;
use crate::boundaries::*;

#[derive(::bon::Builder)]
pub struct SignInInteractor {
	volunteer_repository: ::std::sync::Arc<dyn VolunteerRepository + ::core::marker::Send + ::core::marker::Sync>,
	event_manager_repository: ::std::sync::Arc<dyn EventManagerRepository + ::core::marker::Send + ::core::marker::Sync>,
	administrator_repository: ::std::sync::Arc<dyn AdministratorRepository + ::core::marker::Send + ::core::marker::Sync>,

	password_hasher: ::std::sync::Arc<dyn PasswordHasher + ::core::marker::Send + ::core::marker::Sync>,
}

#[async_trait]
impl SignInBoundary for SignInInteractor {
    async fn apply(self: ::std::sync::Arc<Self>, request: SignInRequest) -> ::aliases::result::Fallible<SignInResponse> {
    	todo!()
    }
}
