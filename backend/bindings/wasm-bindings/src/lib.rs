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
    pub async fn sign_in(&self, request: SignInRequest) -> NestedJsPromise<SignInOkResponse> {
        NestedFallibleExt::into_js(::std::sync::Arc::clone(&self.sign_in_boundary).apply(request).await)
    }

    #[wasm_bindgen(js_name = signUp)]
    pub async fn sign_up(&self, request: SignUpRequest) -> NestedJsPromise<SignUpOkResponse> {
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

        ::aliases::result::Fallible::Ok(Self::builder()
            .user_repository(::std::sync::Arc::new(InMemoryUserRepository::builder().build()))
            .uuid_generator(::std::sync::Arc::new(UuidV7Generator::builder().build()))
            .auth_token_generator(::std::sync::Arc::new(JsonWebTokenGenerator::builder()
                .key(::hmac::Hmac::<::sha2::Sha256>::new_from_slice(b"breadcrumbs")?)
                .build()))
            .password_hasher(::std::sync::Arc::new(Argon2PasswordHasher::builder().build()))
            .build())
    }
}

pub type JsPromise<T = ()> = ::core::result::Result<T, ::wasm_bindgen::JsValue>;

pub(crate) trait FallibleExt<T = ()> {
    fn into_js(self) -> JsPromise<T>;
}

impl<T> FallibleExt<T> for ::aliases::result::Fallible<T> {
    fn into_js(self) -> JsPromise<T> {
        self.map_err(|error| ::wasm_bindgen::JsValue::from_str(&error.to_string()))
    }
}

pub type NestedJsPromise<T = ()> = ::core::result::Result<T, ::std::vec::Vec<::wasm_bindgen::JsValue>>;

pub(crate) trait NestedFallibleExt<T = ()> {
    fn into_js(self) -> NestedJsPromise<T>;
}

impl<T, E> NestedFallibleExt<T> for ::aliases::result::Fallible<::core::result::Result<T, ::std::vec::Vec<E>>>
where
    E: ::core::error::Error,
{
    fn into_js(self) -> NestedJsPromise<T> {
        match self {
            ::core::result::Result::Ok(::core::result::Result::Ok(value)) => ::core::result::Result::Ok(value),
            ::core::result::Result::Ok(::core::result::Result::Err(errors)) => ::core::result::Result::Err(errors
                .into_iter()
                .map(|error| ::wasm_bindgen::JsValue::from_str(&error.to_string()))
                .collect()),

            ::core::result::Result::Err(error) =>
                ::core::result::Result::Err(::std::vec::Vec::from([::wasm_bindgen::JsValue::from_str(&error.to_string())])),
        }
    }
}
