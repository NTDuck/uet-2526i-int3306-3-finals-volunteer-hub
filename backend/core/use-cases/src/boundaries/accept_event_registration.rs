use ::async_trait::async_trait;

#[async_trait]
pub trait AcceptEventRegistrationBoundary {
    async fn apply(self: ::std::sync::Arc<Self>, request: AcceptEventRegistrationRequest) -> ::axiom::result::Fallible<AcceptEventRegistrationResponse>;
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi))]
pub struct AcceptEventRegistrationRequest {
    pub token: ::axiom::string::String,
    pub volunteer_id: ::axiom::string::String,
}

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type AcceptEventRegistrationResponse = ::core::result::Result<AcceptEventRegistrationOkResponse, ::std::vec::Vec<AcceptEventRegistrationErrResponse>>;

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type AcceptEventRegistrationOkResponse = ();

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::thiserror::Error)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum AcceptEventRegistrationErrResponse {
    #[error("Invalid or expired authentication token")]
    AuthenticationTokenInvalid,

    #[error("User not authorized: expecting role `{expected_user_role}`, found `{user_role}`", expected_user_role = AcceptEventRegistrationUserRole::EventManager)]
    UserUnauthorized {
        #[allow(private_interfaces)]
        #[cfg_attr(feature = "serde", serde(skip))]
        user_role: AcceptEventRegistrationUserRole,
    },

    #[error("Event registration not found")]
    EventRegistrationNotFound,

    #[error("Event registration already accepted")]
    EventRegistrationAlreadyAccepted,

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
pub enum AcceptEventRegistrationUserRole {
    Volunteer,
    EventManager,
    Administrator,
}

impl ::core::convert::From<AcceptEventRegistrationUserRole> for ::domain::UserRole {
    fn from(value: AcceptEventRegistrationUserRole) -> Self {
        match value {
            AcceptEventRegistrationUserRole::Volunteer => Self::Volunteer,
            AcceptEventRegistrationUserRole::EventManager => Self::EventManager,
            AcceptEventRegistrationUserRole::Administrator => Self::Administrator,
        }
    }
}

impl ::core::convert::From<::domain::UserRole> for AcceptEventRegistrationUserRole {
    fn from(value: ::domain::UserRole) -> Self {
        match value {
            ::domain::UserRole::Volunteer => Self::Volunteer,
            ::domain::UserRole::EventManager => Self::EventManager,
            ::domain::UserRole::Administrator => Self::Administrator,
        }
    }
}
