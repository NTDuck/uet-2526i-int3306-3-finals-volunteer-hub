use ::async_trait::async_trait;

#[async_trait]
pub trait ViewEventVolunteersBoundary {
    async fn apply(self: ::std::sync::Arc<Self>, request: ViewEventVolunteersRequest) -> ::axiom::result::Fallible<ViewEventVolunteersResponse>;
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi))]
pub struct ViewEventVolunteersRequest {
    pub token: ::axiom::string::String,
    pub event_id: ::axiom::string::String,
}

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type ViewEventVolunteersResponse = ::core::result::Result<ViewEventVolunteersOkResponse, ::std::vec::Vec<ViewEventVolunteersErrResponse>>;

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub struct ViewEventVolunteersOkResponse {
    pub volunteers: ::std::vec::Vec<ViewEventVolunteersVolunteer>,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub struct ViewEventVolunteersVolunteer {
    pub id: ::axiom::string::String,

    pub username: ::axiom::string::String,
    pub email: ::axiom::string::String,
    pub full_name: ::axiom::string::String,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::thiserror::Error)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum ViewEventVolunteersErrResponse {
    #[error("Invalid or expired authentication token")]
    AuthenticationTokenInvalid,

    #[error("User not authorized: expecting role `{expected_user_role}`, found `{user_role}`", expected_user_role = ViewEventVolunteersUserRole::Volunteer)]
    UserUnauthorized {
        #[allow(private_interfaces)]
        #[cfg_attr(feature = "serde", serde(skip))]
        user_role: ViewEventVolunteersUserRole,
    },

    #[error("Event not found")]
    EventNotFound,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::strum::Display)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
enum ViewEventVolunteersUserRole {
    Volunteer,
    EventManager,
    Administrator,
}

impl ::core::convert::From<::domain::UserRole> for ViewEventVolunteersUserRole {
    fn from(value: ::domain::UserRole) -> Self {
        match value {
            ::domain::UserRole::Volunteer => Self::Volunteer,
            ::domain::UserRole::EventManager => Self::EventManager,
            ::domain::UserRole::Administrator => Self::Administrator,
        }
    }
}

impl ::core::convert::From<ViewEventVolunteersUserRole> for ::domain::UserRole {
    fn from(value: ViewEventVolunteersUserRole) -> Self {
        match value {
            ViewEventVolunteersUserRole::Volunteer => Self::Volunteer,
            ViewEventVolunteersUserRole::EventManager => Self::EventManager,
            ViewEventVolunteersUserRole::Administrator => Self::Administrator,
        }
    }
}
