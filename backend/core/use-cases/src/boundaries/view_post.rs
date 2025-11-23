use ::async_trait::async_trait;

#[async_trait]
pub trait ViewEventPostBoundary {
	async fn apply(self: ::std::sync::Arc<Self>, request: ViewEventPostRequest) -> ::axiom::result::Fallible<ViewEventPostResponse>;
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi))]
pub struct ViewEventPostRequest {
	pub token: ::axiom::string::String,
	pub event_post_id: ::axiom::string::String,
}

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type ViewEventPostResponse = ::core::result::Result<ViewEventPostOkResponse, ::std::vec::Vec<ViewEventPostErrResponse>>;

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub struct ViewEventPostOkResponse {
	pub id: ::axiom::string::String,
	pub title: ::axiom::string::String,
	pub content: ::axiom::string::String,

	pub reactions: ::std::vec::Vec<ViewEventPostEventPostReaction>,
	pub comments: ::std::vec::Vec<ViewEventPostEventPostComment>,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub struct ViewEventPostEventPostReaction {
	pub volunteer: ViewEventPostVolunteer,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub struct ViewEventPostEventPostComment {
	pub volunteer: ViewEventPostVolunteer,
	pub content: ::axiom::string::String,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub struct ViewEventPostVolunteer {
    pub id: ::axiom::string::String,
    pub username: ::axiom::string::String,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy)]
#[cfg_attr(feature = "serde", derive(::axiom::Erratum))]
#[cfg_attr(feature = "serde", erratum(rename_all = "kebab-case", rename_all_fields = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum ViewEventPostErrResponse {
	#[error("Invalid or expired authentication token")]
	AuthenticationTokenInvalid,

	#[error("User with role `{user_role}` not authorized: must be `{first_expected_user_role}` or `{second_expected_user_role}`", first_expected_user_role = ViewEventPostUserRole::Volunteer, second_expected_user_role = ViewEventPostUserRole::EventManager)]
	UserUnauthorized {
		user_role: ViewEventPostUserRole,
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
