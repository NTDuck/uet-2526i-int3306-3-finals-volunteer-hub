use ::axiom::prelude::*;

#[async_trait]
pub trait ViewEventsBoundary {
    async fn apply(
        self: ::std::sync::Arc<Self>, request: ViewEventsRequest,
    ) -> ::axiom::result::Fallible<ViewEventsResponse>;
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi))]
pub struct ViewEventsRequest {
    pub token: ::axiom::string::String,

    pub filter: ::core::option::Option<ViewEventsFilter>,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi))]
pub struct ViewEventsFilter {
    pub query: ::core::option::Option<::axiom::string::String>,

    pub statuses: ::core::option::Option<::std::vec::Vec<ViewEventsEventStatus>>,
    pub start_timestamp: ::core::option::Option<::axiom::string::String>,
    pub end_timestamp: ::core::option::Option<::axiom::string::String>,
}

#[::bon::bon]
impl ViewEventsFilter {
    #[builder(finish_fn(name = try_build))]
    pub async fn build_into(
        self,
        #[builder(setters(name = with_timestamp_codec))] timestamp_codec: ::std::sync::Arc<
            dyn crate::gateways::TimestampCodec + ::core::marker::Send + ::core::marker::Sync,
        >,
    ) -> ::axiom::result::Fallible<crate::gateways::EventRepositorySearchFilter> {
        crate::gateways::EventRepositorySearchFilter::builder()
            .maybe_query(self.query)
            .maybe_statuses(self.statuses.map(|statuses| {
                statuses
                    .into_iter()
                    .map(::core::convert::Into::into)
                    .collect::<::std::vec::Vec<_>>()
            }))
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
pub type ViewEventsResponse = ::core::result::Result<ViewEventsOkResponse, ::std::vec::Vec<ViewEventsErrResponse>>;

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub struct ViewEventsOkResponse {
    pub events: ::std::vec::Vec<ViewEventsEvent>,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub struct ViewEventsEvent {
    pub id: ::axiom::string::String,

    pub status: ViewEventsEventStatus,

    pub name: ::axiom::string::String,
    #[builder(with = |values: ::std::vec::Vec<impl ::core::convert::Into<::axiom::string::String>>| values.into_iter().map(::core::convert::Into::into).collect())]
    pub categories: ::std::vec::Vec<::axiom::string::String>,
    pub location: ::axiom::string::String,
}

#[::bon::bon]
impl ViewEventsEvent {
    #[builder(finish_fn(name = try_build))]
    pub async fn build_from(
        #[builder(start_fn)] event: ::domain::Event,
        #[builder(setters(name = with_uuid_codec))] uuid_codec: ::std::sync::Arc<
            dyn crate::gateways::UuidCodec + ::core::marker::Send + ::core::marker::Sync,
        >,
    ) -> ::axiom::result::Fallible<Self> {
        Self::builder()
            .id(uuid_codec.format(event.id).await?)
            .status(*event.statuses.last())
            .name(event.name)
            .categories(event.categories)
            .location(event.location)
            .build()
            .into_ok()
    }
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::strum::Display)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
pub enum ViewEventsEventStatus {
    Created,
    Updated,
    Approved,
    Rejected,
}

impl ::core::convert::From<::domain::EventStatus> for ViewEventsEventStatus {
    fn from(value: ::domain::EventStatus) -> Self {
        match value {
            ::domain::EventStatus::Created { .. } => Self::Created,
            ::domain::EventStatus::Updated { .. } => Self::Updated,
            ::domain::EventStatus::Approved { .. } => Self::Approved,
            ::domain::EventStatus::Rejected { .. } => Self::Rejected,
        }
    }
}

impl ::core::convert::From<ViewEventsEventStatus> for crate::gateways::EventRepositorySearchFilterEventStatus {
    fn from(value: ViewEventsEventStatus) -> Self {
        match value {
            ViewEventsEventStatus::Created => Self::Created,
            ViewEventsEventStatus::Updated => Self::Updated,
            ViewEventsEventStatus::Approved => Self::Approved,
            ViewEventsEventStatus::Rejected => Self::Rejected,
        }
    }
}

impl ::core::convert::From<crate::gateways::EventRepositorySearchFilterEventStatus> for ViewEventsEventStatus {
    fn from(value: crate::gateways::EventRepositorySearchFilterEventStatus) -> Self {
        match value {
            crate::gateways::EventRepositorySearchFilterEventStatus::Created => Self::Created,
            crate::gateways::EventRepositorySearchFilterEventStatus::Updated => Self::Updated,
            crate::gateways::EventRepositorySearchFilterEventStatus::Approved => Self::Approved,
            crate::gateways::EventRepositorySearchFilterEventStatus::Rejected => Self::Rejected,
        }
    }
}

#[derive(::core::fmt::Debug, ::core::clone::Clone)]
#[cfg_attr(feature = "serde", derive(::axiom::Erratum))]
#[cfg_attr(feature = "serde", erratum(rename_all = "kebab-case", rename_all_fields = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum ViewEventsErrResponse {
    #[error("Invalid authentication token")]
    AuthenticationTokenInvalid,

    #[error("Authentication token expired")]
    AuthenticationTokenExpired,

    #[error("User not found")]
    UserNotFound,

    #[error("User with role `{user_role}` not authorized: must be {}", super::utils::fmt::join_with_comma_ad_hoc(.allowed_user_roles))]
    UserUnauthorized {
        user_role: ViewEventsUserRole,
        allowed_user_roles: ::std::vec::Vec<ViewEventsUserRole>,
    },
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::strum::Display)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum ViewEventsUserRole {
    Volunteer,
    EventManager,
    Administrator,
}

impl ::core::convert::From<::domain::UserRole> for ViewEventsUserRole {
    fn from(value: ::domain::UserRole) -> Self {
        match value {
            ::domain::UserRole::Volunteer => Self::Volunteer,
            ::domain::UserRole::EventManager => Self::EventManager,
            ::domain::UserRole::Administrator => Self::Administrator,
        }
    }
}

impl ::core::convert::From<ViewEventsUserRole> for ::domain::UserRole {
    fn from(value: ViewEventsUserRole) -> Self {
        match value {
            ViewEventsUserRole::Volunteer => Self::Volunteer,
            ViewEventsUserRole::EventManager => Self::EventManager,
            ViewEventsUserRole::Administrator => Self::Administrator,
        }
    }
}
