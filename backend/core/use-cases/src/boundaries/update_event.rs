use ::async_trait::async_trait;

#[async_trait]
pub trait UpdateEventBoundary {
    async fn apply(
        self: ::std::sync::Arc<Self>, request: UpdateEventRequest,
    ) -> ::axiom::result::Fallible<UpdateEventResponse>;
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi))]
pub struct UpdateEventRequest {
    pub token: ::axiom::string::String,

    pub event_id: ::axiom::string::String,

    pub event_name: ::core::option::Option<::axiom::string::String>,
    pub event_description: ::core::option::Option<::axiom::string::String>,
    pub event_categories: ::core::option::Option<::std::vec::Vec<::axiom::string::String>>,
    pub event_location: ::core::option::Option<::axiom::string::String>,
}

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type UpdateEventResponse = ::core::result::Result<UpdateEventOkResponse, ::std::vec::Vec<UpdateEventErrResponse>>;

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type UpdateEventOkResponse = ();

#[derive(::core::fmt::Debug, ::core::clone::Clone)]
#[cfg_attr(feature = "serde", derive(::axiom::Erratum))]
#[cfg_attr(feature = "serde", erratum(rename_all = "kebab-case", rename_all_fields = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum UpdateEventErrResponse {
    #[error("Invalid authentication token")]
    AuthenticationTokenInvalid,

    #[error("Authentication token expired")]
    AuthenticationTokenExpired,

    #[error("User not found")]
    UserNotFound,

    #[error("User with role `{user_role}` not authorized: must be `{expected_user_role}`", expected_user_role = UpdateEventUserRole::EventManager)]
    UserUnauthorized {
        user_role: UpdateEventUserRole,
    },

    #[error("User temporarily suspended")]
    UserSuspended,

    #[error("Invalid event name `{event_name}`: {hint}", hint = ::domain::EventName::hint())]
    EventNameInvalid {
        event_name: ::axiom::string::String,
    },

    #[error("Invalid event description `{event_description}`: {hint}", hint = ::domain::EventDescription::hint())]
    EventDescriptionInvalid {
        event_description: ::axiom::string::String,
    },

    #[error("Invalid event categories `{}`: {hint}", format(.event_categories), hint = ::domain::EventCategory::hint())]
    EventCategoriesInvalid {
        event_categories: ::std::vec::Vec<::axiom::string::String>,
    },

    #[error("Invalid event location `{event_location}`: {hint}", hint = ::domain::EventLocation::hint())]
    EventLocationInvalid {
        event_location: ::axiom::string::String,
    },

    #[error("Event not found")]
    EventNotFound,

    #[error("Event with status `{event_status}` not eligible: must be `{}` or `{}`", UpdateEventEventStatus::Created, UpdateEventEventStatus::Updated)]
    EventStatusNotEligible {
        event_status: UpdateEventEventStatus,
    }
}

fn format(values: &::std::vec::Vec<::axiom::string::String>) -> ::axiom::string::String {
    values
        .iter()
        .map(|value| ::std::format!("`{}`", value))
        .collect::<::std::vec::Vec<_>>()
        .join(", ")
        .into()
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::strum::Display)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum UpdateEventUserRole {
    Volunteer,
    EventManager,
    Administrator,
}

impl ::core::convert::From<::domain::UserRole> for UpdateEventUserRole {
    fn from(value: ::domain::UserRole) -> Self {
        match value {
            ::domain::UserRole::Volunteer => Self::Volunteer,
            ::domain::UserRole::EventManager => Self::EventManager,
            ::domain::UserRole::Administrator => Self::Administrator,
        }
    }
}

impl ::core::convert::From<UpdateEventUserRole> for ::domain::UserRole {
    fn from(value: UpdateEventUserRole) -> Self {
        match value {
            UpdateEventUserRole::Volunteer => Self::Volunteer,
            UpdateEventUserRole::EventManager => Self::EventManager,
            UpdateEventUserRole::Administrator => Self::Administrator,
        }
    }
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::strum::Display)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum UpdateEventEventStatus {
    Created,
    Updated,
    Approved,
    Rejected,
}

impl ::core::convert::From<::domain::EventStatus> for UpdateEventEventStatus {
    fn from(value: ::domain::EventStatus) -> Self {
        match value {
            ::domain::EventStatus::Created { .. } => Self::Created,
            ::domain::EventStatus::Updated { .. } => Self::Updated,
            ::domain::EventStatus::Approved { .. } => Self::Approved,
            ::domain::EventStatus::Rejected { .. } => Self::Rejected,
        }
    }
}
