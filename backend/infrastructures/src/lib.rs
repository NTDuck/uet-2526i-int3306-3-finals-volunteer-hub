use ::async_trait::async_trait;
use ::use_cases::gateways::*;

#[derive(::bon::Builder)]
pub struct InMemoryUserRepository {
    #[builder(default, with = |value: ::std::collections::BTreeMap<::core::cmp::Reverse<::domain::Uuid>, ::domain::User>| ::tokio::sync::Mutex::new(value))]
    users_by_ids: ::tokio::sync::Mutex<
        ::std::collections::BTreeMap<::core::cmp::Reverse<::domain::Uuid>, ::domain::User>,
    >,

    #[builder(default, with = |value: ::std::collections::HashMap<::domain::Username, ::domain::User>| ::tokio::sync::Mutex::new(value))]
    users_by_usernames:
        ::tokio::sync::Mutex<::std::collections::HashMap<::domain::Username, ::domain::User>>,

    #[builder(default, with = |value: ::std::collections::HashMap<::domain::Email, ::domain::User>| ::tokio::sync::Mutex::new(value))]
    users_by_emails:
        ::tokio::sync::Mutex<::std::collections::HashMap<::domain::Email, ::domain::User>>,
}

#[async_trait]
impl UserRepository for InMemoryUserRepository {
    async fn save(
        self: ::std::sync::Arc<Self>,
        user: ::domain::User,
    ) -> ::aliases::result::Fallible {
        self.users_by_ids
            .lock()
            .await
            .insert(::core::cmp::Reverse(user.id), user.clone());
        self.users_by_usernames
            .lock()
            .await
            .insert(user.username.clone(), user.clone());
        self.users_by_emails
            .lock()
            .await
            .insert(user.email.clone(), user.clone());

        ::aliases::result::Fallible::Ok(())
    }

    async fn get_by_id(
        self: ::std::sync::Arc<Self>,
        id: ::domain::Uuid,
    ) -> ::aliases::result::Fallible<::core::option::Option<::domain::User>> {
        let user = self
            .users_by_ids
            .lock()
            .await
            .get(&::core::cmp::Reverse(id))
            .cloned();

        ::aliases::result::Fallible::Ok(user)
    }

    async fn get_by_username(
        self: ::std::sync::Arc<Self>,
        username: ::domain::Username,
    ) -> ::aliases::result::Fallible<::core::option::Option<::domain::User>> {
        let user = self.users_by_usernames.lock().await.get(&username).cloned();

        ::aliases::result::Fallible::Ok(user)
    }

    async fn get_by_email(
        self: ::std::sync::Arc<Self>,
        email: ::domain::Email,
    ) -> ::aliases::result::Fallible<::core::option::Option<::domain::User>> {
        let user = self.users_by_emails.lock().await.get(&email).cloned();

        ::aliases::result::Fallible::Ok(user)
    }

    async fn contains_id(
        self: ::std::sync::Arc<Self>,
        id: ::domain::Uuid,
    ) -> ::aliases::result::Fallible<bool> {
        let contains = self
            .users_by_ids
            .lock()
            .await
            .contains_key(&::core::cmp::Reverse(id));

        ::aliases::result::Fallible::Ok(contains)
    }

    async fn contains_username(
        self: ::std::sync::Arc<Self>,
        username: ::domain::Username,
    ) -> ::aliases::result::Fallible<bool> {
        let contains = self.users_by_usernames.lock().await.contains_key(&username);

        ::aliases::result::Fallible::Ok(contains)
    }

    async fn contains_email(
        self: ::std::sync::Arc<Self>,
        email: ::domain::Email,
    ) -> ::aliases::result::Fallible<bool> {
        let contains = self.users_by_emails.lock().await.contains_key(&email);

        ::aliases::result::Fallible::Ok(contains)
    }
}

pub struct UuidV7Generator;

#[::bon::bon]
impl UuidV7Generator {
    #[builder]
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl UuidGenerator for UuidV7Generator {
    async fn generate(self: ::std::sync::Arc<Self>) -> ::aliases::result::Fallible<::domain::Uuid> {
        let uuid = ::uuid::Uuid::now_v7();

        let uuid = ::domain::Uuid::builder().value(uuid.into_bytes()).build();

        ::aliases::result::Fallible::Ok(uuid)
    }

    async fn get_timestamp(
        self: ::std::sync::Arc<Self>,
        uuid: &::domain::Uuid,
    ) -> ::aliases::result::Fallible<::aliases::time::Timestamp> {
        let uuid = ::uuid::Uuid::from_bytes(**uuid);
        ::aliases::result::Fallible::Ok(uuid.into_timestamp()?)
    }
}

trait UuidExt {
    fn into_timestamp(
        self,
    ) -> ::core::result::Result<::aliases::time::Timestamp, UuidIntoTimestampError>;
}

impl UuidExt for ::uuid::Uuid {
    fn into_timestamp(
        self,
    ) -> ::core::result::Result<::aliases::time::Timestamp, UuidIntoTimestampError> {
        match self.get_timestamp() {
            ::core::option::Option::Some(timestamp) => {
                let (seconds, nanoseconds) = timestamp.to_unix();
                let timestamp = ::chrono::DateTime::from_timestamp(seconds as i64, nanoseconds);

                match timestamp {
                    ::core::option::Option::Some(timestamp) => {
                        ::core::result::Result::Ok(timestamp.naive_utc())
                    }
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(UuidIntoTimestampError::OutOfRange)
                    }
                }
            }
            ::core::option::Option::None => {
                ::core::result::Result::Err(UuidIntoTimestampError::IncompatibleUuidVersion {
                    version: self.get_version_num(),
                })
            }
        }
    }
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::thiserror::Error)]
enum UuidIntoTimestampError {
    #[error("Incompatible UUID version (expected v1, v6, or v7, found v{version})")]
    IncompatibleUuidVersion { version: usize },
    #[error("Out-of-range number of seconds and/or invalid nanosecond")]
    OutOfRange,
}

#[derive(::bon::Builder)]
pub struct JsonWebTokenGenerator<Key> {
    key: Key, // expects something like `::hmac::Hmac<::sha2::Sha256>`
}

#[async_trait]
impl<Key> AuthenticationTokenGenerator for JsonWebTokenGenerator<Key>
where
    Key: ::jwt::SigningAlgorithm
        + ::jwt::VerifyingAlgorithm
        + ::core::marker::Send
        + ::core::marker::Sync,
{
    async fn generate(
        self: ::std::sync::Arc<Self>,
        payload: ::use_cases::gateways::models::AuthenticationTokenPayload,
    ) -> ::aliases::result::Fallible<::aliases::string::String> {
        use ::jwt::SignWithKey as _;

        let token = payload.sign_with_key(&self.key)?;
        ::aliases::result::Fallible::Ok(token.into())
    }

    async fn get_payload(
        self: ::std::sync::Arc<Self>,
        token: ::aliases::string::String,
    ) -> ::aliases::result::Fallible<
        ::core::option::Option<::use_cases::gateways::models::AuthenticationTokenPayload>,
    > {
        use ::jwt::VerifyWithKey as _;

        let payload = token.verify_with_key(&self.key)?;
        ::aliases::result::Fallible::Ok(payload)
    }
}

#[derive(::bon::Builder)]
pub struct Argon2PasswordHasher<'pepper> {
    #[builder(default)]
    context: ::argon2::Argon2<'pepper>,
}

#[async_trait]
impl<'pepper> PasswordHasher for Argon2PasswordHasher<'pepper> {
    async fn hash(
        self: ::std::sync::Arc<Self>,
        password: ::domain::Password,
    ) -> ::aliases::result::Fallible<::domain::PasswordDigest> {
        use ::argon2::PasswordHasher as _;

        // TODO: make `::argon2::password_hash::rand_core::OsError` implement `::std::error::Error`
        // `unwrap()` for now, should propagate with `?` later
        let salt = ::argon2::password_hash::SaltString::try_from_rng(
            &mut ::argon2::password_hash::rand_core::OsRng,
        )
        .unwrap();
        let digest = self.context.hash_password(password.as_bytes(), &salt)?;

        ::aliases::result::Fallible::Ok(digest.to_string().into())
    }

    async fn verify(
        self: ::std::sync::Arc<Self>,
        password: ::domain::Password,
        digest: ::domain::PasswordDigest,
    ) -> ::aliases::result::Fallible<bool> {
        use ::argon2::PasswordVerifier as _;

        let digest = ::argon2::password_hash::PasswordHash::new(&digest)?;

        ::aliases::result::Fallible::Ok(
            self.context
                .verify_password(password.as_bytes(), &digest)
                .is_ok(),
        )
    }
}
