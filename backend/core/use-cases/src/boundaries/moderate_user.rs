use ::axiom::prelude::*;

#[async_trait]
pub trait ModerateUserBoundary {
    async fn apply(
        self: ::std::sync::Arc<Self>, request: ModerateUserRequest,
    ) -> ::axiom::result::Fallible<ModerateUserResponse>;
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
    pub user_status: ModerateUserNewUserStatus,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy)]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi))]
pub enum ModerateUserNewUserStatus {
    Suspended,
    Unsuspended,
}

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type ModerateUserResponse =
    ::core::result::Result<ModerateUserOkResponse, ::std::vec::Vec<ModerateUserErrResponse>>;

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type ModerateUserOkResponse = ();

#[derive(::core::fmt::Debug, ::core::clone::Clone)]
#[cfg_attr(feature = "serde", derive(::axiom::Erratum))]
#[cfg_attr(feature = "serde", erratum(rename_all = "kebab-case", rename_all_fields = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum ModerateUserErrResponse {
    #[error("Invalid authentication token")]
    AuthenticationTokenInvalid,

    #[error("Authentication token expired")]
    AuthenticationTokenExpired,

    #[error("User not found")]
    UserNotFound, // Well this will surely be confusing ...

    #[error("User with role `{user_role}` not authorized: must be {}", super::utils::fmt::join_with_comma_ad_hoc(.allowed_user_roles))]
    UserUnauthorized {
        user_role: ModerateUserUserRole,
        allowed_user_roles: ::std::vec::Vec<ModerateUserUserRole>,
    },

    #[error("User with role `{user_role}` not eligible: must be {}", super::utils::fmt::join_with_comma_ad_hoc(.allowed_user_roles))]
    UserRoleNotEligible {
        user_role: ModerateUserUserRole,
        allowed_user_roles: ::std::vec::Vec<ModerateUserUserRole>,
    },

    #[error("User with status `{user_status}` not eligible: must be {}", super::utils::fmt::join_with_comma_ad_hoc(.allowed_user_statuses))]
    UserStatusNotEligible {
        user_status: ModerateUserUserStatus,
        allowed_user_statuses: ::std::vec::Vec<ModerateUserUserStatus>,
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

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::strum::Display)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum ModerateUserUserStatus {
    Created,
    Suspended,
    Unsuspended,
}

impl ::core::convert::From<::domain::UserStatus> for ModerateUserUserStatus {
    fn from(value: ::domain::UserStatus) -> Self {
        match value {
            ::domain::UserStatus::Created => Self::Created,
            ::domain::UserStatus::Suspended { .. } => Self::Suspended,
            ::domain::UserStatus::Unsuspended { .. } => Self::Unsuspended,
        }
    }
}
