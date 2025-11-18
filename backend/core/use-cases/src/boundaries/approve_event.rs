use ::async_trait::async_trait;

#[async_trait]
pub trait ApproveEventBoundary {

}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi))]
pub struct ApproveEventRequest {
    pub token: ::axiom::string::String,
    pub event_id: ::axiom::string::String,
}

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type ApproveEventResponse = ::core::result::Result<ApproveEventOkResponse, ::std::vec::Vec<ApproveEventErrResponse>>;

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type ApproveEventOkResponse = ();

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::thiserror::Error)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum ApproveEventErrResponse {
    #[error("Invalid or expired authentication token")]
    AuthenticationTokenInvalid,

    #[error("User not authorized: expecting role `{expected_user_role}`, found `{user_role}`", expected_user_role = ApproveEventUserRole::Administrator)]
    UserUnauthorized {
        #[allow(private_interfaces)]
        #[cfg_attr(feature = "serde", serde(skip))]
        user_role: ApproveEventUserRole,
    },

    #[error("Event not found")]
    EventNotFound,

    #[error("Event already approved")]
    EventAlreadyApproved,

    #[error("Event already completed")]
    EventAlreadyCompleted,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::strum::Display)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
enum ApproveEventUserRole {
    Volunteer,
    EventManager,
    Administrator,
}

impl ::core::convert::From<::domain::UserRole> for ApproveEventUserRole {
    fn from(value: ::domain::UserRole) -> Self {
        match value {
            ::domain::UserRole::Volunteer => Self::Volunteer,
            ::domain::UserRole::EventManager => Self::EventManager,
            ::domain::UserRole::Administrator => Self::Administrator,
        }
    }
}

impl ::core::convert::From<ApproveEventUserRole> for ::domain::UserRole {
    fn from(value: ApproveEventUserRole) -> Self {
        match value {
            ApproveEventUserRole::Volunteer => Self::Volunteer,
            ApproveEventUserRole::EventManager => Self::EventManager,
            ApproveEventUserRole::Administrator => Self::Administrator,
        }
    }
}
