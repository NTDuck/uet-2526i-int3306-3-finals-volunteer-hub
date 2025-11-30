use ::axiom::prelude::*;

#[async_trait]
pub trait ViewEventHistoryBoundary {
    async fn apply(
        self: ::std::sync::Arc<Self>, request: ViewEventHistoryRequest,
    ) -> ::axiom::result::Fallible<ViewEventHistoryResponse>;
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi))]
pub struct ViewEventHistoryRequest {
    pub token: ::axiom::string::String,
}

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type ViewEventHistoryResponse =
    ::core::result::Result<ViewEventHistoryOkResponse, ::std::vec::Vec<ViewEventHistoryErrResponse>>;

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub struct ViewEventHistoryOkResponse {
    pub events: ::std::vec::Vec<ViewEventHistoryEvent>,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub struct ViewEventHistoryEvent {
    pub id: ::axiom::string::String,

    pub status: ViewEventHistoryEventStatus,
    pub registration_status: ViewEventHistoryEventRegistrationStatus,

    pub name: ::axiom::string::String,
    #[builder(with = |values: ::std::vec::Vec<impl ::core::convert::Into<::axiom::string::String>>| values.into_iter().map(::core::convert::Into::into).collect())]
    pub categories: ::std::vec::Vec<::axiom::string::String>,
    pub location: ::axiom::string::String,
}

#[::bon::bon]
impl ViewEventHistoryEvent {
    #[builder(finish_fn(name = try_build))]
    pub async fn build_from(#[builder(start_fn)] event: ::domain::Event, #[builder(start_fn)] event_registration_status: ::domain::EventRegistrationStatus, #[builder(setters(name = with_uuid_codec))] uuid_codec: ::std::sync::Arc<dyn crate::gateways::UuidCodec + ::core::marker::Send + ::core::marker::Sync>) -> ::axiom::result::Fallible<Self> {
        Self::builder()
            .id(uuid_codec.format(event.id).await?)
            .status(*event.statuses.last())
            .registration_status(event_registration_status)
            .name(event.name)
            .categories(event.categories)
            .location(event.location)
            .build()
            .into_ok()
    }
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::strum::Display)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum ViewEventHistoryEventStatus {
    Created,
    Updated,
    Approved,
    Rejected,
}

impl ::core::convert::From<::domain::EventStatus> for ViewEventHistoryEventStatus {
    fn from(value: ::domain::EventStatus) -> Self {
        match value {
            ::domain::EventStatus::Created { .. } => Self::Created,
            ::domain::EventStatus::Updated { .. } => Self::Updated,
            ::domain::EventStatus::Approved { .. } => Self::Approved,
            ::domain::EventStatus::Rejected { .. } => Self::Rejected,
        }
    }
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::strum::Display)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum ViewEventHistoryEventRegistrationStatus {
    Pending,
    Withdrawn,
    Accepted,
    Declined,
    Completed,
}

impl ::core::convert::From<::domain::EventRegistrationStatus> for ViewEventHistoryEventRegistrationStatus {
    fn from(value: ::domain::EventRegistrationStatus) -> Self {
        match value {
            ::domain::EventRegistrationStatus::Pending { .. } => Self::Pending,
            ::domain::EventRegistrationStatus::Withdrawn { .. } => Self::Withdrawn,
            ::domain::EventRegistrationStatus::Accepted { .. } => Self::Accepted,
            ::domain::EventRegistrationStatus::Declined { .. } => Self::Declined,
            ::domain::EventRegistrationStatus::Completed { .. } => Self::Completed,
        }
    }
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy)]
#[cfg_attr(feature = "serde", derive(::axiom::Erratum))]
#[cfg_attr(feature = "serde", erratum(rename_all = "kebab-case", rename_all_fields = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum ViewEventHistoryErrResponse {
    #[error("Invalid authentication token")]
    AuthenticationTokenInvalid,

    #[error("Authentication token expired")]
    AuthenticationTokenExpired,

    #[error("User not found")]
    UserNotFound,

    #[error("User with role `{user_role}` not authorized: must be `{expected_user_role}`", expected_user_role = ViewEventHistoryUserRole::Volunteer)]
    UserUnauthorized {
        user_role: ViewEventHistoryUserRole,
    },
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::strum::Display)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum ViewEventHistoryUserRole {
    Volunteer,
    EventManager,
    Administrator,
}

impl ::core::convert::From<::domain::UserRole> for ViewEventHistoryUserRole {
    fn from(value: ::domain::UserRole) -> Self {
        match value {
            ::domain::UserRole::Volunteer => Self::Volunteer,
            ::domain::UserRole::EventManager => Self::EventManager,
            ::domain::UserRole::Administrator => Self::Administrator,
        }
    }
}

impl ::core::convert::From<ViewEventHistoryUserRole> for ::domain::UserRole {
    fn from(value: ViewEventHistoryUserRole) -> Self {
        match value {
            ViewEventHistoryUserRole::Volunteer => Self::Volunteer,
            ViewEventHistoryUserRole::EventManager => Self::EventManager,
            ViewEventHistoryUserRole::Administrator => Self::Administrator,
        }
    }
}
