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
