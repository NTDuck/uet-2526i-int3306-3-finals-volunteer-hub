use ::async_trait::async_trait;

#[async_trait]
pub trait ViewUsersBoundary {
    async fn apply(self: ::std::sync::Arc<Self>, request: ViewUsersRequest) -> ::axiom::result::Fallible<ViewUsersResponse>;
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi))]
pub struct ViewUsersRequest {
    pub token: ::axiom::string::String,
}

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type ViewUsersResponse = ::core::result::Result<ViewUsersOkResponse, ::std::vec::Vec<ViewUsersErrResponse>>;

#[derive(::core::fmt::Debug, ::core::clone::Clone)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub struct ViewUsersOkResponse {
    pub users: ::std::vec::Vec<ViewUsersUser>,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub struct ViewUsersUser {
    pub id: ::axiom::string::String,

    pub role: ViewUsersUserRole,
    pub status: ViewUsersUserStatus,

    pub username: ::axiom::string::String,
    pub email: ::axiom::string::String,
    pub full_name: ::axiom::string::String,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::strum::Display)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum ViewUsersUserRole {
    Volunteer,
    EventManager,
    Administrator,
}

impl ::core::convert::From<::domain::UserRole> for ViewUsersUserRole {
    fn from(value: ::domain::UserRole) -> Self {
        match value {
            ::domain::UserRole::Volunteer => Self::Volunteer,
            ::domain::UserRole::EventManager => Self::EventManager,
            ::domain::UserRole::Administrator => Self::Administrator,
        }
    }
}

impl ::core::convert::From<ViewUsersUserRole> for ::domain::UserRole {
    fn from(value: ViewUsersUserRole) -> Self {
        match value {
            ViewUsersUserRole::Volunteer => Self::Volunteer,
            ViewUsersUserRole::EventManager => Self::EventManager,
            ViewUsersUserRole::Administrator => Self::Administrator,
        }
    }
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum ViewUsersUserStatus {
    Created,
    Suspended,
    Unsuspended,
}

impl ::core::convert::From<::domain::UserStatus> for ViewUsersUserStatus {
    fn from(value: ::domain::UserStatus) -> Self {
        match value {
            ::domain::UserStatus::Created => Self::Created,
            ::domain::UserStatus::Suspended { .. } => Self::Suspended,
            ::domain::UserStatus::Unsuspended { .. } => Self::Unsuspended,
        }
    }
}

#[derive(::core::fmt::Debug, ::core::clone::Clone)]
#[cfg_attr(feature = "serde", derive(::axiom::Erratum))]
#[cfg_attr(feature = "serde", erratum(rename_all = "kebab-case", rename_all_fields = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum ViewUsersErrResponse {
    #[error("Invalid or expired authentication token")]
    AuthenticationTokenInvalid,

    #[error("User with role `{user_role}` not authorized: must be `{expected_user_role}`", expected_user_role = ViewUsersUserRole::Administrator)]
    UserUnauthorized {
        user_role: ViewUsersUserRole,
    },
}
