use ::axiom::prelude::*;

#[async_trait]
pub trait UnsubscribeFromEventBoundary {
    async fn apply(
        self: ::std::sync::Arc<Self>, request: UnsubscribeFromEventRequest,
    ) -> ::axiom::result::Fallible<UnsubscribeFromEventResponse>;
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi))]
pub struct UnsubscribeFromEventRequest {
    pub token: ::axiom::string::String,

    pub event_or_registration_id: ::axiom::string::String,
}

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type UnsubscribeFromEventResponse =
    ::core::result::Result<UnsubscribeFromEventOkResponse, ::std::vec::Vec<UnsubscribeFromEventErrResponse>>;

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type UnsubscribeFromEventOkResponse = ();

#[derive(::core::fmt::Debug, ::core::clone::Clone)]
#[cfg_attr(feature = "serde", derive(::axiom::Erratum))]
#[cfg_attr(feature = "serde", erratum(rename_all = "kebab-case", rename_all_fields = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum UnsubscribeFromEventErrResponse {
    #[error("Invalid authentication token")]
    AuthenticationTokenInvalid,

    #[error("Authentication token expired")]
    AuthenticationTokenExpired,

    #[error("User not found")]
    UserNotFound,

    #[error("User with role `{user_role}` not authorized: must be {}", super::fmt::join_with_comma_ad_hoc(.allowed_user_roles))]
    UserUnauthorized {
        user_role: UnsubscribeFromEventUserRole,
        allowed_user_roles: ::std::vec::Vec<UnsubscribeFromEventUserRole>,
    },

    #[error("User temporarily suspended")]
    UserSuspended,

    #[error("Event registration not found")]
    EventRegistrationNotFound,

    #[error("Event registration with status `{event_registration_status}` not eligible: must be {}", super::fmt::join_with_comma_ad_hoc(.allowed_event_registration_statuses))]
    EventRegistrationStatusNotEligible {
        event_registration_status: UnsubscribeFromEventEventRegistrationStatus,
        allowed_event_registration_statuses: ::std::vec::Vec<UnsubscribeFromEventEventRegistrationStatus>,
    },
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::strum::Display)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum UnsubscribeFromEventUserRole {
    Volunteer,
    EventManager,
    Administrator,
}

impl ::core::convert::From<::domain::UserRole> for UnsubscribeFromEventUserRole {
    fn from(value: ::domain::UserRole) -> Self {
        match value {
            ::domain::UserRole::Volunteer => Self::Volunteer,
            ::domain::UserRole::EventManager => Self::EventManager,
            ::domain::UserRole::Administrator => Self::Administrator,
        }
    }
}

impl ::core::convert::From<UnsubscribeFromEventUserRole> for ::domain::UserRole {
    fn from(value: UnsubscribeFromEventUserRole) -> Self {
        match value {
            UnsubscribeFromEventUserRole::Volunteer => Self::Volunteer,
            UnsubscribeFromEventUserRole::EventManager => Self::EventManager,
            UnsubscribeFromEventUserRole::Administrator => Self::Administrator,
        }
    }
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::strum::Display)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum UnsubscribeFromEventEventRegistrationStatus {
    Pending,
    Withdrawn,
    Accepted,
    Declined,
    Completed,
}

impl ::core::convert::From<::domain::EventRegistrationStatus> for UnsubscribeFromEventEventRegistrationStatus {
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
