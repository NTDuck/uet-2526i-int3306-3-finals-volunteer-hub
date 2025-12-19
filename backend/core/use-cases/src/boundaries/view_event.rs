use ::axiom::prelude::*;
use ::futures::prelude::*;

#[async_trait]
pub trait ViewEventBoundary {
    async fn apply(
        self: ::std::sync::Arc<Self>, request: ViewEventRequest,
    ) -> ::axiom::result::Fallible<ViewEventResponse>;
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi))]
pub struct ViewEventRequest {
    pub token: ::axiom::string::String,

    pub event_id: ::axiom::string::String,
}

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type ViewEventResponse = ::core::result::Result<ViewEventOkResponse, ::std::vec::Vec<ViewEventErrResponse>>;

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub struct ViewEventOkResponse {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub event: ViewEventEvent,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub struct ViewEventEvent {
    pub id: ::axiom::string::String,

    pub statuses: ::std::vec::Vec<ViewEventEventStatus>,

    pub name: ::axiom::string::String,
    pub categories: ::std::vec::Vec<::axiom::string::String>,
    pub location: ::axiom::string::String,

    #[builder(required)]
    pub image_url: ::core::option::Option<::axiom::string::String>,
}

#[::bon::bon]
impl ViewEventEvent {
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
            .statuses(
                event
                    .statuses
                    .into_iter()
                    .into_stream()
                    .then(|status| {
                        let uuid_codec = ::std::sync::Arc::clone(&uuid_codec);
                        let timestamp_codec = ::std::sync::Arc::clone(&timestamp_codec);

                        async move {
                            ViewEventEventStatus::build_from(status)
                                .with_uuid_codec(::std::sync::Arc::clone(&uuid_codec))
                                .with_timestamp_codec(::std::sync::Arc::clone(&timestamp_codec))
                                .try_build()
                                .await
                        }
                    })
                    .try_collect::<::std::vec::Vec<_>>()
                    .await?,
            )
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

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::strum::Display)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum ViewEventEventStatus {
    Created {
        created_by_manager_id: ::axiom::string::String,
        created_at: ::axiom::string::String,
    },
    Updated {
        updated_by_manager_id: ::axiom::string::String,
        updated_at: ::axiom::string::String,
    },
    Approved {
        approved_by_administrator_id: ::axiom::string::String,
        approved_at: ::axiom::string::String,
    },
    Rejected {
        rejected_by_administrator_id: ::axiom::string::String,
        rejected_at: ::axiom::string::String,
    },
}

#[::bon::bon]
impl ViewEventEventStatus {
    #[builder(finish_fn(name = try_build))]
    pub async fn build_from(
        #[builder(start_fn)] event_status: ::domain::EventStatus,
        #[builder(setters(name = with_uuid_codec))] uuid_codec: ::std::sync::Arc<
            dyn crate::gateways::UuidCodec + ::core::marker::Send + ::core::marker::Sync,
        >,
        #[builder(setters(name = with_timestamp_codec))] timestamp_codec: ::std::sync::Arc<
            dyn crate::gateways::TimestampCodec + ::core::marker::Send + ::core::marker::Sync,
        >,
    ) -> ::axiom::result::Fallible<Self> {
        match event_status {
            ::domain::EventStatus::Created { created_by_manager_id, created_at } => Self::Created {
                created_by_manager_id: ::std::sync::Arc::clone(&uuid_codec).format(created_by_manager_id).await?,
                created_at: ::std::sync::Arc::clone(&timestamp_codec).format(created_at).await?,
            }
            .into_ok(),
            ::domain::EventStatus::Updated { updated_by_manager_id, updated_at } => Self::Updated {
                updated_by_manager_id: ::std::sync::Arc::clone(&uuid_codec).format(updated_by_manager_id).await?,
                updated_at: ::std::sync::Arc::clone(&timestamp_codec).format(updated_at).await?,
            }
            .into_ok(),
            ::domain::EventStatus::Approved {
                approved_by_administrator_id,
                approved_at,
            } => Self::Approved {
                approved_by_administrator_id: ::std::sync::Arc::clone(&uuid_codec)
                    .format(approved_by_administrator_id)
                    .await?,
                approved_at: ::std::sync::Arc::clone(&timestamp_codec).format(approved_at).await?,
            }
            .into_ok(),
            ::domain::EventStatus::Rejected {
                rejected_by_administrator_id,
                rejected_at,
            } => Self::Rejected {
                rejected_by_administrator_id: ::std::sync::Arc::clone(&uuid_codec)
                    .format(rejected_by_administrator_id)
                    .await?,
                rejected_at: ::std::sync::Arc::clone(&timestamp_codec).format(rejected_at).await?,
            }
            .into_ok(),
        }
    }
}

#[derive(::core::fmt::Debug, ::core::clone::Clone)]
#[cfg_attr(feature = "serde", derive(::axiom::Erratum))]
#[cfg_attr(feature = "serde", erratum(rename_all = "kebab-case", rename_all_fields = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum ViewEventErrResponse {
    #[error("Invalid authentication token")]
    AuthenticationTokenInvalid,

    #[error("Authentication token expired")]
    AuthenticationTokenExpired,

    #[error("User not found")]
    UserNotFound,

    #[error("User with role `{user_role}` not authorized: must be {}", super::fmt::join_with_comma_ad_hoc(.allowed_user_roles))]
    UserUnauthorized {
        user_role: ViewEventUserRole,
        allowed_user_roles: ::std::vec::Vec<ViewEventUserRole>,
    },

    #[error("Event not found")]
    EventNotFound,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::strum::Display)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum ViewEventUserRole {
    Volunteer,
    EventManager,
    Administrator,
}

impl ::core::convert::From<::domain::UserRole> for ViewEventUserRole {
    fn from(value: ::domain::UserRole) -> Self {
        match value {
            ::domain::UserRole::Volunteer => Self::Volunteer,
            ::domain::UserRole::EventManager => Self::EventManager,
            ::domain::UserRole::Administrator => Self::Administrator,
        }
    }
}

impl ::core::convert::From<ViewEventUserRole> for ::domain::UserRole {
    fn from(value: ViewEventUserRole) -> Self {
        match value {
            ViewEventUserRole::Volunteer => Self::Volunteer,
            ViewEventUserRole::EventManager => Self::EventManager,
            ViewEventUserRole::Administrator => Self::Administrator,
        }
    }
}
