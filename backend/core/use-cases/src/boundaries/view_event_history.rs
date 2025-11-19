use ::async_trait::async_trait;

#[async_trait]
pub trait ViewEventHistoryBoundary {
    async fn apply(self: ::std::sync::Arc<Self>, request: ViewEventHistoryRequest) -> ::axiom::result::Fallible<ViewEventHistoryResponse>;
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi))]
pub struct ViewEventHistoryRequest {
    pub token: ::axiom::string::String,
}

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type ViewEventHistoryResponse = ::core::result::Result<ViewEventHistoryOkResponse, ::std::vec::Vec<ViewEventHistoryErrResponse>>;

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub struct ViewEventHistoryOkResponse {
    pub events: ::std::vec::Vec<ViewEventHistoryEvent>,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub struct ViewEventHistoryEvent {
    pub id: ::axiom::string::String,

    pub status: ViewEventHistoryEventStatus,
    pub registration_status: ViewEventHistoryEventRegistrationStatus,

    pub name: ::axiom::string::String,
    pub categories: ::std::vec::Vec<::axiom::string::String>,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::strum::Display)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum ViewEventHistoryEventStatus {
    Created,
    Approved,
    Rejected,
}

impl ::core::convert::From<::domain::EventStatus> for ViewEventHistoryEventStatus {
    fn from(value: ::domain::EventStatus) -> Self {
        match value {
            ::domain::EventStatus::Created { .. } => Self::Created,
            ::domain::EventStatus::Approved { .. } => Self::Approved,
            ::domain::EventStatus::Rejected { .. } => Self::Rejected,
        }
    }
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::strum::Display)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum ViewEventHistoryEventRegistrationStatus {
    Pending,
    Withdrawn,
    Accepted,
    Declined,
    Completed,
}

impl ::core::convert::From<::domain::EventRegistrationStatus> for ViewEventHistoryEventRegistrationStatus {
    fn from(value: ::domain::EventRegistrationStatus) -> Self {
        match value {
            ::domain::EventRegistrationStatus::Pending { .. } => Self::Pending,
            ::domain::EventRegistrationStatus::Withdrawn { .. } => Self::Withdrawn,
            ::domain::EventRegistrationStatus::Accepted { .. } => Self::Accepted,
            ::domain::EventRegistrationStatus::Declined { .. } => Self::Declined,
            ::domain::EventRegistrationStatus::Completed { .. } => Self::Completed,
        }
    }
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::thiserror::Error)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum ViewEventHistoryErrResponse {
    #[error("Invalid or expired authentication token")]
    AuthenticationTokenInvalid,

    #[error("User not authorized: expecting role `{expected_user_role}`, found `{user_role}`", expected_user_role = ViewEventHistoryUserRole::Volunteer)]
    UserUnauthorized {
        #[allow(private_interfaces)]
        #[cfg_attr(feature = "serde", serde(skip))]
        user_role: ViewEventHistoryUserRole,
    },
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::strum::Display)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
enum ViewEventHistoryUserRole {
    Volunteer,
    EventManager,
    Administrator,
}

impl ::core::convert::From<::domain::UserRole> for ViewEventHistoryUserRole {
    fn from(value: ::domain::UserRole) -> Self {
        match value {
            ::domain::UserRole::Volunteer => Self::Volunteer,
            ::domain::UserRole::EventManager => Self::EventManager,
            ::domain::UserRole::Administrator => Self::Administrator,
        }
    }
}

impl ::core::convert::From<ViewEventHistoryUserRole> for ::domain::UserRole {
    fn from(value: ViewEventHistoryUserRole) -> Self {
        match value {
            ViewEventHistoryUserRole::Volunteer => Self::Volunteer,
            ViewEventHistoryUserRole::EventManager => Self::EventManager,
            ViewEventHistoryUserRole::Administrator => Self::Administrator,
        }
    }
}
