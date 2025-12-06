use ::axiom::prelude::*;

#[async_trait]
pub trait RemoveEventPostCommentBoundary {
    async fn apply(
        self: ::std::sync::Arc<Self>, request: RemoveEventPostCommentRequest,
    ) -> ::axiom::result::Fallible<RemoveEventPostCommentResponse>;
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi))]
pub struct RemoveEventPostCommentRequest {
    pub token: ::axiom::string::String,

    pub comment_id: ::axiom::string::String,
}

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type RemoveEventPostCommentResponse =
    ::core::result::Result<RemoveEventPostCommentOkResponse, ::std::vec::Vec<RemoveEventPostCommentErrResponse>>;

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type RemoveEventPostCommentOkResponse = ();

#[derive(::core::fmt::Debug, ::core::clone::Clone)]
#[cfg_attr(feature = "serde", derive(::axiom::Erratum))]
#[cfg_attr(feature = "serde", erratum(rename_all = "kebab-case", rename_all_fields = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum RemoveEventPostCommentErrResponse {
    #[error("Invalid authentication token")]
    AuthenticationTokenInvalid,

    #[error("Authentication token expired")]
    AuthenticationTokenExpired,

    #[error("User not found")]
    UserNotFound,

    #[error("User with role `{user_role}` not authorized: must be {}", super::fmt::join_with_comma_ad_hoc(.allowed_user_roles))]
    UserUnauthorized {
        user_role: RemoveEventPostCommentUserRole,
        allowed_user_roles: ::std::vec::Vec<RemoveEventPostCommentUserRole>,
    },

    #[error("User temporarily suspended")]
    UserSuspended,

    #[error("Comment not found")]
    CommentNotFound,

    #[error("Comment not owned by user")]
    OwnershipMismatch,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::strum::Display)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum RemoveEventPostCommentUserRole {
    Volunteer,
    EventManager,
    Administrator,
}

impl ::core::convert::From<::domain::UserRole> for RemoveEventPostCommentUserRole {
    fn from(value: ::domain::UserRole) -> Self {
        match value {
            ::domain::UserRole::Volunteer => Self::Volunteer,
            ::domain::UserRole::EventManager => Self::EventManager,
            ::domain::UserRole::Administrator => Self::Administrator,
        }
    }
}

impl ::core::convert::From<RemoveEventPostCommentUserRole> for ::domain::UserRole {
    fn from(value: RemoveEventPostCommentUserRole) -> Self {
        match value {
            RemoveEventPostCommentUserRole::Volunteer => Self::Volunteer,
            RemoveEventPostCommentUserRole::EventManager => Self::EventManager,
            RemoveEventPostCommentUserRole::Administrator => Self::Administrator,
        }
    }
}
