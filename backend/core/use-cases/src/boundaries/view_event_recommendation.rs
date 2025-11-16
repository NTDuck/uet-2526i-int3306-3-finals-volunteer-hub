use ::async_trait::async_trait;

#[async_trait]
pub trait ViewEventRecommendationBoundary {
    async fn apply(self: ::std::sync::Arc<Self>, request: ViewEventRecommendationRequest)
        -> ::aliases::result::Fallible<ViewEventRecommendationResponse>;
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
pub struct ViewEventRecommendationRequest {
    pub token: ::aliases::string::String,
    pub r#type: ViewEventRecommendationRecommendationType,
    pub limit: usize,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
pub enum ViewEventRecommendationRecommendationType {
    RecentlyPublished,
    RecentlyPosted,
    Trending,
}

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type ViewEventRecommendationResponse = ::core::result::Result<ViewEventRecommendationOkResponse, ::std::vec::Vec<ViewEventRecommendationErrResponse>>;

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
pub struct ViewEventRecommendationOkResponse {
    pub events: ::std::vec::Vec<ViewEventRecommendationEvent>,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
pub struct ViewEventRecommendationEvent {
    pub id: ::aliases::string::String,

    pub status: ViewEventRecommendationEventStatus,

    pub name: ::aliases::string::String,
    pub categories: ::std::vec::Vec<::aliases::string::String>,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::strum::Display)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged, rename_all = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
pub enum ViewEventRecommendationEventStatus {
    Created,
    Approved,
    Rejected,
    Completed,
}

impl ::core::convert::From<::domain::EventStatus> for ViewEventRecommendationEventStatus {
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
pub enum ViewEventRecommendationErrResponse {
    #[error("Invalid or expired authentication token")]
    AuthenticationTokenInvalid,
}
