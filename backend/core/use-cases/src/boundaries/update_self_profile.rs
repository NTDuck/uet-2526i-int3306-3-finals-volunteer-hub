use ::axiom::prelude::*;

#[async_trait]
pub trait UpdateSelfProfileBoundary {
    async fn apply(
        self: ::std::sync::Arc<Self>, request: UpdateSelfProfileRequest,
    ) -> ::axiom::result::Fallible<UpdateSelfProfileResponse>;
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi))]
pub struct UpdateSelfProfileRequest {
    pub token: ::axiom::string::String,

    pub password: ::core::option::Option<::axiom::string::String>,
    pub new_password: ::core::option::Option<::axiom::string::String>,

    pub full_name: ::core::option::Option<::axiom::string::String>,

    pub avatar: ::core::option::Option<::std::boxed::Box<[u8]>>,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::strum::Display)]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi))]
pub enum UpdateSelfProfileUserRole {
    Volunteer,
    EventManager,
    Administrator,
}

impl ::core::convert::From<::domain::UserRole> for UpdateSelfProfileUserRole {
    fn from(value: ::domain::UserRole) -> Self {
        match value {
            ::domain::UserRole::Volunteer => Self::Volunteer,
            ::domain::UserRole::EventManager => Self::EventManager,
            ::domain::UserRole::Administrator => Self::Administrator,
        }
    }
}

impl ::core::convert::From<UpdateSelfProfileUserRole> for ::domain::UserRole {
    fn from(value: UpdateSelfProfileUserRole) -> Self {
        match value {
            UpdateSelfProfileUserRole::Volunteer => Self::Volunteer,
            UpdateSelfProfileUserRole::EventManager => Self::EventManager,
            UpdateSelfProfileUserRole::Administrator => Self::Administrator,
        }
    }
}

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type UpdateSelfProfileResponse =
    ::core::result::Result<UpdateSelfProfileOkResponse, ::std::vec::Vec<UpdateSelfProfileErrResponse>>;

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type UpdateSelfProfileOkResponse = ();

#[derive(::core::fmt::Debug, ::core::clone::Clone)]
#[cfg_attr(feature = "serde", derive(::axiom::Erratum))]
#[cfg_attr(feature = "serde", erratum(rename_all = "kebab-case", rename_all_fields = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum UpdateSelfProfileErrResponse {
    #[error("Invalid authentication token")]
    AuthenticationTokenInvalid,

    #[error("Authentication token expired")]
    AuthenticationTokenExpired,

    #[error("User not found")]
    UserNotFound,

    #[error("User temporarily suspended")]
    UserSuspended,
    #[error("Invalid password: {hint}", hint = ::domain::Password::hint())]
    PasswordInvalid,

    #[error("Invalid new password: {hint}", hint = ::domain::Password::hint())]
    NewPasswordInvalid,

    #[error("Invalid full name `{full_name}`: {hint}", hint = ::domain::FullName::hint())]
    FullNameInvalid {
        full_name: ::axiom::string::String,
    },

    #[error("Invalid avatar")]
    AvatarInvalid,

    #[error("Passwords do not match")]
    PasswordMismatch,
}
