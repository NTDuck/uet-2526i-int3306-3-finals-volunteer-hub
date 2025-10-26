use ::async_trait::async_trait;
use ::use_cases::gateways::*;

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

        let uuid = ::domain::Uuid::builder()
            .value(uuid.into_bytes())
            .build();

        ::aliases::result::Fallible::Ok(uuid)
    }

    async fn get_timestamp(self: ::std::sync::Arc<Self>, uuid: &::domain::Uuid) -> ::aliases::result::Fallible<::aliases::time::Timestamp> {
        let uuid = ::uuid::Uuid::from_bytes(**uuid);
        ::aliases::result::Fallible::Ok(uuid.into_timestamp()?)
    }
}

trait UuidExt {
    fn into_timestamp(self) -> ::core::result::Result<::aliases::time::Timestamp, UuidIntoTimestampError>;
}

impl UuidExt for ::uuid::Uuid {
    fn into_timestamp(self) -> ::core::result::Result<::aliases::time::Timestamp, UuidIntoTimestampError> {
        match self.get_timestamp() {
            ::core::option::Option::Some(timestamp) => {
                let (seconds, nanoseconds) = timestamp.to_unix();
                let timestamp = ::chrono::DateTime::from_timestamp(seconds as i64, nanoseconds);

                match timestamp {
                    ::core::option::Option::Some(timestamp) => ::core::result::Result::Ok(timestamp.naive_utc()),
                    ::core::option::Option::None => ::core::result::Result::Err(UuidIntoTimestampError::OutOfRange.into()),
                }
            }
            ::core::option::Option::None => ::core::result::Result::Err(UuidIntoTimestampError::IncompatibleUuidVersion { version: self.get_version_num() }.into()),
        }
    }
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy)]
#[derive(::thiserror::Error)]
enum UuidIntoTimestampError {
    #[error("Incompatible UUID version (expected v1, v6, or v7, found v{version})")]
    IncompatibleUuidVersion { version: usize },
    #[error("Out-of-range number of seconds and/or invalid nanosecond")]
    OutOfRange,
}

// Could consider using sha2
#[derive(::bon::Builder)]
pub struct Argon2PasswordHasher<'pepper> {
    context: ::argon2::Argon2<'pepper>,
}

#[async_trait]
impl<'pepper> PasswordHasher for Argon2PasswordHasher<'pepper> {
    async fn hash(self: ::std::sync::Arc<Self>, password: ::domain::Password) -> ::aliases::result::Fallible<::domain::PasswordDigest> {
        use ::argon2::PasswordHasher as _;

        let salt = ::password_hash::SaltString::generate(&mut ::rand_core::OsRng);
        let digest = self.context.hash_password(password.as_bytes(), &salt)?;

        ::aliases::result::Fallible::Ok(digest.to_string().into())
    }

    async fn verify(self: ::std::sync::Arc<Self>, password: ::domain::Password, digest: ::domain::PasswordDigest) -> ::aliases::result::Fallible<bool> {
        use ::argon2::PasswordVerifier as _;

        let digest = ::password_hash::PasswordHash::new(&digest)?;
        
        ::aliases::result::Fallible::Ok(self.context.verify_password(password.as_bytes(), &digest).is_ok())
    }
}
