use ::axiom::prelude::*;

#[async_trait]
pub trait ViewEventRecommendationBoundary {
    async fn apply(
        self: ::std::sync::Arc<Self>, request: ViewEventRecommendationRequest,
    ) -> ::axiom::result::Fallible<ViewEventRecommendationResponse>;
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi))]
pub struct ViewEventRecommendationRequest {
    pub token: ::axiom::string::String,

    pub r#type: ViewEventRecommendationRecommendationType,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy)]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi))]
pub enum ViewEventRecommendationRecommendationType {
    RecentlyPublished,
    RecentlyPosted,
    Trending,
    Personalized,
}

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type ViewEventRecommendationResponse =
    ::core::result::Result<ViewEventRecommendationOkResponse, ::std::vec::Vec<ViewEventRecommendationErrResponse>>;

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub struct ViewEventRecommendationOkResponse {
    pub events: ::std::vec::Vec<ViewEventRecommendationEvent>,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub struct ViewEventRecommendationEvent {
    pub id: ::axiom::string::String,

    pub status: ViewEventRecommendationEventStatus,
    pub status_last_updated_at: ::axiom::string::String,

    pub name: ::axiom::string::String,
    pub categories: ::std::vec::Vec<::axiom::string::String>,
    pub location: ::axiom::string::String,
    #[builder(required)]
    pub image_url: ::core::option::Option<::axiom::string::String>,
}

#[::bon::bon]
impl ViewEventRecommendationEvent {
    #[builder(finish_fn(name = try_build))]
    pub async fn build_from(
        #[builder(start_fn)] event: ::domain::Event,
        #[builder(setters(name = with_uuid_codec))] uuid_codec: ::std::sync::Arc<
            dyn crate::gateways::UuidCodec + ::core::marker::Send + ::core::marker::Sync,
        >,
        #[builder(setters(name = with_timestamp_codec))] timestamp_codec: ::std::sync::Arc<
            dyn crate::gateways::TimestampCodec + ::core::marker::Send + ::core::marker::Sync,
        >,
    ) -> ::axiom::result::Fallible<Self> {
        Self::builder()
            .id(::std::sync::Arc::clone(&uuid_codec).format(event.id).await?)
            .status(*event.statuses.last())
            .status_last_updated_at(::std::sync::Arc::clone(&timestamp_codec).format(event.statuses.last().at()).await?)
            .name(event.name)
            .categories(event.categories.into_iter().map(::core::convert::Into::into).collect::<::std::vec::Vec<_>>())
            .location(event.location)
            .image_url(event.image_url)
            .build()
            .into_ok()
    }
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::strum::Display)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum ViewEventRecommendationEventStatus {
    Created,
    Updated,
    Approved,
    Rejected,
}

impl ::core::convert::From<::domain::EventStatus> for ViewEventRecommendationEventStatus {
    fn from(value: ::domain::EventStatus) -> Self {
        match value {
            ::domain::EventStatus::Created { .. } => Self::Created,
            ::domain::EventStatus::Updated { .. } => Self::Updated,
            ::domain::EventStatus::Approved { .. } => Self::Approved,
            ::domain::EventStatus::Rejected { .. } => Self::Rejected,
        }
    }
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::thiserror::Error)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum ViewEventRecommendationErrResponse {
    #[error("Invalid authentication token")]
    AuthenticationTokenInvalid,

    #[error("Authentication token expired")]
    AuthenticationTokenExpired,

    #[error("User not found")]
    UserNotFound,
}
