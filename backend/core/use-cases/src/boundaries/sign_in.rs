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

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type SignInResponse = ::core::result::Result<SignInOkResponse, ::std::vec::Vec<SignInErrResponse>>;

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
pub struct SignInOkResponse {
    pub user_role: SignInUserRole,
    pub token: ::aliases::string::String,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::strum::Display)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged, rename_all = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
pub enum SignInUserRole {
    Volunteer,
    EventManager,
    Administrator,
}

impl ::core::convert::From<::domain::UserRole> for SignInUserRole {
    fn from(value: ::domain::UserRole) -> Self {
        match value {
            ::domain::UserRole::Volunteer => Self::Volunteer,
            ::domain::UserRole::EventManager => Self::EventManager,
            ::domain::UserRole::Administrator => Self::Administrator,
        }
    }
}

impl ::core::convert::From<SignInUserRole> for ::domain::UserRole {
    fn from(value: SignInUserRole) -> Self {
        match value {
            SignInUserRole::Volunteer => Self::Volunteer,
            SignInUserRole::EventManager => Self::EventManager,
            SignInUserRole::Administrator => Self::Administrator,
        }
    }
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::thiserror::Error)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
pub enum SignInErrResponse {
    #[error("User with username `{username}` not found")]
    UsernameNotFound {
        username: ::aliases::string::String,
    },

    #[error("User with email `{email}` not found")]
    EmailNotFound {
        email: ::aliases::string::String,
    },

    #[error("{} or {}", 0.0, 0.1)]
    UsernameOrEmailInvalid(
        #[cfg_attr(feature = "serde", serde(skip))]
        (::domain::UsernameBuilderError, ::domain::EmailBuilderError),
    ),

    #[error(transparent)]
    PasswordInvalid(#[from] #[cfg_attr(feature = "serde", serde(skip))] ::domain::PasswordBuilderError),

    #[error("Passwords do not match")]
    PasswordMismatch,
}
