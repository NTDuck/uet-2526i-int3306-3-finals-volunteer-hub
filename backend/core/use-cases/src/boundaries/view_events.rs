use ::async_trait::async_trait;

#[async_trait]
pub trait ViewEventsBoundary {
    async fn apply(
        self: ::std::sync::Arc<Self>, request: ViewEventsRequest,
    ) -> ::axiom::result::Fallible<ViewEventsResponse>;
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi))]
pub struct ViewEventsRequest {
    pub token: ::axiom::string::String,
    pub filter: ::core::option::Option<ViewEventsFilter>,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi))]
pub struct ViewEventsFilter {
    pub query: ::core::option::Option<::axiom::string::String>,
    
    pub statuses: ::core::option::Option<::std::vec::Vec<ViewEventsEventStatus>>,
    
    pub start_timestamp: ::core::option::Option<::axiom::string::String>,
    pub end_timestamp: ::core::option::Option<::axiom::string::String>,
}

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type ViewEventsResponse =
    ::core::result::Result<ViewEventsOkResponse, ::std::vec::Vec<ViewEventsErrResponse>>;

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub struct ViewEventsOkResponse {
    pub events: ::std::vec::Vec<ViewEventsEvent>,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub struct ViewEventsEvent {
    pub id: ::axiom::string::String,

    pub status: ViewEventsEventStatus,

    pub name: ::axiom::string::String,
    #[builder(with = |values: ::std::vec::Vec<impl ::core::convert::Into<::axiom::string::String>>| values.into_iter().map(::core::convert::Into::into).collect())]
    pub categories: ::std::vec::Vec<::axiom::string::String>,
    pub location: ::axiom::string::String,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::strum::Display)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
pub enum ViewEventsEventStatus {
    Created,
    Updated,
    Approved,
    Rejected,
}

impl ::core::convert::From<::domain::EventStatus> for ViewEventsEventStatus {
    fn from(value: ::domain::EventStatus) -> Self {
        match value {
            ::domain::EventStatus::Created { .. } => Self::Created,
            ::domain::EventStatus::Updated { .. } => Self::Updated,
            ::domain::EventStatus::Approved { .. } => Self::Approved,
            ::domain::EventStatus::Rejected { .. } => Self::Rejected,
        }
    }
}

impl ::core::convert::From<ViewEventsEventStatus> for crate::gateways::EventRepositorySearchFilterEventStatus {
    fn from(value: ViewEventsEventStatus) -> Self {
        match value {
            ViewEventsEventStatus::Created => Self::Created,
            ViewEventsEventStatus::Updated => Self::Updated,
            ViewEventsEventStatus::Approved => Self::Approved,
            ViewEventsEventStatus::Rejected => Self::Rejected,
        }
    }
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy)]
#[cfg_attr(feature = "serde", derive(::axiom::Erratum))]
#[cfg_attr(feature = "serde", erratum(rename_all = "kebab-case", rename_all_fields = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum ViewEventsErrResponse {
    #[error("Invalid authentication token")]
    AuthenticationTokenInvalid,

    #[error("Authentication token expired")]
    AuthenticationTokenExpired,

    #[error("User not found")]
    UserNotFound,

    #[error("User with role `{user_role}` not authorized: must be `{}` or `{}`", ViewEventsUserRole::EventManager, ViewEventsUserRole::Administrator)]
    UserUnauthorized {
        user_role: ViewEventsUserRole,
    },
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::strum::Display)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum ViewEventsUserRole {
    Volunteer,
    EventManager,
    Administrator,
}

impl ::core::convert::From<::domain::UserRole> for ViewEventsUserRole {
    fn from(value: ::domain::UserRole) -> Self {
        match value {
            ::domain::UserRole::Volunteer => Self::Volunteer,
            ::domain::UserRole::EventManager => Self::EventManager,
            ::domain::UserRole::Administrator => Self::Administrator,
        }
    }
}

impl ::core::convert::From<ViewEventsUserRole> for ::domain::UserRole {
    fn from(value: ViewEventsUserRole) -> Self {
        match value {
            ViewEventsUserRole::Volunteer => Self::Volunteer,
            ViewEventsUserRole::EventManager => Self::EventManager,
            ViewEventsUserRole::Administrator => Self::Administrator,
        }
    }
}
