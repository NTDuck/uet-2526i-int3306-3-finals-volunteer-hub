use ::async_trait::async_trait;

#[async_trait]
pub trait RemoveEventPostBoundary {
    async fn apply(
        self: ::std::sync::Arc<Self>, request: RemoveEventPostRequest,
    ) -> ::axiom::result::Fallible<RemoveEventPostResponse>;
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi))]
pub struct RemoveEventPostRequest {
    pub token: ::axiom::string::String,
    pub post_id: ::axiom::string::String,
}

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type RemoveEventPostResponse =
    ::core::result::Result<RemoveEventPostOkResponse, ::std::vec::Vec<RemoveEventPostErrResponse>>;

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type RemoveEventPostOkResponse = ();

#[derive(::core::fmt::Debug, ::core::clone::Clone)]
#[cfg_attr(feature = "serde", derive(::axiom::Erratum))]
#[cfg_attr(feature = "serde", erratum(rename_all = "kebab-case", rename_all_fields = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum RemoveEventPostErrResponse {
    #[error("Invalid authentication token")]
    AuthenticationTokenInvalid,

    #[error("Authentication token expired")]
    AuthenticationTokenExpired,

    #[error("User not found")]
    UserNotFound,

    #[error("User with role `{user_role}` not authorized: must be `{}` or `{}`", RemoveEventPostUserRole::Volunteer, RemoveEventPostUserRole::EventManager)]
    UserUnauthorized {
        user_role: RemoveEventPostUserRole,
    },

    #[error("User temporarily suspended")]
    UserSuspended,

    #[error("Post not found")]
    PostNotFound,

    #[error("Post not owned by user")]
    OwnershipMismatch,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::strum::Display)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum RemoveEventPostUserRole {
    Volunteer,
    EventManager,
    Administrator,
}

impl ::core::convert::From<::domain::UserRole> for RemoveEventPostUserRole {
    fn from(value: ::domain::UserRole) -> Self {
        match value {
            ::domain::UserRole::Volunteer => Self::Volunteer,
            ::domain::UserRole::EventManager => Self::EventManager,
            ::domain::UserRole::Administrator => Self::Administrator,
        }
    }
}

impl ::core::convert::From<RemoveEventPostUserRole> for ::domain::UserRole {
    fn from(value: RemoveEventPostUserRole) -> Self {
        match value {
            RemoveEventPostUserRole::Volunteer => Self::Volunteer,
            RemoveEventPostUserRole::EventManager => Self::EventManager,
            RemoveEventPostUserRole::Administrator => Self::Administrator,
        }
    }
}
