use ::wasm_bindgen::prelude::*;
use ::use_cases::gateways::*;
use ::use_cases::boundaries::*;
use ::use_cases::interactors::*;
use ::infrastructures::*;

#[wasm_bindgen]
#[derive(::bon::Builder)]
pub struct Application {
    #[wasm_bindgen(skip)]
    sign_in_boundary: ::std::sync::Arc<dyn SignInBoundary + ::core::marker::Send + ::core::marker::Sync>,

    #[wasm_bindgen(skip)]
    sign_up_boundary: ::std::sync::Arc<dyn SignUpBoundary + ::core::marker::Send + ::core::marker::Sync>,
}

#[wasm_bindgen]
impl Application {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Gateways::new().unwrap().into()
    }

    #[wasm_bindgen(js_name = signIn)]
    pub async fn sign_in(&self, request: SignInRequest) -> JsPromise<SignInOkResponse> {
        NestedFallibleExt::into_js(::std::sync::Arc::clone(&self.sign_in_boundary).apply(request).await)
    }

    #[wasm_bindgen(js_name = signUp)]
    pub async fn sign_up(&self, request: SignUpRequest) -> JsPromise<SignUpOkResponse> {
        NestedFallibleExt::into_js(::std::sync::Arc::clone(&self.sign_up_boundary).apply(request).await)
    }
}

#[rustfmt::skip]
impl ::core::convert::From<Gateways> for Application {
    fn from(gateways: Gateways) -> Self {
        Application::builder()
            .sign_in_boundary(::std::sync::Arc::new(SignInInteractor::builder()
                .user_repository(::std::sync::Arc::clone(&gateways.user_repository))
                .auth_token_generator(::std::sync::Arc::clone(&gateways.auth_token_generator))
                .password_hasher(::std::sync::Arc::clone(&gateways.password_hasher))
                .build()))
            .sign_up_boundary(::std::sync::Arc::new(SignUpInteractor::builder()
                .user_repository(::std::sync::Arc::clone(&gateways.user_repository))
                .uuid_generator(::std::sync::Arc::clone(&gateways.uuid_generator))
                .password_hasher(::std::sync::Arc::clone(&gateways.password_hasher))
                .build()))
            .build()
    }
}

#[derive(::bon::Builder)]
struct Gateways {
    user_repository: ::std::sync::Arc<dyn UserRepository + ::core::marker::Send + ::core::marker::Sync>,

    uuid_generator: ::std::sync::Arc<dyn UuidGenerator + ::core::marker::Send + ::core::marker::Sync>,

    auth_token_generator: ::std::sync::Arc<dyn AuthenticationTokenGenerator + ::core::marker::Send + ::core::marker::Sync>,

    password_hasher: ::std::sync::Arc<dyn PasswordHasher + ::core::marker::Send + ::core::marker::Sync>,
}

impl Gateways {
    pub fn new() -> ::aliases::result::Fallible<Self> {
        use ::hmac::Mac as _;

        ::dotenvy::from_filename("../../.env")?;

        ::aliases::result::Fallible::Ok(Self::builder()
            .user_repository(::std::sync::Arc::new(InMemoryUserRepository::builder().build()))
            .uuid_generator(::std::sync::Arc::new(UuidV7Generator::builder().build()))
            .auth_token_generator(::std::sync::Arc::new(JsonWebTokenGenerator::builder()
                .key(::hmac::Hmac::<::sha2::Sha256>::new_from_slice(::std::env::var("JWT_SECRET_KEY")?.as_bytes())?)
                .build()))
            .password_hasher(::std::sync::Arc::new(Argon2PasswordHasher::builder()
                .context(::argon2::Argon2::new_with_secret(
                    ::std::boxed::Box::leak(::std::env::var("ARGON2_SECRET_KEY")?.into_boxed_str()).as_bytes(),
                    ::argon2::Algorithm::Argon2id, ::argon2::Version::V0x13,
                    ::argon2::Params::default())?)
                .build()))
            .build())
    }
}

pub type JsPromise<T = ()> = ::core::result::Result<T, ::wasm_bindgen::JsValue>;

#[allow(dead_code)]
pub(crate) trait FallibleExt<T = ()> {
    fn into_js(self) -> JsPromise<T>;
}

impl<T> FallibleExt<T> for ::aliases::result::Fallible<T> {
    fn into_js(self) -> JsPromise<T> {
        self.map_err(|error| ::wasm_bindgen::JsValue::from_str(&error.to_string()))
    }
}

pub(crate) trait NestedFallibleExt<T = ()> {
    fn into_js(self) -> JsPromise<T>;
}

impl<T, E> NestedFallibleExt<T> for ::aliases::result::Fallible<::core::result::Result<T, ::std::vec::Vec<E>>>
where
    E: ::core::error::Error,
{
    fn into_js(self) -> JsPromise<T> {
        self.map_err(|error| ::wasm_bindgen::JsValue::from_str(&error.to_string()))
            .and_then(|inner| inner
                .map_err(|errors|
                    errors.into_iter().map(|error| ::wasm_bindgen::JsValue::from_str(&error.to_string())).collect::<::std::vec::Vec<_>>())
                .map_err(::core::convert::Into::into))
    }
}
