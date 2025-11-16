use ::async_trait::async_trait;

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
    pub user_role: SignUpUserRole,

    pub username: ::aliases::string::String,
    pub email: ::aliases::string::String,
    pub password: ::aliases::string::String,

    pub first_name: ::aliases::string::String,
    pub last_name: ::aliases::string::String,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::strum::Display)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged, rename_all = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
pub enum SignUpUserRole {
    Volunteer,
    EventManager,
    Administrator,
}

impl ::core::convert::From<::domain::UserRole> for SignUpUserRole {
    fn from(value: ::domain::UserRole) -> Self {
        match value {
            ::domain::UserRole::Volunteer => Self::Volunteer,
            ::domain::UserRole::EventManager => Self::EventManager,
            ::domain::UserRole::Administrator => Self::Administrator,
        }
    }
}

impl ::core::convert::From<SignUpUserRole> for ::domain::UserRole {
    fn from(value: SignUpUserRole) -> Self {
        match value {
            SignUpUserRole::Volunteer => Self::Volunteer,
            SignUpUserRole::EventManager => Self::EventManager,
            SignUpUserRole::Administrator => Self::Administrator,
        }
    }
}

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type SignUpResponse = ::core::result::Result<SignUpOkResponse, ::std::vec::Vec<SignUpErrResponse>>;

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type SignUpOkResponse = ();

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::thiserror::Error)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
pub enum SignUpErrResponse {
    #[error(transparent)]
    UsernameInvalid(#[from] #[cfg_attr(feature = "serde", serde(skip))] ::domain::UsernameBuilderError),

    #[error(transparent)]
    EmailInvalid(#[from] #[cfg_attr(feature = "serde", serde(skip))] ::domain::EmailBuilderError),

    #[error(transparent)]
    PasswordInvalid(#[from] #[cfg_attr(feature = "serde", serde(skip))] ::domain::PasswordBuilderError),

    #[error("User with username `{username}` already exists")]
    UsernameAlreadyExists {
        username: ::aliases::string::String,
    },

    #[error("User with email `{email}` already exists")]
    EmailAlreadyExists {
        email: ::aliases::string::String,
    },
}
