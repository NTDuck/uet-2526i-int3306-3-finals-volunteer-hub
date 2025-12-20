use ::axiom::prelude::*;

#[async_trait]
pub trait ViewEventPostBoundary {
    async fn apply(
        self: ::std::sync::Arc<Self>, request: ViewEventPostRequest,
    ) -> ::axiom::result::Fallible<ViewEventPostResponse>;
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi))]
pub struct ViewEventPostRequest {
    pub token: ::axiom::string::String,

    pub post_id: ::axiom::string::String,
}

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type ViewEventPostResponse =
    ::core::result::Result<ViewEventPostOkResponse, ::std::vec::Vec<ViewEventPostErrResponse>>;

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub struct ViewEventPostOkResponse {
    pub id: ::axiom::string::String,

    pub last_updated_at: ::axiom::string::String,
    pub title: ::axiom::string::String,
    pub content: ::axiom::string::String,

    #[builder(required)]
    pub image_url: ::core::option::Option<::axiom::string::String>,

    pub reactions: ::std::vec::Vec<ViewEventPostEventPostReaction>,
    pub comments: ::std::vec::Vec<ViewEventPostEventPostComment>,

    pub author: ::core::option::Option<ViewEventPostUser>,
    pub is_reacted_by_actor: bool,
    pub is_authored_by_actor: bool,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub struct ViewEventPostEventPostReaction {
    pub id: ::axiom::string::String,

    pub author: ::core::option::Option<ViewEventPostUser>,
}

#[::bon::bon]
impl ViewEventPostEventPostReaction {
    #[builder(finish_fn(name = try_build))]
    pub async fn build_from(
        #[builder(start_fn)] reaction: ::domain::EventPostReaction,
        #[builder(start_fn)] author: ::core::option::Option<::domain::User>,
        #[builder(setters(name = with_uuid_codec))] uuid_codec: ::std::sync::Arc<
            dyn crate::gateways::UuidCodec + ::core::marker::Send + ::core::marker::Sync,
        >,
    ) -> ::axiom::result::Fallible<Self> {
        Self::builder()
            .id(::std::sync::Arc::clone(&uuid_codec).format(reaction.id).await?)
            .maybe_author(
                author
                    .map_async(|author| async move {
                        ViewEventPostUser::build_from(author)
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
pub struct ViewEventPostEventPostComment {
    pub id: ::axiom::string::String,

    pub last_updated_at: ::axiom::string::String,

    #[builder(required)]
    pub content: ::core::option::Option<::axiom::string::String>,
    #[builder(required)]
    pub image_url: ::core::option::Option<::axiom::string::String>,

    pub author: ::core::option::Option<ViewEventPostUser>,

    pub is_authored_by_actor: bool,
}

#[::bon::bon]
impl ViewEventPostEventPostComment {
    #[builder(finish_fn(name = try_build))]
    pub async fn build_from(
        #[builder(start_fn)] comment: ::domain::EventPostComment,
        #[builder(start_fn)] author: ::core::option::Option<::domain::User>,
        #[builder(start_fn)] actor_id: ::domain::Uuid,
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
                        ViewEventPostUser::build_from(author)
                            .with_uuid_codec(::std::sync::Arc::clone(&uuid_codec))
                            .try_build()
                            .await
                    })
                    .await
                    .transpose()?,
            )
            .is_authored_by_actor(comment.author_id == actor_id)
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
pub struct ViewEventPostUser {
    pub id: ::axiom::string::String,
    pub username: ::axiom::string::String,
}

#[::bon::bon]
impl ViewEventPostUser {
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
pub enum ViewEventPostErrResponse {
    #[error("Invalid authentication token")]
    AuthenticationTokenInvalid,

    #[error("Authentication token expired")]
    AuthenticationTokenExpired,

    #[error("User not found")]
    UserNotFound,

    #[error("User with role `{user_role}` not authorized: must be {}", super::fmt::join_with_comma_ad_hoc(.allowed_user_roles))]
    UserUnauthorized {
        user_role: ViewEventPostUserRole,
        allowed_user_roles: ::std::vec::Vec<ViewEventPostUserRole>,
    },

    #[error("Post not found")]
    PostNotFound,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::strum::Display)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum ViewEventPostUserRole {
    Volunteer,
    EventManager,
    Administrator,
}

impl ::core::convert::From<::domain::UserRole> for ViewEventPostUserRole {
    fn from(value: ::domain::UserRole) -> Self {
        match value {
            ::domain::UserRole::Volunteer => Self::Volunteer,
            ::domain::UserRole::EventManager => Self::EventManager,
            ::domain::UserRole::Administrator => Self::Administrator,
        }
    }
}

impl ::core::convert::From<ViewEventPostUserRole> for ::domain::UserRole {
    fn from(value: ViewEventPostUserRole) -> Self {
        match value {
            ViewEventPostUserRole::Volunteer => Self::Volunteer,
            ViewEventPostUserRole::EventManager => Self::EventManager,
            ViewEventPostUserRole::Administrator => Self::Administrator,
        }
    }
}
