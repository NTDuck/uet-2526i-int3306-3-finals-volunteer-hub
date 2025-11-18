use ::async_trait::async_trait;

#[async_trait]
pub trait CompleteEventRegistrationBoundary {
    async fn apply(self: ::std::sync::Arc<Self>, request: CompleteEventRegistrationRequest) -> ::axiom::result::Fallible<CompleteEventRegistrationResponse>;
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi))]
pub struct CompleteEventRegistrationRequest {
    pub token: ::axiom::string::String,
    pub volunteer_id: ::axiom::string::String,
}

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type CompleteEventRegistrationResponse = ::core::result::Result<CompleteEventRegistrationOkResponse, ::std::vec::Vec<CompleteEventRegistrationErrResponse>>;

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type CompleteEventRegistrationOkResponse = ();

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::thiserror::Error)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum CompleteEventRegistrationErrResponse {
    #[error("Invalid or expired authentication token")]
    AuthenticationTokenInvalid,

    #[error("User not authorized: expecting role `{expected_user_role}`, found `{user_role}`", expected_user_role = CompleteEventRegistrationUserRole::EventManager)]
    UserUnauthorized {
        #[allow(private_interfaces)]
        #[cfg_attr(feature = "serde", serde(skip))]
        user_role: CompleteEventRegistrationUserRole,
    },

    #[error("Event registration not found")]
    EventRegistrationNotFound,

    #[error("Event registration already declined")]
    EventRegistrationAlreadyDeclined,

    #[error("Event registration already withdrawn")]
    EventRegistrationAlreadyWithdrawn,

    #[error("Event registration already completed")]
    EventRegistrationAlreadyCompleted,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::strum::Display)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum CompleteEventRegistrationUserRole {
    Volunteer,
    EventManager,
    Administrator,
}

impl ::core::convert::From<CompleteEventRegistrationUserRole> for ::domain::UserRole {
    fn from(value: CompleteEventRegistrationUserRole) -> Self {
        match value {
            CompleteEventRegistrationUserRole::Volunteer => Self::Volunteer,
            CompleteEventRegistrationUserRole::EventManager => Self::EventManager,
            CompleteEventRegistrationUserRole::Administrator => Self::Administrator,
        }
    }
}

impl ::core::convert::From<::domain::UserRole> for CompleteEventRegistrationUserRole {
    fn from(value: ::domain::UserRole) -> Self {
        match value {
            ::domain::UserRole::Volunteer => Self::Volunteer,
            ::domain::UserRole::EventManager => Self::EventManager,
            ::domain::UserRole::Administrator => Self::Administrator,
        }
    }
}
