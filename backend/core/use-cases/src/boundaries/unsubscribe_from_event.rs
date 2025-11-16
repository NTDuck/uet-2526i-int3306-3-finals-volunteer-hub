use ::async_trait::async_trait;

#[async_trait]
pub trait UnsubscribeFromEventBoundary {
    async fn apply(self: ::std::sync::Arc<Self>, request: UnsubscribeFromEventRequest) -> ::aliases::result::Fallible<UnsubscribeFromEventResponse>;
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
pub struct UnsubscribeFromEventRequest {
    token: ::aliases::string::String,
    event_id: ::aliases::string::String,
}

pub type UnsubscribeFromEventResponse = ::core::result::Result<UnsubscribeFromEventOkResponse, ::std::vec::Vec<UnsubscribeFromEventErrResponse>>;

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type UnsubscribeFromEventOkResponse = ();

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::thiserror::Error)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
pub enum UnsubscribeFromEventErrResponse {
    #[error("Invalid or expired authentication token")]
    AuthenticationTokenInvalid,

    #[error("User not authorized: expecting role `{expected_user_role}`, found `{user_role}`", expected_user_role = UnsubscribeFromEventUserRole::Volunteer)]
    UserUnauthorized { user_role: UnsubscribeFromEventUserRole },
    
    #[error("Event not found")]  // or not published yet
    EventNotFound,

    #[error("User not subscribed")]
    UserNotSubscribed,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::strum::Display)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged, rename_all = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
pub enum UnsubscribeFromEventUserRole {
    Volunteer,
    EventManager,
    Administrator,
}

impl ::core::convert::From<::domain::UserRole> for UnsubscribeFromEventUserRole {
    fn from(value: ::domain::UserRole) -> Self {
        match value {
            ::domain::UserRole::Volunteer => Self::Volunteer,
            ::domain::UserRole::EventManager => Self::EventManager,
            ::domain::UserRole::Administrator => Self::Administrator,
        }
    }
}

impl ::core::convert::From<UnsubscribeFromEventUserRole> for ::domain::UserRole {
    fn from(value: UnsubscribeFromEventUserRole) -> Self {
        match value {
            UnsubscribeFromEventUserRole::Volunteer => Self::Volunteer,
            UnsubscribeFromEventUserRole::EventManager => Self::EventManager,
            UnsubscribeFromEventUserRole::Administrator => Self::Administrator,
        }
    }
}
