use ::async_trait::async_trait;

#[async_trait]
pub trait UpdateEventBoundary {
	async fn apply(self: ::std::sync::Arc<Self>, request: UpdateEventRequest) -> ::axiom::result::Fallible<UpdateEventResponse>;
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
    pub event_status: ::core::option::Option<UpdateEventEventStatus>,

    pub event_name: ::core::option::Option<::axiom::string::String>,
    pub event_description: ::core::option::Option<::axiom::string::String>,
    pub event_categories: ::core::option::Option<::std::vec::Vec<::axiom::string::String>>,
    pub event_location: ::core::option::Option<::axiom::string::String>,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy)]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi))]
pub enum UpdateEventEventStatus {
    Completed,
}

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type UpdateEventResponse = ::core::result::Result<UpdateEventOkResponse, ::std::vec::Vec<UpdateEventErrResponse>>;

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type UpdateEventOkResponse = ();

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::thiserror::Error)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum UpdateEventErrResponse {
    #[error("Invalid or expired authentication token")]
    AuthenticationTokenInvalid,

    #[error("User not authorized: expecting role `{expected_user_role}`, found `{user_role}`", expected_user_role = UpdateEventUserRole::EventManager)]
    UserUnauthorized {
        #[allow(private_interfaces)]
        #[cfg_attr(feature = "serde", serde(skip))]
        user_role: UpdateEventUserRole,
    },

    #[error("Invalid event name `{event_name}`: {hint}", hint = ::domain::EventName::hint())]
    EventNameInvalid {
        #[cfg_attr(feature = "serde", serde(skip))]
        event_name: ::axiom::string::String,
    },

    #[error("Invalid event description `{event_description}`: {hint}", hint = ::domain::EventDescription::hint())]
    EventDescriptionInvalid {
        #[cfg_attr(feature = "serde", serde(skip))]
        event_description: ::axiom::string::String,
    },

    #[error("Invalid event categories `{}`: {hint}", format(.event_categories), hint = ::domain::EventCategory::hint())]
    EventCategoriesInvalid {
        #[cfg_attr(feature = "serde", serde(skip))]
        event_categories: ::std::vec::Vec<::axiom::string::String>,
    },

    #[error("Invalid event location `{event_location}`: {hint}", hint = ::domain::EventLocation::hint())]
    EventLocationInvalid {
        #[cfg_attr(feature = "serde", serde(skip))]
        event_location: ::axiom::string::String,
    },

    #[error("Event not found")]
    EventNotFound,

    #[error("Event with name `{event_name}` already exists")]
    EventNameAlreadyExists {
        event_name: ::axiom::string::String,
    },

    #[error("Event not approved")]
    EventNotApproved,

    #[error("Event already declined")]
    EventAlreadyDeclined,

    #[error("Event already completed")]
    EventAlreadyCompleted,
}

fn format(values: &::std::vec::Vec<::axiom::string::String>) -> ::axiom::string::String {
    values.into_iter()
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
enum UpdateEventUserRole {
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
