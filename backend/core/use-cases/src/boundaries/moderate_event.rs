use ::async_trait::async_trait;

#[async_trait]
pub trait ModerateEventBoundary {
    async fn apply(self: ::std::sync::Arc<Self>, request: ModerateEventRequest) -> ::axiom::result::Fallible<ModerateEventResponse>;
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi))]
pub struct ModerateEventRequest {
    pub token: ::axiom::string::String,

    pub event_id: ::axiom::string::String,
    pub event_status: ModerateEventEventStatus,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy)]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi))]
pub enum ModerateEventEventStatus {
    Approved,
    Rejected,
}

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type ModerateEventResponse = ::core::result::Result<ModerateEventOkResponse, ::std::vec::Vec<ModerateEventErrResponse>>;

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type ModerateEventOkResponse = ();

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy)]
#[cfg_attr(feature = "serde", derive(::axiom::Erratum))]
#[cfg_attr(feature = "serde", erratum(rename_all = "kebab-case", rename_all_fields = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum ModerateEventErrResponse {
    #[error("Invalid or expired authentication token")]
    AuthenticationTokenInvalid,

    #[error("User with role `{user_role}` not authorized: must be `{expected_user_role}`", expected_user_role = ModerateEventUserRole::Administrator)]
    UserUnauthorized {
        user_role: ModerateEventUserRole,
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
pub enum ModerateEventUserRole {
    Volunteer,
    EventManager,
    Administrator,
}

impl ::core::convert::From<::domain::UserRole> for ModerateEventUserRole {
    fn from(value: ::domain::UserRole) -> Self {
        match value {
            ::domain::UserRole::Volunteer => Self::Volunteer,
            ::domain::UserRole::EventManager => Self::EventManager,
            ::domain::UserRole::Administrator => Self::Administrator,
        }
    }
}

impl ::core::convert::From<ModerateEventUserRole> for ::domain::UserRole {
    fn from(value: ModerateEventUserRole) -> Self {
        match value {
            ModerateEventUserRole::Volunteer => Self::Volunteer,
            ModerateEventUserRole::EventManager => Self::EventManager,
            ModerateEventUserRole::Administrator => Self::Administrator,
        }
    }
}
