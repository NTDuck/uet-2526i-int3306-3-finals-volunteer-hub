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
    #[wasm_bindgen]
    pub async fn create() -> Promise<Self> {
        Self::new().await.into_promise()
    }

    async fn new() -> ::aliases::result::Fallible<Self> {
        Gateways::new().await.map(::core::convert::Into::into)
    }

    #[wasm_bindgen(js_name = signIn)]
    pub async fn sign_in(&self, request: SignInRequest) -> Promise<SignInOkResponse> {
        ::std::sync::Arc::clone(&self.sign_in_boundary).apply(request).await.into_promise()
    }

    #[wasm_bindgen(js_name = signUp)]
    pub async fn sign_up(&self, request: SignUpRequest) -> Promise<SignUpOkResponse> {
        ::std::sync::Arc::clone(&self.sign_up_boundary).apply(request).await.into_promise()
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
    async fn new() -> ::aliases::result::Fallible<Self> {
        use ::hmac::Mac as _;

        // let logger = ::tracing_appender::rolling::never("/logs/wasm-bindings/", ".log");
        // let (logger, _logger_guard) = ::tracing_appender::non_blocking(logger);

        // ::tracing_subscriber::fmt()
        //     .with_writer(logger)
        //     .with_env_filter(::tracing_subscriber::EnvFilter::try_from_default_env()?)
        //     .with_ansi(false)
        //     .init();

        ::tracing_wasm::try_set_as_global_default()?;

        // ::dotenvy::from_filename("../../.env")?;

        ::tracing::debug!("{}", ::std::env!("CARGO_MANIFEST_DIR"));
        ::tracing::debug!("{}", ::std::env!("CARGO_WORKSPACE_DIR"));

        // ::tracing::debug!("{}",::std::env::var("JWT_SECRET_KEY")?);
        // ::tracing::debug!("{}",::std::env::var("ARGON2_SECRET_KEY")?);

        ::aliases::result::Fallible::Ok(Self::builder()
            .user_repository(::std::sync::Arc::new(InMemoryUserRepository::builder().build()))
            .uuid_generator(::std::sync::Arc::new(UuidV7Generator::builder().build()))
            .auth_token_generator(::std::sync::Arc::new(JsonWebTokenGenerator::builder()
                // .key(::hmac::Hmac::<::sha2::Sha256>::new_from_slice("root".as_bytes())?)
                .key(::hmac::Hmac::<::sha2::Sha256>::new_from_slice(::std::env::var("JWT_SECRET_KEY")?.as_bytes())?)
                .build()))
            .password_hasher(::std::sync::Arc::new(Argon2PasswordHasher::builder()
                .context(::argon2::Argon2::new_with_secret(
                    // "root".as_bytes(),
                    ::std::boxed::Box::leak(::std::env::var("ARGON2_SECRET_KEY")?.into_boxed_str()).as_bytes(),
                    ::argon2::Algorithm::Argon2id, ::argon2::Version::V0x13,
                    ::argon2::Params::default())?)
                .build()))
            .build())
    }
}

pub type Promise<T = ()> = ::core::result::Result<T, ::wasm_bindgen::JsValue>;

trait FallibleExt<T = ()> {
    fn into_promise(self) -> Promise<T>;
}

impl<T> FallibleExt<T> for ::aliases::result::Fallible<T> {
    fn into_promise(self) -> Promise<T> {
        self.map_err(|error| ::wasm_bindgen::JsValue::from_str(&error.to_string()))
    }
}

impl<T, E> FallibleExt<T> for ::aliases::result::Fallible<::core::result::Result<T, ::std::vec::Vec<E>>>
where
    E: ::core::error::Error,
{
    fn into_promise(self) -> Promise<T> {
        self.map_err(|error| ::wasm_bindgen::JsValue::from_str(&error.to_string()))
            .and_then(|inner| inner
                .map_err(|errors| errors.into_iter()
                    .map(|error| ::wasm_bindgen::JsValue::from_str(&error.to_string()))
                    .collect::<::std::vec::Vec<_>>())
                .map_err(::core::convert::Into::into))
    }
}
