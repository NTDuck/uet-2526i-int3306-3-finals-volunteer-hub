use ::axiom::prelude::*;

#[async_trait]
pub trait RemoveEventPostReactionBoundary {
    async fn apply(
        self: ::std::sync::Arc<Self>, request: RemoveEventPostReactionRequest,
    ) -> ::axiom::result::Fallible<RemoveEventPostReactionResponse>;
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi))]
pub struct RemoveEventPostReactionRequest {
    pub token: ::axiom::string::String,
    
    pub reaction_or_post_id: ::axiom::string::String,
}

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type RemoveEventPostReactionResponse =
    ::core::result::Result<RemoveEventPostReactionOkResponse, ::std::vec::Vec<RemoveEventPostReactionErrResponse>>;

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type RemoveEventPostReactionOkResponse = ();

#[derive(::core::fmt::Debug, ::core::clone::Clone)]
#[cfg_attr(feature = "serde", derive(::axiom::Erratum))]
#[cfg_attr(feature = "serde", erratum(rename_all = "kebab-case", rename_all_fields = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum RemoveEventPostReactionErrResponse {
    #[error("Invalid authentication token")]
    AuthenticationTokenInvalid,

    #[error("Authentication token expired")]
    AuthenticationTokenExpired,

    #[error("User not found")]
    UserNotFound,

    #[error("User with role `{user_role}` not authorized: must be {}", super::utils::fmt::join_with_comma_ad_hoc(.allowed_user_roles))]
    UserUnauthorized {
        user_role: RemoveEventPostReactionUserRole,
        allowed_user_roles: ::std::vec::Vec<RemoveEventPostReactionUserRole>,
    },

    #[error("User temporarily suspended")]
    UserSuspended,

    #[error("Post not found")]
    PostNotFound,

    #[error("Reaction not found")]
    ReactionNotFound,

    #[error("Reaction not owned by user")]
    OwnershipMismatch,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::strum::Display)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum RemoveEventPostReactionUserRole {
    Volunteer,
    EventManager,
    Administrator,
}

impl ::core::convert::From<::domain::UserRole> for RemoveEventPostReactionUserRole {
    fn from(value: ::domain::UserRole) -> Self {
        match value {
            ::domain::UserRole::Volunteer => Self::Volunteer,
            ::domain::UserRole::EventManager => Self::EventManager,
            ::domain::UserRole::Administrator => Self::Administrator,
        }
    }
}

impl ::core::convert::From<RemoveEventPostReactionUserRole> for ::domain::UserRole {
    fn from(value: RemoveEventPostReactionUserRole) -> Self {
        match value {
            RemoveEventPostReactionUserRole::Volunteer => Self::Volunteer,
            RemoveEventPostReactionUserRole::EventManager => Self::EventManager,
            RemoveEventPostReactionUserRole::Administrator => Self::Administrator,
        }
    }
}
