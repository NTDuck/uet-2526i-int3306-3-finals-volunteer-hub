use ::async_trait::async_trait;

#[async_trait]
pub trait ViewPublishedEventsBoundary {
    async fn apply(self: ::std::sync::Arc<Self>, request: ViewPublishedEventsRequest)
        -> ::aliases::result::Fallible<ViewPublishedEventsResponse>;
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
pub struct ViewPublishedEventsRequest {
    pub token: ::aliases::string::String,

    #[cfg_attr(feature = "serde", serde(flatten))]
    pub filter: ViewPublishedEventsFilter,
}

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type ViewPublishedEventsResponse = ::core::result::Result<ViewPublishedEventsOkResponse, ::std::vec::Vec<ViewPublishedEventsErrResponse>>;

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
pub struct ViewPublishedEventsOkResponse {
    pub events: ::std::vec::Vec<ViewPublishedEventsEvent>,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
pub struct ViewPublishedEventsEvent {
    pub id: ::aliases::string::String,

    pub status: ViewPublishedEventsEventStatus,

    pub name: ::aliases::string::String,
    pub categories: ::std::vec::Vec<::aliases::string::String>,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::strum::Display)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged, rename_all = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
pub enum ViewPublishedEventsEventStatus {
    Created,
    Approved,
    Rejected,
    Completed,
}

impl ::core::convert::From<::domain::EventStatus> for ViewPublishedEventsEventStatus {
    fn from(value: ::domain::EventStatus) -> Self {
        match value {
            ::domain::EventStatus::Created { .. } => Self::Created,
            ::domain::EventStatus::Approved { .. } => Self::Approved,
            ::domain::EventStatus::Rejected { .. } => Self::Rejected,
            ::domain::EventStatus::Completed { .. } => Self::Completed,
        }
    }
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::thiserror::Error)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
pub enum ViewPublishedEventsErrResponse {
    #[error("Invalid or expired authentication token")]
    AuthenticationTokenInvalid,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
pub struct ViewPublishedEventsFilter {
    pub name: ::core::option::Option<::aliases::string::String>,
    pub description: ::core::option::Option<::aliases::string::String>,
    pub category: ::core::option::Option<::aliases::string::String>,
    pub location: ::core::option::Option<::aliases::string::String>,

    #[builder(default)]
    pub timestamps: ::core::ops::Range<::core::option::Option<::aliases::time::Timestamp>>,
}

impl ::core::convert::From<ViewPublishedEventsFilter> for crate::gateways::EventRepositoryViewFilter {
    fn from(value: ViewPublishedEventsFilter) -> Self {
        Self::builder()
            .statuses(::std::vec![
                crate::gateways::EventRepositoryViewFilterEventStatus::Approved,
            ])
            .maybe_name(value.name)
            .maybe_description(value.description)
            .maybe_category(value.category)
            .maybe_location(value.location)
            .timestamps(value.timestamps)
            .build()
    }
}