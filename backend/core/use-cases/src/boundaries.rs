use ::async_trait::async_trait;
#[cfg(feature = "wasm-bindings")]
use ::wasm_bindgen::prelude::*;

#[async_trait]
pub trait SignInBoundary {
    async fn apply(self: ::std::sync::Arc<Self>, request: SignInRequest)
        -> ::aliases::result::Fallible<SignInResponse>;
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
pub struct SignInRequest {
    pub username_or_email: ::aliases::string::String,
    pub password: ::aliases::string::String,
}

pub type SignInResponse = ::core::result::Result<SignInOkResponse, ::std::vec::Vec<SignInErrResponse>>;

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
pub struct SignInOkResponse {
    pub user_role: self::models::UserRole,
    pub token: ::aliases::string::String,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::thiserror::Error)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase", rename_all_fields = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
pub enum SignInErrResponse {
    #[error("User with username not found")]
    UsernameNotFound,

    #[error("User with email not found")]
    EmailNotFound,

    #[error("Invalid username or email format")]
    UsernameOrEmailInvalid,

    #[error("Invalid password")]
    PasswordInvalid,

    #[error("Password does not match")]
    PasswordMismatch,
}

#[async_trait]
pub trait SignUpBoundary {
    async fn apply(self: ::std::sync::Arc<Self>, request: SignUpRequest)
        -> ::aliases::result::Fallible<SignUpResponse>;
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
pub struct SignUpRequest {
    pub user_role: self::models::UserRole,

    pub username: ::aliases::string::String,
    pub email: ::aliases::string::String,
    pub password: ::aliases::string::String,

    pub first_name: ::aliases::string::String,
    pub last_name: ::aliases::string::String,
}

pub type SignUpResponse = ::core::result::Result<SignUpOkResponse, ::std::vec::Vec<SignUpErrResponse>>;

pub type SignUpOkResponse = ();

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::thiserror::Error)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase", rename_all_fields = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
pub enum SignUpErrResponse {
    #[error("Invalid username format")]
    UsernameInvalid,

    #[error("Invalid email format")]
    EmailInvalid,

    #[error("Invalid password format")]
    PasswordInvalid,

    #[error("User with username already exists")]
    UsernameAlreadyExists,

    #[error("User with email already exists")]
    EmailAlreadyExists,
}

pub mod models {
    #[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy)]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
    #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
    #[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
    #[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
    pub enum UserRole {
        Volunteer,
        EventManager,
        Administrator,
    }

    impl ::core::convert::From<::domain::UserRole> for UserRole {
        fn from(value: ::domain::UserRole) -> Self {
            match value {
                ::domain::UserRole::Volunteer => Self::Volunteer,
                ::domain::UserRole::EventManager => Self::EventManager,
                ::domain::UserRole::Administrator => Self::Administrator,
            }
        }
    }

    impl ::core::convert::From<UserRole> for ::domain::UserRole {
        fn from(value: UserRole) -> Self {
            match value {
                UserRole::Volunteer => Self::Volunteer,
                UserRole::EventManager => Self::EventManager,
                UserRole::Administrator => Self::Administrator,
            }
        }
    }
}
