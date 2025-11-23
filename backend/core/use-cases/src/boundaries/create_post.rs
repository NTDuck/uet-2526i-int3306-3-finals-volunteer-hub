use ::async_trait::async_trait;

#[async_trait]
pub trait CreateEventPostBoundary {
	async fn apply(self: ::std::sync::Arc<Self>, request: CreateEventPostRequest) -> ::axiom::result::Fallible<CreateEventPostResponse>;
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi))]
pub struct CreateEventPostRequest {
	pub token: ::axiom::string::String,

	pub post_title: ::axiom::string::String,
	pub post_content: ::axiom::string::String,
}

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type CreateEventPostResponse = ::core::result::Result<CreateEventPostOkResponse, ::std::vec::Vec<CreateEventPostErrResponse>>;

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type CreateEventPostOkResponse = ();

#[derive(::core::fmt::Debug, ::core::clone::Clone)]
#[cfg_attr(feature = "serde", derive(::axiom::Erratum))]
#[cfg_attr(feature = "serde", erratum(rename_all = "kebab-case", rename_all_fields = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum CreateEventPostErrResponse {
	#[error("Invalid or expired authentication token")]
	AuthenticationTokenInvalid,

	#[error("User with role `{user_role}` not authorized: must be `{first_expected_user_role}` or `{second_expected_user_role}`", first_expected_user_role = CreateEventPostUserRole::Volunteer, second_expected_user_role = CreateEventPostUserRole::EventManager)]
	UserUnauthorized {
		user_role: CreateEventPostUserRole,
	},

	#[error("Invalid post title `{post_title}`; {hint}", hint = ::domain::EventPostTitle::hint())]
	PostTitleInvalid {
		post_title: ::axiom::string::String,
	},

	#[error("Invalid post ")]

	#[error("Post not found")]
	PostNotFound,
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
