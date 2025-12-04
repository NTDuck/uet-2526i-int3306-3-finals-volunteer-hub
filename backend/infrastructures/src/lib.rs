use ::axiom::prelude::*;
use ::use_cases::gateways::*;
use ::rayon::prelude::*;

/// Since implementations of `::domain::Uuid` preserves order, consider using `::std::vec::Vec<_>` for performance gains.

#[derive(::bon::Builder)]
pub struct InMemoryEventRepository {
    #[builder(default, with = |value: ::std::collections::BTreeMap<::core::cmp::Reverse<::domain::Uuid>, ::domain::Event>| ::tokio::sync::Mutex::new(value))]
    events_by_ids:
        ::tokio::sync::Mutex<::std::collections::BTreeMap<::core::cmp::Reverse<::domain::Uuid>, ::domain::Event>>,
}

#[async_trait]
impl EventRepository for InMemoryEventRepository {
    async fn save(self: ::std::sync::Arc<Self>, event: ::domain::Event) -> ::axiom::result::Fallible {
        self.events_by_ids.lock().await.insert(::core::cmp::Reverse(event.id), event);

        ::axiom::result::Fallible::Ok(())
    }

    async fn remove(self: ::std::sync::Arc<Self>, event_id: ::domain::Uuid) -> ::axiom::result::Fallible {
        self.events_by_ids.lock().await.remove(&::core::cmp::Reverse(event_id));

        ::axiom::result::Fallible::Ok(())
    }

    async fn get_by_id(
        self: ::std::sync::Arc<Self>, event_id: ::domain::Uuid,
    ) -> ::axiom::result::Fallible<::core::option::Option<::domain::Event>> {
        self.events_by_ids.lock().await.get(&::core::cmp::Reverse(event_id)).cloned().into_ok()
    }

    async fn contains_id(self: ::std::sync::Arc<Self>, event_id: ::domain::Uuid) -> ::axiom::result::Fallible<bool> {
        self.events_by_ids.lock().await.contains_key(&::core::cmp::Reverse(event_id)).into_ok()
    }

    async fn search(
        self: ::std::sync::Arc<Self>, filter: EventRepositorySearchFilter,
    ) -> ::axiom::result::Fallible<::std::vec::Vec<::domain::Event>> {
        let filter = move |event: &&::domain::Event| -> bool {
            if let ::core::option::Option::Some(ref query) = filter.query {
                if event.name.is_subsequence(&query) || event.description.is_subsequence(&query) || event.categories.iter().any(|category| category.is_subsequence(&query)) || event.location.is_subsequence(&query) {
                    return true;
                }
            }

            if let ::core::option::Option::Some(ref filter_statuses) = filter.statuses {
                if filter_statuses.contains(&event.statuses.last().into()) {
                    return true;
                }
            }

            let event_timestamp = event.statuses.last().at();

            match filter.timestamps {
                ::core::ops::Range { start: ::core::option::Option::Some(start), end: ::core::option::Option::Some(end) } => 
                    event_timestamp >= start && event_timestamp <= end,
                ::core::ops::Range { start: ::core::option::Option::Some(start), end: ::core::option::Option::None } => 
                    event_timestamp <= start,
                ::core::ops::Range { start: ::core::option::Option::None, end: ::core::option::Option::Some(end) } => 
                    event_timestamp >= end,
                _ => false,
            }
        };

        self.events_by_ids.lock().await.iter()
            .map(|(_, event)| event)
            .filter(filter)
            .cloned()
            .collect::<::std::vec::Vec<_>>()
            .into_ok()
    }

    async fn view(self: ::std::sync::Arc<Self>) -> ::axiom::result::Fallible<::std::vec::Vec<::domain::Event>> {
        self.events_by_ids.lock().await.iter()
            .map(|(_, event)| event)
            .cloned()
            .collect::<::std::vec::Vec<_>>()
            .into_ok()
    }
}

pub trait StringSliceExt {
    fn is_subsequence(&self, pattern: &str) -> bool;
}

impl StringSliceExt for str {
    fn is_subsequence(&self, pattern: &str) -> bool
    {
        let mut it = self.chars();

        for pat_chr in pattern.chars() {
            match it.find(|&hey_chr| hey_chr == pat_chr) {
                ::core::option::Option::Some(_) => continue,
                ::core::option::Option::None => return false,
            }
        }

        true
    }
}

#[derive(::bon::Builder)]
pub struct InMemoryUserRepository {
    #[builder(default, with = |value: ::std::collections::BTreeMap<::core::cmp::Reverse<::domain::Uuid>, ::domain::User>| ::tokio::sync::Mutex::new(value))]
    users_by_ids:
        ::tokio::sync::Mutex<::std::collections::BTreeMap<::core::cmp::Reverse<::domain::Uuid>, ::domain::User>>,

    #[builder(default, with = |value: ::std::collections::HashMap<::domain::Username, ::domain::User>| ::tokio::sync::Mutex::new(value))]
    users_by_usernames: ::tokio::sync::Mutex<::std::collections::HashMap<::domain::Username, ::domain::User>>,

    #[builder(default, with = |value: ::std::collections::HashMap<::domain::Email, ::domain::User>| ::tokio::sync::Mutex::new(value))]
    users_by_emails: ::tokio::sync::Mutex<::std::collections::HashMap<::domain::Email, ::domain::User>>,
}

#[async_trait]
impl UserRepository for InMemoryUserRepository {
    async fn save(self: ::std::sync::Arc<Self>, user: ::domain::User) -> ::axiom::result::Fallible {
        self.users_by_ids
            .lock()
            .await
            .insert(::core::cmp::Reverse(user.id), user.clone());
        self.users_by_usernames.lock().await.insert(user.username.clone(), user.clone());
        self.users_by_emails.lock().await.insert(user.email.clone(), user.clone());

        ().into_ok()
    }

    async fn get_by_id(
        self: ::std::sync::Arc<Self>, id: ::domain::Uuid,
    ) -> ::axiom::result::Fallible<::core::option::Option<::domain::User>> {
        let user = self.users_by_ids.lock().await.get(&::core::cmp::Reverse(id)).cloned();

        ::axiom::result::Fallible::Ok(user)
    }

    async fn get_by_username(
        self: ::std::sync::Arc<Self>, username: ::domain::Username,
    ) -> ::axiom::result::Fallible<::core::option::Option<::domain::User>> {
        let user = self.users_by_usernames.lock().await.get(&username).cloned();

        ::axiom::result::Fallible::Ok(user)
    }

    async fn get_by_email(
        self: ::std::sync::Arc<Self>, email: ::domain::Email,
    ) -> ::axiom::result::Fallible<::core::option::Option<::domain::User>> {
        let user = self.users_by_emails.lock().await.get(&email).cloned();

        ::axiom::result::Fallible::Ok(user)
    }

    async fn contains_id(self: ::std::sync::Arc<Self>, id: ::domain::Uuid) -> ::axiom::result::Fallible<bool> {
        let contains = self.users_by_ids.lock().await.contains_key(&::core::cmp::Reverse(id));

        ::axiom::result::Fallible::Ok(contains)
    }

    async fn contains_username(
        self: ::std::sync::Arc<Self>, username: ::domain::Username,
    ) -> ::axiom::result::Fallible<bool> {
        let contains = self.users_by_usernames.lock().await.contains_key(&username);

        ::axiom::result::Fallible::Ok(contains)
    }

    async fn contains_email(self: ::std::sync::Arc<Self>, email: ::domain::Email) -> ::axiom::result::Fallible<bool> {
        let contains = self.users_by_emails.lock().await.contains_key(&email);

        ::axiom::result::Fallible::Ok(contains)
    }

    async fn search(
        self: ::std::sync::Arc<Self>, _filter: UserRepositorySearchFilter,
    ) -> ::axiom::result::Fallible<::std::vec::Vec<::domain::User>> {
        todo!()
    }

    async fn view(self: ::std::sync::Arc<Self>) -> ::axiom::result::Fallible<::std::vec::Vec<::domain::User>> {
        todo!()
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
    async fn generate(self: ::std::sync::Arc<Self>) -> ::axiom::result::Fallible<::domain::Uuid> {
        let uuid = ::uuid::Uuid::now_v7();

        let uuid = ::domain::Uuid::builder().value(uuid.into_bytes()).build();

        ::axiom::result::Fallible::Ok(uuid)
    }
}

trait UuidExt {
    fn into_timestamp(self) -> ::core::result::Result<::axiom::time::Timestamp, UuidIntoTimestampError>;
}

impl UuidExt for ::uuid::Uuid {
    fn into_timestamp(self) -> ::core::result::Result<::axiom::time::Timestamp, UuidIntoTimestampError> {
        match self.get_timestamp() {
            ::core::option::Option::Some(timestamp) => {
                let (seconds, nanoseconds) = timestamp.to_unix();

                ::axiom::time::Timestamp::from_timestamp(seconds as i64, nanoseconds)
                    .ok_or(UuidIntoTimestampError::OutOfRange)
            },

            ::core::option::Option::None =>
                ::core::result::Result::Err(UuidIntoTimestampError::IncompatibleUuidVersion {
                    version: self.get_version_num(),
                }),
        }
    }
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::thiserror::Error)]
enum UuidIntoTimestampError {
    #[error("Incompatible UUID version (expected v1, v6, or v7, found v{version})")]
    IncompatibleUuidVersion {
        version: usize,
    },
    #[error("Out-of-range number of seconds and/or invalid nanosecond")]
    OutOfRange,
}

pub struct LowerUrnUuidCodec;

#[::bon::bon]
impl LowerUrnUuidCodec {
    #[builder(builder_type(vis = "pub"))]
    fn new() -> Self {
        Self
    }
}

#[async_trait]
impl UuidCodec for LowerUrnUuidCodec {
    async fn format(
        self: ::std::sync::Arc<Self>, uuid: ::domain::Uuid,
    ) -> ::axiom::result::Fallible<::axiom::string::String> {
        let uuid = ::uuid::Uuid::from_bytes(*uuid);

        let mut buffer = [0u8; 45];
        let urn = uuid.as_urn().encode_lower(&mut buffer).to_string().into();

        ::axiom::result::Fallible::Ok(urn)
    }

    async fn parse(
        self: ::std::sync::Arc<Self>, urn: ::axiom::string::String,
    ) -> ::axiom::result::Fallible<::domain::Uuid> {
        let uuid = ::uuid::Uuid::parse_str(&urn)?;
        let uuid = ::domain::Uuid::builder().value(uuid.into_bytes()).build();

        ::axiom::result::Fallible::Ok(uuid)
    }
}

#[derive(::bon::Builder)]
pub struct JsonWebTokenGenerator<Key> {
    key: Key, // expects something like `::hmac::Hmac<::sha2::Sha256>`
}

#[async_trait]
impl<Key> AuthTokenGenerator for JsonWebTokenGenerator<Key>
where
    Key: ::jwt::SigningAlgorithm + ::jwt::VerifyingAlgorithm + ::core::marker::Send + ::core::marker::Sync,
{
    async fn generate(
        self: ::std::sync::Arc<Self>, payload: ::use_cases::gateways::AuthenticationTokenPayload,
    ) -> ::axiom::result::Fallible<::axiom::string::String> {
        use ::jwt::SignWithKey as _;

        let token = payload.sign_with_key(&self.key)?;
        ::axiom::result::Fallible::Ok(token.into())
    }

    async fn get_payload(
        self: ::std::sync::Arc<Self>, token: ::axiom::string::String,
    ) -> ::axiom::result::Fallible<::core::option::Option<::use_cases::gateways::AuthenticationTokenPayload>> {
        use ::jwt::VerifyWithKey as _;

        let payload = token.verify_with_key(&self.key)?;
        ::axiom::result::Fallible::Ok(payload)
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
        self: ::std::sync::Arc<Self>, password: ::domain::Password,
    ) -> ::axiom::result::Fallible<::domain::PasswordDigest> {
        use ::argon2::PasswordHasher as _;

        // TODO: make `::argon2::password_hash::rand_core::OsError` implement
        // `::std::error::Error` `unwrap()` for now, should propagate with `?`
        // later
        let salt =
            ::argon2::password_hash::SaltString::try_from_rng(&mut ::argon2::password_hash::rand_core::OsRng).unwrap();
        let digest = self.context.hash_password(password.as_bytes(), &salt)?;

        ::axiom::result::Fallible::Ok(digest.to_string().into())
    }

    async fn verify(
        self: ::std::sync::Arc<Self>, password: ::domain::Password, digest: ::domain::PasswordDigest,
    ) -> ::axiom::result::Fallible<bool> {
        use ::argon2::PasswordVerifier as _;

        let digest = ::argon2::password_hash::PasswordHash::new(&digest)?;

        ::axiom::result::Fallible::Ok(self.context.verify_password(password.as_bytes(), &digest).is_ok())
    }
}
