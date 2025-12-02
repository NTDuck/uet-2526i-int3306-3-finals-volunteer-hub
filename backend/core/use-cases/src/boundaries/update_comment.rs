use ::axiom::prelude::*;

#[async_trait]
pub trait UpdateEventPostCommentBoundary {
    async fn apply(
        self: ::std::sync::Arc<Self>, request: UpdateEventPostCommentRequest,
    ) -> ::axiom::result::Fallible<UpdateEventPostCommentResponse>;
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi))]
pub struct UpdateEventPostCommentRequest {
    pub token: ::axiom::string::String,

    pub comment_id: ::axiom::string::String,
    pub comment_content: ::core::option::Option<::axiom::string::String>,
}

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type UpdateEventPostCommentResponse =
    ::core::result::Result<UpdateEventPostCommentOkResponse, ::std::vec::Vec<UpdateEventPostCommentErrResponse>>;

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type UpdateEventPostCommentOkResponse = ();

#[derive(::core::fmt::Debug, ::core::clone::Clone)]
#[cfg_attr(feature = "serde", derive(::axiom::Erratum))]
#[cfg_attr(feature = "serde", erratum(rename_all = "kebab-case", rename_all_fields = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum UpdateEventPostCommentErrResponse {
    #[error("Invalid authentication token")]
    AuthenticationTokenInvalid,

    #[error("Authentication token expired")]
    AuthenticationTokenExpired,

    #[error("User not found")]
    UserNotFound,

    #[error("User with role `{user_role}` not authorized: must be {}", super::utils::fmt::join_with_comma_ad_hoc(.allowed_user_roles))]
    UserUnauthorized {
        user_role: UpdateEventPostCommentUserRole,
        allowed_user_roles: ::std::vec::Vec<UpdateEventPostCommentUserRole>,
    },

    #[error("User temporarily suspended")]
    UserSuspended,

    #[error("Comment not found")]
    CommentNotFound,

    #[error("Invalid comment content `{comment_content}`: {hint}", hint = ::domain::EventPostCommentContent::hint())]
    CommentContentInvalid {
        comment_content: ::axiom::string::String,
    },

    #[error("Comment not owned by user")]
    OwnershipMismatch,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::strum::Display)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum UpdateEventPostCommentUserRole {
    Volunteer,
    EventManager,
    Administrator,
}

impl ::core::convert::From<::domain::UserRole> for UpdateEventPostCommentUserRole {
    fn from(value: ::domain::UserRole) -> Self {
        match value {
            ::domain::UserRole::Volunteer => Self::Volunteer,
            ::domain::UserRole::EventManager => Self::EventManager,
            ::domain::UserRole::Administrator => Self::Administrator,
        }
    }
}

impl ::core::convert::From<UpdateEventPostCommentUserRole> for ::domain::UserRole {
    fn from(value: UpdateEventPostCommentUserRole) -> Self {
        match value {
            UpdateEventPostCommentUserRole::Volunteer => Self::Volunteer,
            UpdateEventPostCommentUserRole::EventManager => Self::EventManager,
            UpdateEventPostCommentUserRole::Administrator => Self::Administrator,
        }
    }
}
