use ::async_trait::async_trait;

#[async_trait]
pub trait UpdateEventPostBoundary {
	async fn apply(self: ::std::sync::Arc<Self>, request: UpdateEventPostRequest) -> ::axiom::result::Fallible<UpdateEventPostResponse>;
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi))]
pub struct UpdateEventPostRequest {
	pub token: ::axiom::string::String,

    pub post_id: ::axiom::string::String,

	pub post_title: ::core::option::Option<::axiom::string::String>,
	pub post_content: ::core::option::Option<::axiom::string::String>,
}

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type UpdateEventPostResponse = ::core::result::Result<UpdateEventPostOkResponse, ::std::vec::Vec<UpdateEventPostErrResponse>>;

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type UpdateEventPostOkResponse = ();

#[derive(::core::fmt::Debug, ::core::clone::Clone)]
#[cfg_attr(feature = "serde", derive(::axiom::Erratum))]
#[cfg_attr(feature = "serde", erratum(rename_all = "kebab-case", rename_all_fields = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum UpdateEventPostErrResponse {
	#[error("Invalid or expired authentication token")]
	AuthenticationTokenInvalid,

	#[error("User with role `{user_role}` not authorized: must be `{first_expected_user_role}` or `{second_expected_user_role}`", first_expected_user_role = UpdateEventPostUserRole::Volunteer, second_expected_user_role = UpdateEventPostUserRole::EventManager)]
	UserUnauthorized {
		user_role: UpdateEventPostUserRole,
	},

	#[error("Invalid post title `{post_title}`: {hint}", hint = ::domain::EventPostTitle::hint())]
	PostTitleInvalid {
		post_title: ::axiom::string::String,
	},

	#[error("Invalid post content `{post_content}`: {hint}", hint = ::domain::EventPostContent::hint())]
    PostContentInvalid {
        post_content: ::axiom::string::String,
    },

	#[error("Post not found")]
	PostNotFound,

    #[error("Post not owned by user")]
    OwnershipMismatch,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::strum::Display)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum UpdateEventPostUserRole {
    Volunteer,
    EventManager,
    Administrator,
}

impl ::core::convert::From<::domain::UserRole> for UpdateEventPostUserRole {
    fn from(value: ::domain::UserRole) -> Self {
        match value {
            ::domain::UserRole::Volunteer => Self::Volunteer,
            ::domain::UserRole::EventManager => Self::EventManager,
            ::domain::UserRole::Administrator => Self::Administrator,
        }
    }
}

impl ::core::convert::From<UpdateEventPostUserRole> for ::domain::UserRole {
    fn from(value: UpdateEventPostUserRole) -> Self {
        match value {
            UpdateEventPostUserRole::Volunteer => Self::Volunteer,
            UpdateEventPostUserRole::EventManager => Self::EventManager,
            UpdateEventPostUserRole::Administrator => Self::Administrator,
        }
    }
}
