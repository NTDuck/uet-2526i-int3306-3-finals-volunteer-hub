use ::axiom::prelude::*;

#[async_trait]
pub trait ModerateEventRegistrationBoundary {
    async fn apply(
        self: ::std::sync::Arc<Self>, request: ModerateEventRegistrationRequest,
    ) -> ::axiom::result::Fallible<ModerateEventRegistrationResponse>;
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi))]
pub struct ModerateEventRegistrationRequest {
    pub token: ::axiom::string::String,

    pub event_registration_id: ::axiom::string::String,
    pub event_registration_status: ModerateEventRegistrationNewEventRegistrationStatus,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy)]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi))]
pub enum ModerateEventRegistrationNewEventRegistrationStatus {
    Accepted,
    Declined,
    Completed,
}

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type ModerateEventRegistrationResponse =
    ::core::result::Result<ModerateEventRegistrationOkResponse, ::std::vec::Vec<ModerateEventRegistrationErrResponse>>;

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type ModerateEventRegistrationOkResponse = ();

#[derive(::core::fmt::Debug, ::core::clone::Clone)]
#[cfg_attr(feature = "serde", derive(::axiom::Erratum))]
#[cfg_attr(feature = "serde", erratum(rename_all = "kebab-case", rename_all_fields = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum ModerateEventRegistrationErrResponse {
    #[error("Invalid authentication token")]
    AuthenticationTokenInvalid,

    #[error("Authentication token expired")]
    AuthenticationTokenExpired,

    #[error("User not found")]
    UserNotFound,

    #[error("User with role `{user_role}` not authorized: must be `{expected_user_role}`", expected_user_role = ModerateEventRegistrationUserRole::EventManager)]
    UserUnauthorized {
        user_role: ModerateEventRegistrationUserRole,
    },

    #[error("User temporarily suspended")]
    UserSuspended,

    #[error("Event registration not found")]
    EventRegistrationNotFound,

    #[error("Event registration with status `{event_registration_status}` not eligible: must be `{}`", format(.allowed_event_registration_statuses))]
    EventRegistrationStatusNotEligible {
        event_registration_status: ModerateEventRegistrationEventRegistrationStatus,
        allowed_event_registration_statuses: ::std::vec::Vec<ModerateEventRegistrationEventRegistrationStatus>,
    },
}

fn format<T: ::core::fmt::Display>(values: &[T]) -> ::axiom::string::String {
    match values {
        [] => ::core::default::Default::default(),
        [first] => ::std::format!("`{first}`").into(),
        [first, last] => ::std::format!("`{first}` or `{last}`").into(),
        [firsts @ .., last] => {
            let firsts = firsts.iter().map(|value| ::std::format!("`{value}`")).collect::<::std::vec::Vec<_>>().join(", ");
            ::std::format!("{firsts}, or `{last}`").into()
        }
    }
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

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::strum::Display)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum ModerateEventRegistrationEventRegistrationStatus {
    Pending,
    Withdrawn,
    Accepted,
    Declined,
    Completed,
}

impl ::core::convert::From<::domain::EventRegistrationStatus> for ModerateEventRegistrationEventRegistrationStatus {
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
