use ::async_trait::async_trait;

#[async_trait]
pub trait RemoveEventBoundary {
    async fn apply(&self, request: RemoveEventRequest) -> ::axiom::result::Fallible<RemoveEventResponse>;
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
    #[error("Invalid or expired authentication token")]
    AuthenticationTokenInvalid,

    #[error("User with role `{user_role}` not authorized: must be `{expected_user_role}`", expected_user_role = RemoveEventUserRole::EventManager)]
    UserUnauthorized {
        user_role: RemoveEventUserRole,
    },

    #[error("Event not found")]
    EventNotFound,
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
