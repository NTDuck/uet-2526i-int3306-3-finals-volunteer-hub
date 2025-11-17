use ::async_trait::async_trait;

#[async_trait]
pub trait CreateEventBoundary {
	async fn apply(self: ::std::sync::Arc<Self>, request: CreateEventRequest) -> ::axiom::result::Fallible<CreateEventResponse>;
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
pub struct CreateEventRequest {
    pub name: ::axiom::string::String,
    pub description: ::axiom::string::String,
    pub categories: ::std::vec::Vec<::axiom::string::String>,
    pub location: ::axiom::string::String,
}

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type CreateEventResponse = ::core::result::Result<CreateEventOkResponse, ::std::vec::Vec<CreateEventErrResponse>>;

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type CreateEventOkResponse = ();

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::thiserror::Error)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
pub enum CreateEventErrResponse {
    #[error("Invalid or expired authentication token")]
    AuthenticationTokenInvalid,

    #[error("User not authorized: expecting role `{expected_user_role}`, found `{user_role}`", expected_user_role = CreateEventUserRole::EventManager)]
    UserUnauthorized { user_role: CreateEventUserRole },

    // EventNameInvalid {
    //     name: ::axiom::string::String,
    // },

    // EventDescriptionInvalid {
    //     description: ::axiom::string::String,
    // },

    // EventCategoriesInvalid {
    //     categories: ::std::vec::Vec<::axiom::string::String>,
    // },

    // EventLocationInvalid {
    //     location: ::axiom::string::String,
    // },

    #[error("Event with name `{name}` already exists")]
    EventAlreadyExists {
        name: ::axiom::string::String,
    },
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::strum::Display)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
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
