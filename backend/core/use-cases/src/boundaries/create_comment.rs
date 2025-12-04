use ::axiom::prelude::*;

#[async_trait]
pub trait CreateEventPostCommentBoundary {
    async fn apply(
        self: ::std::sync::Arc<Self>, request: CreateEventPostCommentRequest,
    ) -> ::axiom::result::Fallible<CreateEventPostCommentResponse>;
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi))]
pub struct CreateEventPostCommentRequest {
    pub token: ::axiom::string::String,

    pub post_id: ::axiom::string::String,

    pub comment_content: ::core::option::Option<::axiom::string::String>,
    pub comment_image: ::core::option::Option<::std::boxed::Box<[u8]>>,
}

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type CreateEventPostCommentResponse =
    ::core::result::Result<CreateEventPostCommentOkResponse, ::std::vec::Vec<CreateEventPostCommentErrResponse>>;

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type CreateEventPostCommentOkResponse = ();

#[derive(::core::fmt::Debug, ::core::clone::Clone)]
#[cfg_attr(feature = "serde", derive(::axiom::Erratum))]
#[cfg_attr(feature = "serde", erratum(rename_all = "kebab-case", rename_all_fields = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum CreateEventPostCommentErrResponse {
    #[error("Invalid authentication token")]
    AuthenticationTokenInvalid,

    #[error("Authentication token expired")]
    AuthenticationTokenExpired,

    #[error("User not found")]
    UserNotFound,

    #[error("User with role `{user_role}` not authorized: must be {}", super::fmt::join_with_comma_ad_hoc(.allowed_user_roles))]
    UserUnauthorized {
        user_role: CreateEventPostCommentUserRole,
        allowed_user_roles: ::std::vec::Vec<CreateEventPostCommentUserRole>,
    },

    #[error("User temporarily suspended")]
    UserSuspended,

    #[error("Post not found")]
    PostNotFound,

    #[error("Invalid comment content `{comment_content}`: {hint}", hint = ::domain::EventPostCommentContent::hint())]
    CommentContentInvalid {
        comment_content: ::axiom::string::String,
    },

    #[error("Invalid comment image")]
    CommentImageInvalid,

    #[error("Either `comment_content` or `comment_image` must be provided")]
    MissingRequiredFields,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::strum::Display)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum CreateEventPostCommentUserRole {
    Volunteer,
    EventManager,
    Administrator,
}

impl ::core::convert::From<::domain::UserRole> for CreateEventPostCommentUserRole {
    fn from(value: ::domain::UserRole) -> Self {
        match value {
            ::domain::UserRole::Volunteer => Self::Volunteer,
            ::domain::UserRole::EventManager => Self::EventManager,
            ::domain::UserRole::Administrator => Self::Administrator,
        }
    }
}

impl ::core::convert::From<CreateEventPostCommentUserRole> for ::domain::UserRole {
    fn from(value: CreateEventPostCommentUserRole) -> Self {
        match value {
            CreateEventPostCommentUserRole::Volunteer => Self::Volunteer,
            CreateEventPostCommentUserRole::EventManager => Self::EventManager,
            CreateEventPostCommentUserRole::Administrator => Self::Administrator,
        }
    }
}
