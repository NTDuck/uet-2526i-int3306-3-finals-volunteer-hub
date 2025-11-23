use ::async_trait::async_trait;

#[async_trait]
pub trait ModerateEventRegistrationBoundary {
    async fn apply(self: ::std::sync::Arc<Self>, request: ModerateEventRegistrationRequest) -> ::axiom::result::Fallible<ModerateEventRegistrationResponse>;
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi))]
pub struct ModerateEventRegistrationRequest {
    pub token: ::axiom::string::String,

    pub registration_id: ::axiom::string::String,
    pub registration_status: ModerateEventRegistrationEventRegistrationStatus,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy)]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi))]
pub enum ModerateEventRegistrationEventRegistrationStatus {
    Accepted,
    Declined,
    Completed,
}

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type ModerateEventRegistrationResponse = ::core::result::Result<ModerateEventRegistrationOkResponse, ::std::vec::Vec<ModerateEventRegistrationErrResponse>>;

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type ModerateEventRegistrationOkResponse = ();

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy)]
#[cfg_attr(feature = "serde", derive(::axiom::Erratum))]
#[cfg_attr(feature = "serde", erratum(rename_all = "kebab-case", rename_all_fields = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum ModerateEventRegistrationErrResponse {
    #[error("Invalid or expired authentication token")]
    AuthenticationTokenInvalid,

    #[error("User with role `{user_role}` not authorized: must be `{expected_user_role}`", expected_user_role = ModerateEventRegistrationUserRole::EventManager)]
    UserUnauthorized {
        user_role: ModerateEventRegistrationUserRole,
    },

    #[error("User temporarily suspended")]
    UserSuspended,

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
pub enum ModerateEventRegistrationUserRole {
    Volunteer,
    EventManager,
    Administrator,
}

impl ::core::convert::From<ModerateEventRegistrationUserRole> for ::domain::UserRole {
    fn from(value: ModerateEventRegistrationUserRole) -> Self {
        match value {
            ModerateEventRegistrationUserRole::Volunteer => Self::Volunteer,
            ModerateEventRegistrationUserRole::EventManager => Self::EventManager,
            ModerateEventRegistrationUserRole::Administrator => Self::Administrator,
        }
    }
}

impl ::core::convert::From<::domain::UserRole> for ModerateEventRegistrationUserRole {
    fn from(value: ::domain::UserRole) -> Self {
        match value {
            ::domain::UserRole::Volunteer => Self::Volunteer,
            ::domain::UserRole::EventManager => Self::EventManager,
            ::domain::UserRole::Administrator => Self::Administrator,
        }
    }
}
