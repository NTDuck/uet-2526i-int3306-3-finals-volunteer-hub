use ::axiom::prelude::*;
use ::futures::prelude::*;

#[async_trait]
pub trait ViewUserBoundary {
    async fn apply(
        self: ::std::sync::Arc<Self>, request: ViewUserRequest,
    ) -> ::axiom::result::Fallible<ViewUserResponse>;
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi))]
pub struct ViewUserRequest {
    pub token: ::axiom::string::String,

    pub user_id: ::axiom::string::String,
}

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type ViewUserResponse = ::core::result::Result<ViewUserOkResponse, ::std::vec::Vec<ViewUserErrResponse>>;

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub struct ViewUserOkResponse {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub user: ViewUserUser,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub struct ViewUserUser {
    pub id: ::axiom::string::String,

    pub role: ViewUserUserRole,
    pub statuses: ::std::vec::Vec<ViewUserUserStatus>,

    pub username: ::axiom::string::String,
    pub email: ::axiom::string::String,
    pub full_name: ::axiom::string::String,

    #[builder(required)]
    pub avatar_url: ::core::option::Option<::axiom::string::String>,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::strum::Display)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
pub enum ViewUserUserRole {
    Volunteer,
    EventManager,
    Administrator,
}

impl ::core::convert::From<::domain::UserRole> for ViewUserUserRole {
    fn from(value: ::domain::UserRole) -> Self {
        match value {
            ::domain::UserRole::Volunteer => Self::Volunteer,
            ::domain::UserRole::EventManager => Self::EventManager,
            ::domain::UserRole::Administrator => Self::Administrator,
        }
    }
}

impl ::core::convert::From<ViewUserUserRole> for ::domain::UserRole {
    fn from(value: ViewUserUserRole) -> Self {
        match value {
            ViewUserUserRole::Volunteer => Self::Volunteer,
            ViewUserUserRole::EventManager => Self::EventManager,
            ViewUserUserRole::Administrator => Self::Administrator,
        }
    }
}

impl ::core::convert::From<crate::gateways::UserRepositoryViewFilterUserRole> for ViewUserUserRole {
    fn from(value: crate::gateways::UserRepositoryViewFilterUserRole) -> Self {
        match value {
            crate::gateways::UserRepositoryViewFilterUserRole::Volunteer => Self::Volunteer,
            crate::gateways::UserRepositoryViewFilterUserRole::EventManager => Self::EventManager,
            crate::gateways::UserRepositoryViewFilterUserRole::Administrator => Self::Administrator,
        }
    }
}

impl ::core::convert::From<ViewUserUserRole> for crate::gateways::UserRepositoryViewFilterUserRole {
    fn from(value: ViewUserUserRole) -> Self {
        match value {
            ViewUserUserRole::Volunteer => Self::Volunteer,
            ViewUserUserRole::EventManager => Self::EventManager,
            ViewUserUserRole::Administrator => Self::Administrator,
        }
    }
}

#[derive(::core::fmt::Debug, ::core::clone::Clone)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
pub enum ViewUserUserStatus {
    Created {
        created_at: ::axiom::string::String,
    },
    Updated {
        updated_at: ::axiom::string::String,
    },
    Suspended {
        suspended_by_administrator_id: ::axiom::string::String,
        suspended_at: ::axiom::string::String,
    },
    Unsuspended {
        unsuspended_by_administrator_id: ::axiom::string::String,
        unsuspended_at: ::axiom::string::String,
    },
}

#[::bon::bon]
impl ViewUserUserStatus {
    #[builder(finish_fn(name = try_build))]
    pub async fn build_from(
        #[builder(start_fn)] status: ::domain::UserStatus,
        #[builder(setters(name = with_uuid_codec))] uuid_codec: ::std::sync::Arc<
            dyn crate::gateways::UuidCodec + ::core::marker::Send + ::core::marker::Sync,
        >,
        #[builder(setters(name = with_timestamp_codec))] timestamp_codec: ::std::sync::Arc<
            dyn crate::gateways::TimestampCodec + ::core::marker::Send + ::core::marker::Sync,
        >,
    ) -> ::axiom::result::Fallible<Self> {
        match status {
            ::domain::UserStatus::Created { created_at } => ViewUserUserStatus::Created {
                created_at: ::std::sync::Arc::clone(&timestamp_codec).format(created_at).await?,
            }
            .into_ok(),
            ::domain::UserStatus::Updated { updated_at } => ViewUserUserStatus::Updated {
                updated_at: ::std::sync::Arc::clone(&timestamp_codec).format(updated_at).await?,
            }
            .into_ok(),
            ::domain::UserStatus::Suspended {
                suspended_by_administrator_id,
                suspended_at,
            } => ViewUserUserStatus::Suspended {
                suspended_by_administrator_id: ::std::sync::Arc::clone(&uuid_codec)
                    .format(suspended_by_administrator_id)
                    .await?,
                suspended_at: ::std::sync::Arc::clone(&timestamp_codec).format(suspended_at).await?,
            }
            .into_ok(),
            ::domain::UserStatus::Unsuspended {
                unsuspended_by_administrator_id,
                unsuspended_at,
            } => ViewUserUserStatus::Unsuspended {
                unsuspended_by_administrator_id: ::std::sync::Arc::clone(&uuid_codec)
                    .format(unsuspended_by_administrator_id)
                    .await?,
                unsuspended_at: ::std::sync::Arc::clone(&timestamp_codec).format(unsuspended_at).await?,
            }
            .into_ok(),
        }
    }
}

#[::bon::bon]
impl ViewUserUser {
    #[builder(finish_fn(name = try_build))]
    pub async fn build_from(
        #[builder(start_fn)] user: ::domain::User,
        #[builder(setters(name = with_uuid_codec))] uuid_codec: ::std::sync::Arc<
            dyn crate::gateways::UuidCodec + ::core::marker::Send + ::core::marker::Sync,
        >,
        #[builder(setters(name = with_timestamp_codec))] timestamp_codec: ::std::sync::Arc<
            dyn crate::gateways::TimestampCodec + ::core::marker::Send + ::core::marker::Sync,
        >,
    ) -> ::axiom::result::Fallible<Self> {
        Self::builder()
            .id(::std::sync::Arc::clone(&uuid_codec).format(user.id).await?)
            .role(user.role)
            .statuses(
                user.statuses
                    .into_iter()
                    .into_stream()
                    .then(|status| {
                        let uuid_codec = ::std::sync::Arc::clone(&uuid_codec);
                        let timestamp_codec = ::std::sync::Arc::clone(&timestamp_codec);

                        async move {
                            ViewUserUserStatus::build_from(status)
                                .with_uuid_codec(::std::sync::Arc::clone(&uuid_codec))
                                .with_timestamp_codec(::std::sync::Arc::clone(&timestamp_codec))
                                .try_build()
                                .await
                        }
                    })
                    .try_collect::<::std::vec::Vec<_>>()
                    .await?,
            )
            .username(user.username)
            .email(user.email)
            .full_name(user.full_name)
            .avatar_url(user.avatar_url)
            .build()
            .into_ok()
    }
}

#[derive(::core::fmt::Debug, ::core::clone::Clone)]
#[cfg_attr(feature = "serde", derive(::axiom::Erratum))]
#[cfg_attr(feature = "serde", erratum(rename_all = "kebab-case", rename_all_fields = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum ViewUserErrResponse {
    #[error("Invalid authentication token")]
    AuthenticationTokenInvalid,

    #[error("Authentication token expired")]
    AuthenticationTokenExpired,

    #[error("User not found")]
    UserNotFound,

    #[error("User with role `{user_role}` not authorized: must be {}", super::fmt::join_with_comma_ad_hoc(.allowed_user_roles))]
    UserUnauthorized {
        user_role: ViewUserUserRole,
        allowed_user_roles: ::std::vec::Vec<ViewUserUserRole>,
    },
}
