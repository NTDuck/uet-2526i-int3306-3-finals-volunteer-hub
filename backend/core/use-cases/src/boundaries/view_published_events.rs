use ::async_trait::async_trait;

#[async_trait]
pub trait ViewPublishedEventsBoundary {
    async fn apply(self: ::std::sync::Arc<Self>, request: ViewPublishedEventsRequest)
        -> ::axiom::result::Fallible<ViewPublishedEventsResponse>;
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi))]
pub struct ViewPublishedEventsRequest {
    pub token: ::axiom::string::String,
    pub filter: ViewPublishedEventsFilter,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi))]
pub struct ViewPublishedEventsFilter {
    pub name: ::core::option::Option<::axiom::string::String>,
    pub description: ::core::option::Option<::axiom::string::String>,
    pub category: ::core::option::Option<::axiom::string::String>,
    pub location: ::core::option::Option<::axiom::string::String>,

    #[builder(default)]
    pub timestamps: ::core::ops::Range<::core::option::Option<::axiom::time::Timestamp>>,
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

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type ViewPublishedEventsResponse = ::core::result::Result<ViewPublishedEventsOkResponse, ::std::vec::Vec<ViewPublishedEventsErrResponse>>;

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

    pub name: ::axiom::string::String,
    pub categories: ::std::vec::Vec<::axiom::string::String>,
    pub location: ::axiom::string::String,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::strum::Display)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum ViewPublishedEventsEventStatus {
    Created,
    Approved,
    Rejected,
}

impl ::core::convert::From<::domain::EventStatus> for ViewPublishedEventsEventStatus {
    fn from(value: ::domain::EventStatus) -> Self {
        match value {
            ::domain::EventStatus::Created { .. } => Self::Created,
            ::domain::EventStatus::Approved { .. } => Self::Approved,
            ::domain::EventStatus::Rejected { .. } => Self::Rejected,
        }
    }
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy)]
#[cfg_attr(feature = "serde", derive(::axiom::Erratum))]
#[cfg_attr(feature = "serde", erratum(rename_all = "kebab-case", rename_all_fields = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum ViewPublishedEventsErrResponse {
    #[error("Invalid or expired authentication token")]
    AuthenticationTokenInvalid,

    #[error("User with role `{user_role}` not authorized: must be `{expected_user_role}`", expected_user_role = ViewPublishedEventsUserRole::Volunteer)]
    UserUnauthorized {
        user_role: ViewPublishedEventsUserRole,
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
