use ::async_trait::async_trait;

#[async_trait]
pub trait SignInBoundary {
    async fn apply(self: ::std::sync::Arc<Self>, request: SignInRequest) -> ::aliases::result::Fallible<SignInResponse>;
}

#[derive(::core::fmt::Debug, ::core::clone::Clone)]
#[derive(::bon::Builder)]
#[builder(on(::aliases::string::String, into))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
pub struct SignInRequest {
    pub username_or_email: ::aliases::string::String,
    pub password: ::aliases::string::String,
}

pub struct SignInResponse {

}

pub mod models {

}
