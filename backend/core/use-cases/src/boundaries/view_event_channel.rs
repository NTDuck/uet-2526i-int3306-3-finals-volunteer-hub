use ::axiom::prelude::*;

#[async_trait]
pub trait ViewEventChannelBoundary {
    async fn apply(
        self: ::std::sync::Arc<Self>, request: ViewEventChannelRequest,
    ) -> ::axiom::result::Fallible<ViewEventChannelResponse>;
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi))]
pub struct ViewEventChannelRequest {
    pub token: ::axiom::string::String,

    pub event_id: ::axiom::string::String,
}

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type ViewEventChannelResponse =
    ::core::result::Result<ViewEventChannelOkResponse, ::std::vec::Vec<ViewEventChannelErrResponse>>;

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub struct ViewEventChannelOkResponse {
    pub posts: ::std::vec::Vec<ViewEventChannelEventPost>,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub struct ViewEventChannelEventPost {
    pub id: ::axiom::string::String,

    pub last_updated_at: ::axiom::string::String,
    pub title: ::axiom::string::String,
    pub content: ::axiom::string::String,
    
    #[builder(required)]
    pub image_url: ::core::option::Option<::axiom::string::String>,

    pub reaction_count: ::core::primitive::u64,
    pub comment_count: ::core::primitive::u64,

    pub author: ::core::option::Option<ViewEventChannelUser>,
    pub is_reacted_by_actor: bool,
    pub comments_by_actor: ::std::vec::Vec<ViewEventChannelEventPostComment>,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub struct ViewEventChannelEventPostComment {
    pub id: ::axiom::string::String,

    pub last_updated_at: ::axiom::string::String,

    #[builder(required)]
    pub content: ::core::option::Option<::axiom::string::String>,
    #[builder(required)]
    pub image_url: ::core::option::Option<::axiom::string::String>,

    pub author: ::core::option::Option<ViewEventChannelUser>,
}

#[::bon::bon]
impl ViewEventChannelEventPostComment {
    #[builder(finish_fn(name = try_build))]
    pub async fn build_from(
        #[builder(start_fn)] comment: ::domain::EventPostComment,
        #[builder(start_fn)] author: ::core::option::Option<::domain::User>,
        #[builder(setters(name = with_uuid_codec))] uuid_codec: ::std::sync::Arc<
            dyn crate::gateways::UuidCodec + ::core::marker::Send + ::core::marker::Sync,
        >,
        #[builder(setters(name = with_timestamp_codec))] timestamp_codec: ::std::sync::Arc<
            dyn crate::gateways::TimestampCodec + ::core::marker::Send + ::core::marker::Sync,
        >,
    ) -> ::axiom::result::Fallible<Self> {
        Self::builder()
            .id(::std::sync::Arc::clone(&uuid_codec).format(comment.id).await?)
            .last_updated_at(
                ::std::sync::Arc::clone(&timestamp_codec)
                    .format(comment.last_updated_at)
                    .await?,
            )
            .content(comment.content.map(::core::convert::Into::into))
            .image_url(comment.image_url)
            .maybe_author(
                author
                    .map_async(|author| async move {
                        ViewEventChannelUser::build_from(author)
                            .with_uuid_codec(::std::sync::Arc::clone(&uuid_codec))
                            .try_build()
                            .await
                    })
                    .await
                    .transpose()?,
            )
            .build()
            .into_ok()
    }
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub struct ViewEventChannelUser {
    pub id: ::axiom::string::String,
    pub username: ::axiom::string::String,
}

#[::bon::bon]
impl ViewEventChannelUser {
    #[builder(finish_fn(name = try_build))]
    pub async fn build_from(
        #[builder(start_fn)] user: ::domain::User,
        #[builder(setters(name = with_uuid_codec))] uuid_codec: ::std::sync::Arc<
            dyn crate::gateways::UuidCodec + ::core::marker::Send + ::core::marker::Sync,
        >,
    ) -> ::axiom::result::Fallible<Self> {
        Self::builder()
            .id(::std::sync::Arc::clone(&uuid_codec).format(user.id).await?)
            .username(user.username)
            .build()
            .into_ok()
    }
}

#[derive(::core::fmt::Debug, ::core::clone::Clone)]
#[cfg_attr(feature = "serde", derive(::axiom::Erratum))]
#[cfg_attr(feature = "serde", erratum(rename_all = "kebab-case", rename_all_fields = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum ViewEventChannelErrResponse {
    #[error("Invalid authentication token")]
    AuthenticationTokenInvalid,

    #[error("Authentication token expired")]
    AuthenticationTokenExpired,

    #[error("User not found")]
    UserNotFound,

    #[error("User with role `{user_role}` not authorized: must be {}", super::fmt::join_with_comma_ad_hoc(.allowed_user_roles))]
    UserUnauthorized {
        user_role: ViewEventChannelUserRole,
        allowed_user_roles: ::std::vec::Vec<ViewEventChannelUserRole>,
    },

    #[error("Event channel not found")]
    EventChannelNotFound,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::strum::Display)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum ViewEventChannelUserRole {
    Volunteer,
    EventManager,
    Administrator,
}

impl ::core::convert::From<::domain::UserRole> for ViewEventChannelUserRole {
    fn from(value: ::domain::UserRole) -> Self {
        match value {
            ::domain::UserRole::Volunteer => Self::Volunteer,
            ::domain::UserRole::EventManager => Self::EventManager,
            ::domain::UserRole::Administrator => Self::Administrator,
        }
    }
}

impl ::core::convert::From<ViewEventChannelUserRole> for ::domain::UserRole {
    fn from(value: ViewEventChannelUserRole) -> Self {
        match value {
            ViewEventChannelUserRole::Volunteer => Self::Volunteer,
            ViewEventChannelUserRole::EventManager => Self::EventManager,
            ViewEventChannelUserRole::Administrator => Self::Administrator,
        }
    }
}
