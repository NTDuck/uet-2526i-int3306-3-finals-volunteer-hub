use ::async_trait::async_trait;

#[async_trait]
pub trait SignInBoundary {
    async fn apply(self: ::std::sync::Arc<Self>, request: SignInRequest) -> ::aliases::result::Fallible<SignInResponse>;
}

#[derive(::core::fmt::Debug, ::core::clone::Clone)]
#[derive(::bon::Builder)]
#[builder(on(::aliases::string::String, into))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
pub struct SignInRequest {
    pub username_or_email: ::aliases::string::String,
    pub password: ::aliases::string::String,
}

pub type SignInResponse = ::core::result::Result<SignInOkResponse, ::std::vec::Vec<SignInErrResponse>>;

#[derive(::core::fmt::Debug, ::core::clone::Clone)]
#[derive(::bon::Builder)]
#[builder(on(::aliases::string::String, into))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
pub struct SignInOkResponse {
    #[cfg_attr(feature = "wasm-bindings", wasm_bindgen(readonly))]
    pub token: ::aliases::string::String,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone)]
#[derive(::thiserror::Error)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase", rename_all_fields = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
pub enum SignInErrResponse {
    #[error("User with username {username} not found")]
    UsernameNotFound {
        username: ::aliases::string::String,
    },

    #[error("User with email {email} not found")]
    EmailNotFound {
        email: ::aliases::string::String,
    },

    #[error("Password does not match")]
    PasswordMismatch,
}

#[async_trait]
pub trait VolunteerSignUpBoundary {
    async fn apply(self: ::std::sync::Arc<Self>, request: VolunteerSignUpRequest) -> ::aliases::result::Fallible<VolunteerSignUpResponse>;
}

#[derive(::core::fmt::Debug, ::core::clone::Clone)]
#[derive(::bon::Builder)]
#[builder(on(::aliases::string::String, into))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
pub struct VolunteerSignUpRequest {
    pub username: ::aliases::string::String,
    pub email: ::aliases::string::String,
    pub password: ::aliases::string::String,

    pub first_name: ::aliases::string::String,
    pub last_name: ::aliases::string::String,
}

pub type VolunteerSignUpResponse = ::core::result::Result<VolunteerSignUpOkResponse, ::std::vec::Vec<VolunteerSignUpErrResponse>>;

pub type VolunteerSignUpOkResponse = ();

#[derive(::core::fmt::Debug, ::core::clone::Clone)]
#[derive(::thiserror::Error)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase", rename_all_fields = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
pub enum VolunteerSignUpErrResponse {

}

#[async_trait]
pub trait EventManagerSignUpBoundary {
    async fn apply(self: ::std::sync::Arc<Self>, request: EventManagerSignUpRequest) -> ::aliases::result::Fallible<EventManagerSignUpResponse>;
}

#[derive(::core::fmt::Debug, ::core::clone::Clone)]
#[derive(::bon::Builder)]
#[builder(on(::aliases::string::String, into))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
pub struct EventManagerSignUpRequest {
    pub username: ::aliases::string::String,
    pub email: ::aliases::string::String,
    pub password: ::aliases::string::String,

    pub first_name: ::aliases::string::String,
    pub last_name: ::aliases::string::String,
}

pub type EventManagerSignUpResponse = ::core::result::Result<EventManagerSignUpOkResponse, ::std::vec::Vec<EventManagerSignUpErrResponse>>;

pub type EventManagerSignUpOkResponse = ();

#[derive(::core::fmt::Debug, ::core::clone::Clone)]
#[derive(::thiserror::Error)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase", rename_all_fields = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
pub enum EventManagerSignUpErrResponse {

}

#[async_trait]
pub trait AdministratorSignUpBoundary {
    async fn apply(self: ::std::sync::Arc<Self>, request: AdministratorSignUpRequest) -> ::aliases::result::Fallible<AdministratorSignUpResponse>;
}

#[derive(::core::fmt::Debug, ::core::clone::Clone)]
#[derive(::bon::Builder)]
#[builder(on(::aliases::string::String, into))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
pub struct AdministratorSignUpRequest {
    pub username: ::aliases::string::String,
    pub email: ::aliases::string::String,
    pub password: ::aliases::string::String,

    pub first_name: ::aliases::string::String,
    pub last_name: ::aliases::string::String,
}

pub type AdministratorSignUpResponse = ::core::result::Result<AdministratorSignUpOkResponse, ::std::vec::Vec<AdministratorSignUpErrResponse>>;

pub type AdministratorSignUpOkResponse = ();

#[derive(::core::fmt::Debug, ::core::clone::Clone)]
#[derive(::thiserror::Error)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase", rename_all_fields = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
pub enum AdministratorSignUpErrResponse {

}

pub mod models {

}
