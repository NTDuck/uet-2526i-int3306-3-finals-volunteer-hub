use ::async_trait::async_trait;

#[async_trait]
pub trait CreateEventPostReactionBoundary {
    async fn apply(self: ::std::sync::Arc<Self>, request: CreateEventPostReactionRequest) -> ::axiom::result::Fallible<CreateEventPostReactionResponse>;
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi))]
pub struct CreateEventPostReactionRequest {
    pub token: ::axiom::string::String,
    pub post_id: ::axiom::string::String,
}

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type CreateEventPostReactionResponse = ::core::result::Result<CreateEventPostReactionOkResponse, ::std::vec::Vec<CreateEventPostReactionErrResponse>>;

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type CreateEventPostReactionOkResponse = ();

#[derive(::core::fmt::Debug, ::core::clone::Clone)]
#[cfg_attr(feature = "serde", derive(::axiom::Erratum))]
#[cfg_attr(feature = "serde", erratum(rename_all = "kebab-case", rename_all_fields = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum CreateEventPostReactionErrResponse {
	#[error("Invalid or expired authentication token")]
	AuthenticationTokenInvalid,

	#[error("User with role `{user_role}` not authorized: must be `{first_expected_user_role}` or `{second_expected_user_role}`", first_expected_user_role = CreateEventPostReactionUserRole::Volunteer, second_expected_user_role = CreateEventPostReactionUserRole::EventManager)]
	UserUnauthorized {
		user_role: CreateEventPostReactionUserRole,
	},

    #[error("User temporarily suspended")]
    UserSuspended,

    #[error("Post not found")]
    PostNotFound,

    #[error("Reaction already exists")]
    ReactionAlreadyExists,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::strum::Display)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum CreateEventPostReactionUserRole {
    Volunteer,
    EventManager,
    Administrator,
}

impl ::core::convert::From<::domain::UserRole> for CreateEventPostReactionUserRole {
    fn from(value: ::domain::UserRole) -> Self {
        match value {
            ::domain::UserRole::Volunteer => Self::Volunteer,
            ::domain::UserRole::EventManager => Self::EventManager,
            ::domain::UserRole::Administrator => Self::Administrator,
        }
    }
}

impl ::core::convert::From<CreateEventPostReactionUserRole> for ::domain::UserRole {
    fn from(value: CreateEventPostReactionUserRole) -> Self {
        match value {
            CreateEventPostReactionUserRole::Volunteer => Self::Volunteer,
            CreateEventPostReactionUserRole::EventManager => Self::EventManager,
            CreateEventPostReactionUserRole::Administrator => Self::Administrator,
        }
    }
}
