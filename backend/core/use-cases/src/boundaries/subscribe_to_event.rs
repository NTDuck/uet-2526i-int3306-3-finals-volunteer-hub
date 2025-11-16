use ::async_trait::async_trait;

#[async_trait]
pub trait SubscribeToEventBoundary {
    async fn apply(self: ::std::sync::Arc<Self>, request: SubscribeToEventRequest) -> ::aliases::result::Fallible<SubscribeToEventResponse>;
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
pub struct SubscribeToEventRequest {
    token: ::aliases::string::String,
    event_id: ::aliases::string::String,
}

pub type SubscribeToEventResponse = ::core::result::Result<SubscribeToEventOkResponse, ::std::vec::Vec<SubscribeToEventErrResponse>>;

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type SubscribeToEventOkResponse = ();

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::thiserror::Error)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
pub enum SubscribeToEventErrResponse {
    #[error("Invalid or expired authentication token")]
    AuthenticationTokenInvalid,

    #[error("User not authorized: expecting role `{expected_user_role}`, found `{user_role}`", expected_user_role = SubscribeToEventUserRole::Volunteer)]
    UserUnauthorized { user_role: SubscribeToEventUserRole },
    
    #[error("Event not found")]  // or not published yet
    EventNotFound,

    #[error("User already subscribed")]
    UserAlreadySubscribed,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::strum::Display)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged, rename_all = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
pub enum SubscribeToEventUserRole {
    Volunteer,
    EventManager,
    Administrator,
}

impl ::core::convert::From<::domain::UserRole> for SubscribeToEventUserRole {
    fn from(value: ::domain::UserRole) -> Self {
        match value {
            ::domain::UserRole::Volunteer => Self::Volunteer,
            ::domain::UserRole::EventManager => Self::EventManager,
            ::domain::UserRole::Administrator => Self::Administrator,
        }
    }
}

impl ::core::convert::From<SubscribeToEventUserRole> for ::domain::UserRole {
    fn from(value: SubscribeToEventUserRole) -> Self {
        match value {
            SubscribeToEventUserRole::Volunteer => Self::Volunteer,
            SubscribeToEventUserRole::EventManager => Self::EventManager,
            SubscribeToEventUserRole::Administrator => Self::Administrator,
        }
    }
}
