use ::axiom::prelude::*;

#[async_trait]
pub trait RemoveEventBoundary {
    async fn apply(
        self: ::std::sync::Arc<Self>, request: RemoveEventRequest,
    ) -> ::axiom::result::Fallible<RemoveEventResponse>;
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi))]
pub struct RemoveEventRequest {
    pub token: ::axiom::string::String,

    pub event_id: ::axiom::string::String,
}

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type RemoveEventResponse = ::core::result::Result<RemoveEventOkResponse, ::std::vec::Vec<RemoveEventErrResponse>>;

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type RemoveEventOkResponse = ();

#[derive(::core::fmt::Debug, ::core::clone::Clone)]
#[cfg_attr(feature = "serde", derive(::axiom::Erratum))]
#[cfg_attr(feature = "serde", erratum(rename_all = "kebab-case", rename_all_fields = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum RemoveEventErrResponse {
    #[error("Invalid authentication token")]
    AuthenticationTokenInvalid,

    #[error("Expired authentication token")]
    AuthenticationTokenExpired,

    #[error("User not found")]
    UserNotFound,

    #[error("User with role `{user_role}` not authorized: must be {}", super::fmt::join_with_comma_ad_hoc(.allowed_user_roles))]
    UserUnauthorized {
        user_role: RemoveEventUserRole,
        allowed_user_roles: ::std::vec::Vec<RemoveEventUserRole>,
    },

    #[error("User temporarily suspended")]
    UserSuspended,

    #[error("Event not found")]
    EventNotFound,

    #[error("Event with status `{event_status}` not eligible: must be {}", super::fmt::join_with_comma_ad_hoc(.allowed_event_statuses))]
    EventStatusNotEligible {
        event_status: RemoveEventEventStatus,
        allowed_event_statuses: ::std::vec::Vec<RemoveEventEventStatus>,
    },
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::strum::Display)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum RemoveEventUserRole {
    Volunteer,
    EventManager,
    Administrator,
}

impl ::core::convert::From<::domain::UserRole> for RemoveEventUserRole {
    fn from(value: ::domain::UserRole) -> Self {
        match value {
            ::domain::UserRole::Volunteer => Self::Volunteer,
            ::domain::UserRole::EventManager => Self::EventManager,
            ::domain::UserRole::Administrator => Self::Administrator,
        }
    }
}

impl ::core::convert::From<RemoveEventUserRole> for ::domain::UserRole {
    fn from(value: RemoveEventUserRole) -> Self {
        match value {
            RemoveEventUserRole::Volunteer => Self::Volunteer,
            RemoveEventUserRole::EventManager => Self::EventManager,
            RemoveEventUserRole::Administrator => Self::Administrator,
        }
    }
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::strum::Display)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum RemoveEventEventStatus {
    Created,
    Updated,
    Approved,
    Rejected,
}

impl ::core::convert::From<::domain::EventStatus> for RemoveEventEventStatus {
    fn from(value: ::domain::EventStatus) -> Self {
        match value {
            ::domain::EventStatus::Created { .. } => Self::Created,
            ::domain::EventStatus::Updated { .. } => Self::Updated,
            ::domain::EventStatus::Approved { .. } => Self::Approved,
            ::domain::EventStatus::Rejected { .. } => Self::Rejected,
        }
    }
}
