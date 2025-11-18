use ::async_trait::async_trait;

#[async_trait]
pub trait RejectEventBoundary {

}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi))]
pub struct RejectEventRequest {
    pub token: ::axiom::string::String,
    pub event_id: ::axiom::string::String,
}

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type RejectEventResponse = ::core::result::Result<RejectEventOkResponse, ::std::vec::Vec<RejectEventErrResponse>>;

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type RejectEventOkResponse = ();

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::thiserror::Error)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum RejectEventErrResponse {
    #[error("Invalid or expired authentication token")]
    AuthenticationTokenInvalid,

    #[error("User not authorized: expecting role `{expected_user_role}`, found `{user_role}`", expected_user_role = RejectEventUserRole::Administrator)]
    UserUnauthorized {
        #[allow(private_interfaces)]
        #[cfg_attr(feature = "serde", serde(skip))]
        user_role: RejectEventUserRole,
    },

    #[error("Event not found")]
    EventNotFound,

    #[error("Event already rejectd")]
    EventAlreadyRejectd,

    #[error("Event already completed")]
    EventAlreadyCompleted,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::strum::Display)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
enum RejectEventUserRole {
    Volunteer,
    EventManager,
    Administrator,
}

impl ::core::convert::From<::domain::UserRole> for RejectEventUserRole {
    fn from(value: ::domain::UserRole) -> Self {
        match value {
            ::domain::UserRole::Volunteer => Self::Volunteer,
            ::domain::UserRole::EventManager => Self::EventManager,
            ::domain::UserRole::Administrator => Self::Administrator,
        }
    }
}

impl ::core::convert::From<RejectEventUserRole> for ::domain::UserRole {
    fn from(value: RejectEventUserRole) -> Self {
        match value {
            RejectEventUserRole::Volunteer => Self::Volunteer,
            RejectEventUserRole::EventManager => Self::EventManager,
            RejectEventUserRole::Administrator => Self::Administrator,
        }
    }
}
