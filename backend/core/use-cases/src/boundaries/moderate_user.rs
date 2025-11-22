use ::async_trait::async_trait;

#[async_trait]
pub trait ModerateUserBoundary {
    async fn apply(self: ::std::sync::Arc<Self>, request: ModerateUserRequest) -> ::axiom::result::Fallible<ModerateUserResponse>;
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi))]
pub struct ModerateUserRequest {
    pub token: ::axiom::string::String,

    pub user_id: ::axiom::string::String,
    pub user_status: ModerateUserUserStatus,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy)]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi))]
pub enum ModerateUserUserStatus {
    Suspended,
    Unsuspended,
}

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type ModerateUserResponse = ::core::result::Result<ModerateUserOkResponse, ::std::vec::Vec<ModerateUserErrResponse>>;

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type ModerateUserOkResponse = ();

#[derive(::core::fmt::Debug, ::core::clone::Clone)]
#[cfg_attr(feature = "serde", derive(::axiom::Erratum))]
#[cfg_attr(feature = "serde", erratum(rename_all = "kebab-case", rename_all_fields = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum ModerateUserErrResponse {
    #[error("Invalid or expired authentication token")]
    AuthenticationTokenInvalid,

    #[error("User with role `{user_role}` not authorized: must be `{expected_user_role}`", expected_user_role = ModerateUserUserRole::Administrator)]
    UserUnauthorized {
        user_role: ModerateUserUserRole,
    },
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::strum::Display)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum ModerateUserUserRole {
    Volunteer,
    EventManager,
    Administrator,
}

impl ::core::convert::From<::domain::UserRole> for ModerateUserUserRole {
    fn from(value: ::domain::UserRole) -> Self {
        match value {
            ::domain::UserRole::Volunteer => Self::Volunteer,
            ::domain::UserRole::EventManager => Self::EventManager,
            ::domain::UserRole::Administrator => Self::Administrator,
        }
    }
}

impl ::core::convert::From<ModerateUserUserRole> for ::domain::UserRole {
    fn from(value: ModerateUserUserRole) -> Self {
        match value {
            ModerateUserUserRole::Volunteer => Self::Volunteer,
            ModerateUserUserRole::EventManager => Self::EventManager,
            ModerateUserUserRole::Administrator => Self::Administrator,
        }
    }
}
