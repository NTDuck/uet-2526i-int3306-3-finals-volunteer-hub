use ::axiom::prelude::*;

#[async_trait]
pub trait CreateEventBoundary {
    async fn apply(
        self: ::std::sync::Arc<Self>, request: CreateEventRequest,
    ) -> ::axiom::result::Fallible<CreateEventResponse>;
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi))]
pub struct CreateEventRequest {
    pub token: ::axiom::string::String,

    pub event_name: ::axiom::string::String,
    pub event_description: ::axiom::string::String,
    pub event_categories: ::std::vec::Vec<::axiom::string::String>,
    pub event_location: ::axiom::string::String,
}

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type CreateEventResponse = ::core::result::Result<CreateEventOkResponse, ::std::vec::Vec<CreateEventErrResponse>>;

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type CreateEventOkResponse = ();

#[derive(::core::fmt::Debug, ::core::clone::Clone)]
#[cfg_attr(feature = "serde", derive(::axiom::Erratum))]
#[cfg_attr(feature = "serde", erratum(rename_all = "kebab-case", rename_all_fields = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum CreateEventErrResponse {
    #[error("Invalid authentication token")]
    AuthenticationTokenInvalid,

    #[error("Authentication token expired")]
    AuthenticationTokenExpired,

    #[error("User not found")]
    UserNotFound,

    #[error("User with role `{user_role}` not authorized: must be {}", super::utils::fmt::join_with_comma_ad_hoc(.allowed_user_roles))]
    UserUnauthorized {
        user_role: CreateEventUserRole,
        allowed_user_roles: ::std::vec::Vec<CreateEventUserRole>,
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

    #[error("Invalid event categories `{}`: {hint}", super::utils::fmt::join_with_comma(.event_categories), hint = ::domain::EventCategory::hint())]
    EventCategoriesInvalid {
        event_categories: ::std::vec::Vec<::axiom::string::String>,
    },

    #[error("Invalid event location `{event_location}`: {hint}", hint = ::domain::EventLocation::hint())]
    EventLocationInvalid {
        event_location: ::axiom::string::String,
    },
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::strum::Display)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum CreateEventUserRole {
    Volunteer,
    EventManager,
    Administrator,
}

impl ::core::convert::From<::domain::UserRole> for CreateEventUserRole {
    fn from(value: ::domain::UserRole) -> Self {
        match value {
            ::domain::UserRole::Volunteer => Self::Volunteer,
            ::domain::UserRole::EventManager => Self::EventManager,
            ::domain::UserRole::Administrator => Self::Administrator,
        }
    }
}

impl ::core::convert::From<CreateEventUserRole> for ::domain::UserRole {
    fn from(value: CreateEventUserRole) -> Self {
        match value {
            CreateEventUserRole::Volunteer => Self::Volunteer,
            CreateEventUserRole::EventManager => Self::EventManager,
            CreateEventUserRole::Administrator => Self::Administrator,
        }
    }
}
