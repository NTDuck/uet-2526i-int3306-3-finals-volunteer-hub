use ::axiom::prelude::*;

#[async_trait]
pub trait ViewUsersBoundary {
    async fn apply(
        self: ::std::sync::Arc<Self>, request: ViewUsersRequest,
    ) -> ::axiom::result::Fallible<ViewUsersResponse>;
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi))]
pub struct ViewUsersRequest {
    pub token: ::axiom::string::String,
    pub filter: ::core::option::Option<ViewUsersFilter>,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi))]
pub struct ViewUsersFilter {
    pub query: ::core::option::Option<::axiom::string::String>,

    pub roles: ::core::option::Option<::std::vec::Vec<ViewUsersUserRole>>,
    pub statuses: ::core::option::Option<::std::vec::Vec<ViewUsersUserStatus>>,
}

impl ::core::convert::From<ViewUsersFilter> for crate::gateways::UserRepositorySearchFilter {
    fn from(value: ViewUsersFilter) -> Self {
        Self::builder()
            .maybe_query(value.query)
            .maybe_roles(value.roles.map(|roles| {
                roles
                    .into_iter()
                    .map(::core::convert::Into::into)
                    .collect::<::std::vec::Vec<_>>()
            }))
            .maybe_statuses(value.statuses.map(|statuses| {
                statuses
                    .into_iter()
                    .map(::core::convert::Into::into)
                    .collect::<::std::vec::Vec<_>>()
            }))
            .build()
    }
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::strum::Display)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
pub enum ViewUsersUserRole {
    Volunteer,
    EventManager,
    Administrator,
}

impl ::core::convert::From<::domain::UserRole> for ViewUsersUserRole {
    fn from(value: ::domain::UserRole) -> Self {
        match value {
            ::domain::UserRole::Volunteer => Self::Volunteer,
            ::domain::UserRole::EventManager => Self::EventManager,
            ::domain::UserRole::Administrator => Self::Administrator,
        }
    }
}

impl ::core::convert::From<ViewUsersUserRole> for ::domain::UserRole {
    fn from(value: ViewUsersUserRole) -> Self {
        match value {
            ViewUsersUserRole::Volunteer => Self::Volunteer,
            ViewUsersUserRole::EventManager => Self::EventManager,
            ViewUsersUserRole::Administrator => Self::Administrator,
        }
    }
}

impl ::core::convert::From<crate::gateways::UserRepositoryViewFilterUserRole> for ViewUsersUserRole {
    fn from(value: crate::gateways::UserRepositoryViewFilterUserRole) -> Self {
        match value {
            crate::gateways::UserRepositoryViewFilterUserRole::Volunteer => Self::Volunteer,
            crate::gateways::UserRepositoryViewFilterUserRole::EventManager => Self::EventManager,
            crate::gateways::UserRepositoryViewFilterUserRole::Administrator => Self::Administrator,
        }
    }
}

impl ::core::convert::From<ViewUsersUserRole> for crate::gateways::UserRepositoryViewFilterUserRole {
    fn from(value: ViewUsersUserRole) -> Self {
        match value {
            ViewUsersUserRole::Volunteer => Self::Volunteer,
            ViewUsersUserRole::EventManager => Self::EventManager,
            ViewUsersUserRole::Administrator => Self::Administrator,
        }
    }
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
pub enum ViewUsersUserStatus {
    Created,
    Suspended,
    Unsuspended,
}

impl ::core::convert::From<::domain::UserStatus> for ViewUsersUserStatus {
    fn from(value: ::domain::UserStatus) -> Self {
        match value {
            ::domain::UserStatus::Created => Self::Created,
            ::domain::UserStatus::Suspended { .. } => Self::Suspended,
            ::domain::UserStatus::Unsuspended { .. } => Self::Unsuspended,
        }
    }
}

impl ::core::convert::From<crate::gateways::UserRepositoryViewFilterUserStatus> for ViewUsersUserStatus {
    fn from(value: crate::gateways::UserRepositoryViewFilterUserStatus) -> Self {
        match value {
            crate::gateways::UserRepositoryViewFilterUserStatus::Created => Self::Created,
            crate::gateways::UserRepositoryViewFilterUserStatus::Suspended => Self::Suspended,
            crate::gateways::UserRepositoryViewFilterUserStatus::Unsuspended => Self::Unsuspended,
        }
    }
}

impl ::core::convert::From<ViewUsersUserStatus> for crate::gateways::UserRepositoryViewFilterUserStatus {
    fn from(value: ViewUsersUserStatus) -> Self {
        match value {
            ViewUsersUserStatus::Created => Self::Created,
            ViewUsersUserStatus::Suspended => Self::Suspended,
            ViewUsersUserStatus::Unsuspended => Self::Unsuspended,
        }
    }
}

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type ViewUsersResponse = ::core::result::Result<ViewUsersOkResponse, ::std::vec::Vec<ViewUsersErrResponse>>;

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub struct ViewUsersOkResponse {
    pub users: ::std::vec::Vec<ViewUsersUser>,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub struct ViewUsersUser {
    pub id: ::axiom::string::String,

    pub role: ViewUsersUserRole,
    pub status: ViewUsersUserStatus,

    pub username: ::axiom::string::String,
    pub email: ::axiom::string::String,
    pub full_name: ::axiom::string::String,
}

#[::bon::bon]
impl ViewUsersUser {
    #[builder(finish_fn(name = try_build))]
    pub async fn build_from(
        #[builder(start_fn)] user: ::domain::User,
        #[builder(setters(name = with_uuid_codec))] uuid_codec: ::std::sync::Arc<
            dyn crate::gateways::UuidCodec + ::core::marker::Send + ::core::marker::Sync,
        >,
    ) -> ::axiom::result::Fallible<Self> {
        Self::builder()
            .id(uuid_codec.format(user.id).await?)
            .role(user.role)
            .status(*user.statuses.last())
            .username(user.username)
            .email(user.email)
            .full_name(user.full_name)
            .build()
            .into_ok()
    }
}

#[derive(::core::fmt::Debug, ::core::clone::Clone)]
#[cfg_attr(feature = "serde", derive(::axiom::Erratum))]
#[cfg_attr(feature = "serde", erratum(rename_all = "kebab-case", rename_all_fields = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum ViewUsersErrResponse {
    #[error("Invalid authentication token")]
    AuthenticationTokenInvalid,

    #[error("Authentication token expired")]
    AuthenticationTokenExpired,

    #[error("User not found")]
    UserNotFound,

    #[error("User with role `{user_role}` not authorized: must be {}", super::utils::fmt::join_with_comma_ad_hoc(.allowed_user_roles))]
    UserUnauthorized {
        user_role: ViewUsersUserRole,
        allowed_user_roles: ::std::vec::Vec<ViewUsersUserRole>,
    },
}
