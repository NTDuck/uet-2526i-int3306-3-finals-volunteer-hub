use ::axiom::prelude::*;

#[async_trait]
pub trait CreateEventPostBoundary {
    async fn apply(
        self: ::std::sync::Arc<Self>, request: CreateEventPostRequest,
    ) -> ::axiom::result::Fallible<CreateEventPostResponse>;
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi))]
pub struct CreateEventPostRequest {
    pub token: ::axiom::string::String,

    pub event_id: ::axiom::string::String,

    pub post_title: ::axiom::string::String,
    pub post_content: ::axiom::string::String,

    pub post_image: ::core::option::Option<::std::boxed::Box<[u8]>>,
}

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type CreateEventPostResponse =
    ::core::result::Result<CreateEventPostOkResponse, ::std::vec::Vec<CreateEventPostErrResponse>>;

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type CreateEventPostOkResponse = ();

#[derive(::core::fmt::Debug, ::core::clone::Clone)]
#[cfg_attr(feature = "serde", derive(::axiom::Erratum))]
#[cfg_attr(feature = "serde", erratum(rename_all = "kebab-case", rename_all_fields = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum CreateEventPostErrResponse {
    #[error("Invalid authentication token")]
    AuthenticationTokenInvalid,

    #[error("Authentication token expired")]
    AuthenticationTokenExpired,

    #[error("User not found")]
    UserNotFound,

    #[error("User with role `{user_role}` not authorized: must be {}", super::fmt::join_with_comma_ad_hoc(.allowed_user_roles))]
    UserUnauthorized {
        user_role: CreateEventPostUserRole,
        allowed_user_roles: ::std::vec::Vec<CreateEventPostUserRole>,
    },

    #[error("User temporarily suspended")]
    UserSuspended,

    #[error("Event channel not found")]
    EventChannelNotFound,

    #[error("Invalid post title `{post_title}`: {hint}", hint = ::domain::EventPostTitle::hint())]
    PostTitleInvalid {
        post_title: ::axiom::string::String,
    },

    #[error("Invalid post content `{post_content}`: {hint}", hint = ::domain::EventPostContent::hint())]
    PostContentInvalid {
        post_content: ::axiom::string::String,
    },

    #[error("Invalid post image")]
    PostImageInvalid,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::strum::Display)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum CreateEventPostUserRole {
    Volunteer,
    EventManager,
    Administrator,
}

impl ::core::convert::From<::domain::UserRole> for CreateEventPostUserRole {
    fn from(value: ::domain::UserRole) -> Self {
        match value {
            ::domain::UserRole::Volunteer => Self::Volunteer,
            ::domain::UserRole::EventManager => Self::EventManager,
            ::domain::UserRole::Administrator => Self::Administrator,
        }
    }
}

impl ::core::convert::From<CreateEventPostUserRole> for ::domain::UserRole {
    fn from(value: CreateEventPostUserRole) -> Self {
        match value {
            CreateEventPostUserRole::Volunteer => Self::Volunteer,
            CreateEventPostUserRole::EventManager => Self::EventManager,
            CreateEventPostUserRole::Administrator => Self::Administrator,
        }
    }
}
