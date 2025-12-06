use ::axiom::prelude::*;

#[async_trait]
pub trait ViewPublishedEventsBoundary {
    async fn apply(
        self: ::std::sync::Arc<Self>, request: ViewPublishedEventsRequest,
    ) -> ::axiom::result::Fallible<ViewPublishedEventsResponse>;
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi))]
pub struct ViewPublishedEventsRequest {
    pub token: ::axiom::string::String,
    pub filter: ::core::option::Option<ViewPublishedEventsFilter>,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi))]
pub struct ViewPublishedEventsFilter {
    pub query: ::core::option::Option<::axiom::string::String>,

    pub start_timestamp: ::core::option::Option<::axiom::string::String>,
    pub end_timestamp: ::core::option::Option<::axiom::string::String>,
}

#[::bon::bon]
impl ViewPublishedEventsFilter {
    #[builder(finish_fn(name = try_build))]
    pub async fn build_into(
        self,
        #[builder(setters(name = with_timestamp_codec))] timestamp_codec: ::std::sync::Arc<
            dyn crate::gateways::TimestampCodec + ::core::marker::Send + ::core::marker::Sync,
        >,
    ) -> ::axiom::result::Fallible<crate::gateways::EventRepositorySearchFilter> {
        crate::gateways::EventRepositorySearchFilter::builder()
            .maybe_query(self.query)
            .statuses([crate::gateways::EventRepositorySearchFilterEventStatus::Approved])
            .timestamps(::core::ops::Range {
                start: self
                    .start_timestamp
                    .map_async(|timestamp| {
                        let timestamp_codec = ::std::sync::Arc::clone(&timestamp_codec);
                        async move { timestamp_codec.parse(timestamp).await }
                    })
                    .await
                    .transpose()?,
                end: self
                    .end_timestamp
                    .map_async(|timestamp| {
                        let timestamp_codec = ::std::sync::Arc::clone(&timestamp_codec);
                        async move { timestamp_codec.parse(timestamp).await }
                    })
                    .await
                    .transpose()?,
            })
            .build()
            .into_ok()
    }
}

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type ViewPublishedEventsResponse =
    ::core::result::Result<ViewPublishedEventsOkResponse, ::std::vec::Vec<ViewPublishedEventsErrResponse>>;

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub struct ViewPublishedEventsOkResponse {
    pub events: ::std::vec::Vec<ViewPublishedEventsEvent>,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub struct ViewPublishedEventsEvent {
    pub id: ::axiom::string::String,

    pub status: ViewPublishedEventsEventStatus,
    pub status_last_updated_at: ::axiom::string::String,

    pub name: ::axiom::string::String,
    pub categories: ::std::vec::Vec<::axiom::string::String>,
    pub location: ::axiom::string::String,

    #[builder(required)]
    pub image_url: ::core::option::Option<::axiom::string::String>,
}

#[::bon::bon]
impl ViewPublishedEventsEvent {
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
        let event_status = *event.statuses.last();

        Self::builder()
            .id(::std::sync::Arc::clone(&uuid_codec).format(event.id).await?)
            .status(event_status)
            .status_last_updated_at(::std::sync::Arc::clone(&timestamp_codec).format(event_status.at()).await?)
            .name(event.name)
            .categories(
                event
                    .categories
                    .into_iter()
                    .map(::core::convert::Into::into)
                    .collect::<::std::vec::Vec<_>>(),
            )
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
pub enum ViewPublishedEventsEventStatus {
    Created,
    Updated,
    Approved,
    Rejected,
}

impl ::core::convert::From<::domain::EventStatus> for ViewPublishedEventsEventStatus {
    fn from(value: ::domain::EventStatus) -> Self {
        match value {
            ::domain::EventStatus::Created { .. } => Self::Created,
            ::domain::EventStatus::Updated { .. } => Self::Updated,
            ::domain::EventStatus::Approved { .. } => Self::Approved,
            ::domain::EventStatus::Rejected { .. } => Self::Rejected,
        }
    }
}

#[derive(::core::fmt::Debug, ::core::clone::Clone)]
#[cfg_attr(feature = "serde", derive(::axiom::Erratum))]
#[cfg_attr(feature = "serde", erratum(rename_all = "kebab-case", rename_all_fields = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum ViewPublishedEventsErrResponse {
    #[error("Invalid authentication token")]
    AuthenticationTokenInvalid,

    #[error("Authentication token expired")]
    AuthenticationTokenExpired,

    #[error("User not found")]
    UserNotFound,

    #[error("User with role `{user_role}` not authorized: must be {}", super::fmt::join_with_comma_ad_hoc(.allowed_user_roles))]
    UserUnauthorized {
        user_role: ViewPublishedEventsUserRole,
        allowed_user_roles: ::std::vec::Vec<ViewPublishedEventsUserRole>,
    },
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::strum::Display)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum ViewPublishedEventsUserRole {
    Volunteer,
    EventManager,
    Administrator,
}

impl ::core::convert::From<::domain::UserRole> for ViewPublishedEventsUserRole {
    fn from(value: ::domain::UserRole) -> Self {
        match value {
            ::domain::UserRole::Volunteer => Self::Volunteer,
            ::domain::UserRole::EventManager => Self::EventManager,
            ::domain::UserRole::Administrator => Self::Administrator,
        }
    }
}

impl ::core::convert::From<ViewPublishedEventsUserRole> for ::domain::UserRole {
    fn from(value: ViewPublishedEventsUserRole) -> Self {
        match value {
            ViewPublishedEventsUserRole::Volunteer => Self::Volunteer,
            ViewPublishedEventsUserRole::EventManager => Self::EventManager,
            ViewPublishedEventsUserRole::Administrator => Self::Administrator,
        }
    }
}
