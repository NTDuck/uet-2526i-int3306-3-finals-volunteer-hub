#[cfg(any(
    all(feature = "ahash", any(feature = "fxhash", feature = "metrohash", feature = "seahash")),
    all(feature = "fxhash", any(feature = "ahash", feature = "metrohash", feature = "seahash")),
    all(feature = "metrohash", any(feature = "ahash", feature = "fxhash", feature = "seahash")),
    all(feature = "seahash", any(feature = "ahash", feature = "fxhash", feature = "metrohash")),
))]
::core::compile_error!("The following feature flags are mutually exclusive: `ahash`, `fxhash`, `metrohash`, `seahash`");

use ::axiom::prelude::*;
use ::use_cases::gateways::*;
use ::rayon::prelude::*;

/// Since implementations of `::domain::Uuid` preserves order, consider using `::std::vec::Vec<_>` for performance gains.

#[derive(::bon::Builder)]
pub struct InMemoryEventRepository {
    #[builder(default, with = |value: ::std::collections::BTreeMap<::core::cmp::Reverse<::domain::Uuid>, ::domain::Event>| ::tokio::sync::RwLock::new(value))]
    events_by_ids:
        ::tokio::sync::RwLock<::std::collections::BTreeMap<::core::cmp::Reverse<::domain::Uuid>, ::domain::Event>>,
}

#[async_trait]
impl EventRepository for InMemoryEventRepository {
    async fn save(self: ::std::sync::Arc<Self>, event: ::domain::Event) -> ::axiom::result::Fallible {
        self.events_by_ids.write().await.insert(::core::cmp::Reverse(event.id), event);

        ::axiom::result::Fallible::Ok(())
    }

    async fn remove(self: ::std::sync::Arc<Self>, event_id: ::domain::Uuid) -> ::axiom::result::Fallible {
        self.events_by_ids.write().await.remove(&::core::cmp::Reverse(event_id));

        ::axiom::result::Fallible::Ok(())
    }

    async fn get_by_id(
        self: ::std::sync::Arc<Self>, event_id: ::domain::Uuid,
    ) -> ::axiom::result::Fallible<::core::option::Option<::domain::Event>> {
        self.events_by_ids.read().await.get(&::core::cmp::Reverse(event_id)).cloned().into_ok()
    }

    async fn contains_id(self: ::std::sync::Arc<Self>, event_id: ::domain::Uuid) -> ::axiom::result::Fallible<bool> {
        self.events_by_ids.read().await.contains_key(&::core::cmp::Reverse(event_id)).into_ok()
    }

    async fn search(
        self: ::std::sync::Arc<Self>, filter: EventRepositorySearchFilter,
    ) -> ::axiom::result::Fallible<::std::vec::Vec<::domain::Event>> {
        use self::string::StringSliceExt as _;

        let filter_query = filter.query.as_deref().map(str::trim).map(str::to_lowercase);

        let apply_filter_query = move |event: &&::domain::Event| {
            if let ::core::option::Option::Some(ref query) = filter_query {
                event.name.is_subsequence(query)
                    || event.description.is_subsequence(query)
                    || event.categories.iter().any(|category| category.is_subsequence(query))
                    || event.location.is_subsequence(query)
            } else {
                true
            }
        };

        let filter_statuses = filter.statuses.as_ref().map(|statuses| statuses.iter().collect::<::std::collections::HashSet<_>>());

        let apply_filter_statuses = move |event: &&::domain::Event| {
            if let ::core::option::Option::Some(ref statuses) = filter_statuses {
                statuses.contains(&&event.statuses.last().into())
            } else {
                true
            }
        };

        let apply_filter_timestamps = move |event: &&::domain::Event| {
            let event_timestamp = event.statuses.last().at();

            match filter.timestamps {
                ::core::ops::Range { start: ::core::option::Option::Some(start), end: ::core::option::Option::Some(end) } => 
                    event_timestamp >= start && event_timestamp <= end,
                ::core::ops::Range { start: ::core::option::Option::Some(start), end: ::core::option::Option::None } => 
                    event_timestamp <= start,
                ::core::ops::Range { start: ::core::option::Option::None, end: ::core::option::Option::Some(end) } => 
                    event_timestamp >= end,
                _ => true,
            }
        };

        let apply_filter = move |event: &&::domain::Event| apply_filter_query(event) && apply_filter_statuses(event) && apply_filter_timestamps(event);

        self.events_by_ids.read().await.values()
            .filter(apply_filter)
            .cloned()
            .collect::<::std::vec::Vec<_>>()
            .into_ok()
    }

    async fn view(self: ::std::sync::Arc<Self>) -> ::axiom::result::Fallible<::std::vec::Vec<::domain::Event>> {
        self.events_by_ids.read().await.values()
            .cloned()
            .collect::<::std::vec::Vec<_>>()
            .into_ok()
    }
}

#[derive(::bon::Builder)]
pub struct InMemoryEventRegistrationRepository {
    #[builder(default, with = |value: ::std::collections::BTreeMap<::core::cmp::Reverse<::domain::Uuid>, ::domain::EventRegistration>| ::tokio::sync::RwLock::new(value))]
    registrations_by_ids: ::tokio::sync::RwLock<::std::collections::BTreeMap<::core::cmp::Reverse<::domain::Uuid>, ::domain::EventRegistration>>,

    #[builder(default, with = |value: ::std::collections::HashMap<(::domain::Uuid, ::domain::Uuid), ::domain::EventRegistration, self::hash::BuildHasher>| ::tokio::sync::RwLock::new(value))]
    registrations_by_event_and_volunteer_ids: ::tokio::sync::RwLock<::std::collections::HashMap<(::domain::Uuid, ::domain::Uuid), ::domain::EventRegistration, self::hash::BuildHasher>>,
}

#[async_trait]
impl EventRegistrationRepository for InMemoryEventRegistrationRepository {
    async fn save(self: ::std::sync::Arc<Self>, registration: ::domain::EventRegistration)
        -> ::axiom::result::Fallible {
        self.registrations_by_ids.write().await.insert(::core::cmp::Reverse(registration.id), registration.clone());
        self.registrations_by_event_and_volunteer_ids.write().await.insert((registration.event_id, registration.volunteer_id), registration.clone());

        ::axiom::result::Fallible::Ok(())
    }

    async fn get_by_id(
        self: ::std::sync::Arc<Self>, id: ::domain::Uuid,
    ) -> ::axiom::result::Fallible<::core::option::Option<::domain::EventRegistration>> {
        self.registrations_by_ids.read().await.get(&::core::cmp::Reverse(id)).cloned().into_ok()
    }

    async fn get_by_event_and_volunteer_id(
        self: ::std::sync::Arc<Self>, event_id: ::domain::Uuid, user_id: ::domain::Uuid,
    ) -> ::axiom::result::Fallible<::core::option::Option<::domain::EventRegistration>> {
        self.registrations_by_event_and_volunteer_ids.read().await.get(&(event_id, user_id)).cloned().into_ok()
    }

    async fn view_by_event_id(
        self: ::std::sync::Arc<Self>, event_id_: ::domain::Uuid,
    ) -> ::axiom::result::Fallible<::std::vec::Vec<::domain::EventRegistration>> {
        self.registrations_by_event_and_volunteer_ids.read().await.iter()
            .filter(|((event_id, _), _)| *event_id == event_id_)
            .map(|(_, event_registration)| event_registration)
            .cloned()
            .collect::<::std::vec::Vec<_>>()
            .into_ok()
    }

    async fn view_by_volunteer_id(
        self: ::std::sync::Arc<Self>, volunteer_id_: ::domain::Uuid,
    ) -> ::axiom::result::Fallible<::std::vec::Vec<::domain::EventRegistration>> {
        self.registrations_by_event_and_volunteer_ids.read().await.iter()
            .filter(|((_, volunteer_id), _)| *volunteer_id == volunteer_id_)
            .map(|(_, event_registration)| event_registration)
            .cloned()
            .collect::<::std::vec::Vec<_>>()
            .into_ok()
    }
}

#[derive(::bon::Builder)]
pub struct InMemoryEventPostRepository {
    #[builder(default, with = |value: ::std::collections::BTreeMap<::core::cmp::Reverse<::domain::Uuid>, ::domain::EventPost>| ::tokio::sync::RwLock::new(value))]
    event_posts_by_ids: ::tokio::sync::RwLock<::std::collections::BTreeMap<::core::cmp::Reverse<::domain::Uuid>, ::domain::EventPost>>,
}

#[async_trait]
impl EventPostRepository for InMemoryEventPostRepository {
    async fn save(self: ::std::sync::Arc<Self>, post: ::domain::EventPost) -> ::axiom::result::Fallible {
        self.event_posts_by_ids.write().await.insert(::core::cmp::Reverse(post.id), post.clone());

        ::axiom::result::Fallible::Ok(())
    }

    async fn remove(self: ::std::sync::Arc<Self>, post_id: ::domain::Uuid) -> ::axiom::result::Fallible {
        self.event_posts_by_ids.write().await.remove(&::core::cmp::Reverse(post_id));

        ::axiom::result::Fallible::Ok(())
    }

    async fn get_by_id(
        self: ::std::sync::Arc<Self>, post_id: ::domain::Uuid,
    ) -> ::axiom::result::Fallible<::core::option::Option<::domain::EventPost>> {
        self.event_posts_by_ids.read().await.get(&::core::cmp::Reverse(post_id)).cloned().into_ok()
    }

    async fn view_by_event_id(
        self: ::std::sync::Arc<Self>, event_id_: ::domain::Uuid,
    ) -> ::axiom::result::Fallible<::std::vec::Vec<::domain::EventPost>> {
        self.event_posts_by_ids.read().await.values()
            .filter(|&&::domain::EventPost { event_id, .. }| event_id == event_id_)
            .cloned()
            .collect::<::std::vec::Vec<_>>()
            .into_ok()
    }
}

#[derive(::bon::Builder)]
pub struct InMemoryEventPostReactionRepository {
    #[builder(default, with = |value: ::std::collections::BTreeMap<::core::cmp::Reverse<::domain::Uuid>, ::domain::EventPostReaction>| ::tokio::sync::RwLock::new(value))]
    reactions_by_ids: ::tokio::sync::RwLock<::std::collections::BTreeMap<::core::cmp::Reverse<::domain::Uuid>, ::domain::EventPostReaction>>,

    #[builder(default, with = |value: ::std::collections::HashMap<(::domain::Uuid, ::domain::Uuid), ::domain::EventPostReaction, self::hash::BuildHasher>| ::tokio::sync::RwLock::new(value))]
    reactions_by_post_and_user_ids: ::tokio::sync::RwLock<::std::collections::HashMap<(::domain::Uuid, ::domain::Uuid), ::domain::EventPostReaction, self::hash::BuildHasher>>,
}

#[async_trait]
impl EventPostReactionRepository for InMemoryEventPostReactionRepository {
    async fn save(self: ::std::sync::Arc<Self>, reaction: ::domain::EventPostReaction) -> ::axiom::result::Fallible {
        self.reactions_by_ids.write().await.insert(::core::cmp::Reverse(reaction.id), reaction.clone());
        self.reactions_by_post_and_user_ids.write().await.insert((reaction.post_id, reaction.author_id), reaction.clone());

        ::axiom::result::Fallible::Ok(())
    }

    async fn remove(self: ::std::sync::Arc<Self>, reaction_id: ::domain::Uuid) -> ::axiom::result::Fallible {
        self.reactions_by_ids.write().await.remove(&::core::cmp::Reverse(reaction_id));
        self.reactions_by_post_and_user_ids.write().await.remove(&(reaction_id, reaction_id));

        ::axiom::result::Fallible::Ok(())
    }

    async fn get_by_id(
        self: ::std::sync::Arc<Self>, reaction_id: ::domain::Uuid,
    ) -> ::axiom::result::Fallible<::core::option::Option<::domain::EventPostReaction>> {
        self.reactions_by_ids.read().await.get(&::core::cmp::Reverse(reaction_id)).cloned().into_ok()
    }

    async fn get_by_post_and_user_id(
        self: ::std::sync::Arc<Self>, post_id: ::domain::Uuid, user_id: ::domain::Uuid,
    ) -> ::axiom::result::Fallible<::core::option::Option<::domain::EventPostReaction>> {
        self.reactions_by_post_and_user_ids.read().await.get(&(post_id, user_id)).cloned().into_ok()
    }

    async fn contains_post_and_user_id(
        self: ::std::sync::Arc<Self>, post_id: ::domain::Uuid, user_id: ::domain::Uuid,
    ) -> ::axiom::result::Fallible<bool> {
        self.reactions_by_post_and_user_ids.read().await.contains_key(&(post_id, user_id)).into_ok()
    }

    async fn view_by_post_id(
        self: ::std::sync::Arc<Self>, post_id_: ::domain::Uuid,
    ) -> ::axiom::result::Fallible<::std::vec::Vec<::domain::EventPostReaction>> {
        self.reactions_by_ids.read().await.values()
            .filter(|&&::domain::EventPostReaction { post_id, .. }| post_id == post_id_)
            .cloned()
            .collect::<::std::vec::Vec<_>>()
            .into_ok()
    }

    async fn count_by_post_id(self: ::std::sync::Arc<Self>, post_id_: ::domain::Uuid) -> ::axiom::result::Fallible<u64> {
        (
            self.reactions_by_ids.read().await.values()
            .filter(|&&::domain::EventPostReaction { post_id, .. }| post_id == post_id_)
            .count() as u64
        )
            .into_ok()
    }
}

#[derive(::bon::Builder)]
pub struct InMemoryEventPostCommentRepository {
    #[builder(default, with = |value: ::std::collections::BTreeMap<::core::cmp::Reverse<::domain::Uuid>, ::domain::EventPostComment>| ::tokio::sync::RwLock::new(value))]
    comments_by_ids: ::tokio::sync::RwLock<::std::collections::BTreeMap<::core::cmp::Reverse<::domain::Uuid>, ::domain::EventPostComment>>,

    #[builder(default, with = |value: ::std::collections::HashMap<(::domain::Uuid, ::domain::Uuid), ::domain::EventPostComment, self::hash::BuildHasher>| ::tokio::sync::RwLock::new(value))]
    comments_by_post_and_user_ids: ::tokio::sync::RwLock<::std::collections::HashMap<(::domain::Uuid, ::domain::Uuid), ::domain::EventPostComment, self::hash::BuildHasher>>,
}

#[async_trait]
impl EventPostCommentRepository for InMemoryEventPostCommentRepository {
    async fn save(self: ::std::sync::Arc<Self>, comment: ::domain::EventPostComment) -> ::axiom::result::Fallible {
        self.comments_by_ids.write().await.insert(::core::cmp::Reverse(comment.id), comment.clone());
        self.comments_by_post_and_user_ids.write().await.insert((comment.post_id, comment.author_id), comment.clone());

        ::axiom::result::Fallible::Ok(())
    }

    async fn remove(self: ::std::sync::Arc<Self>, comment_id: ::domain::Uuid) -> ::axiom::result::Fallible {
        self.comments_by_ids.write().await.remove(&::core::cmp::Reverse(comment_id));
        self.comments_by_post_and_user_ids.write().await.remove(&(comment_id, comment_id));

        ::axiom::result::Fallible::Ok(())
    }

    async fn get_by_id(
        self: ::std::sync::Arc<Self>, comment_id: ::domain::Uuid,
    ) -> ::axiom::result::Fallible<::core::option::Option<::domain::EventPostComment>> {
        self.comments_by_ids.read().await.get(&::core::cmp::Reverse(comment_id)).cloned().into_ok()
    }

    async fn contains_id(self: ::std::sync::Arc<Self>, comment_id: ::domain::Uuid) -> ::axiom::result::Fallible<bool> {
        self.comments_by_ids.read().await.contains_key(&::core::cmp::Reverse(comment_id)).into_ok()
    }

    async fn view_by_post_and_user_id(
        self: ::std::sync::Arc<Self>, post_id_: ::domain::Uuid, user_id_: ::domain::Uuid,
    ) -> ::axiom::result::Fallible<::std::vec::Vec<::domain::EventPostComment>> {
        self.comments_by_post_and_user_ids.read().await.values()
            .filter(|&&::domain::EventPostComment { post_id, author_id, .. }| post_id == post_id && author_id == user_id_)
            .cloned()
            .collect::<::std::vec::Vec<_>>()
            .into_ok()
    }

    async fn view_by_post_id(
        self: ::std::sync::Arc<Self>, post_id_: ::domain::Uuid,
    ) -> ::axiom::result::Fallible<::std::vec::Vec<::domain::EventPostComment>> {
        self.comments_by_ids.read().await.values()
            .filter(|&&::domain::EventPostComment { post_id, .. }| post_id == post_id_)
            .cloned()
            .collect::<::std::vec::Vec<_>>()
            .into_ok()
    }

    async fn count_by_post_id(self: ::std::sync::Arc<Self>, post_id_: ::domain::Uuid) -> ::axiom::result::Fallible<u64> {
        (
            self.comments_by_ids.read().await.values()
            .filter(|&&::domain::EventPostComment { post_id, .. }| post_id == post_id_)
            .count() as u64
        )
            .into_ok()
    }
}

#[derive(::bon::Builder)]
pub struct InMemoryUserRepository {
    #[builder(default, with = |value: ::std::collections::BTreeMap<::core::cmp::Reverse<::domain::Uuid>, ::domain::User>| ::tokio::sync::RwLock::new(value))]
    users_by_ids:
        ::tokio::sync::RwLock<::std::collections::BTreeMap<::core::cmp::Reverse<::domain::Uuid>, ::domain::User>>,

    #[builder(default, with = |value: ::std::collections::HashMap<::domain::Username, ::domain::User, self::hash::BuildHasher>| ::tokio::sync::RwLock::new(value))]
    users_by_usernames: ::tokio::sync::RwLock<::std::collections::HashMap<::domain::Username, ::domain::User, self::hash::BuildHasher>>,

    #[builder(default, with = |value: ::std::collections::HashMap<::domain::Email, ::domain::User, self::hash::BuildHasher>| ::tokio::sync::RwLock::new(value))]
    users_by_emails: ::tokio::sync::RwLock<::std::collections::HashMap<::domain::Email, ::domain::User, self::hash::BuildHasher>>,
}

#[async_trait]
impl UserRepository for InMemoryUserRepository {
    async fn save(self: ::std::sync::Arc<Self>, user: ::domain::User) -> ::axiom::result::Fallible {
        self.users_by_ids
            .write()
            .await
            .insert(::core::cmp::Reverse(user.id), user.clone());
        self.users_by_usernames.write().await.insert(user.username.clone(), user.clone());
        self.users_by_emails.write().await.insert(user.email.clone(), user.clone());

        ::axiom::result::Fallible::Ok(())
    }

    async fn get_by_id(
        self: ::std::sync::Arc<Self>, user_id: ::domain::Uuid,
    ) -> ::axiom::result::Fallible<::core::option::Option<::domain::User>> {
        self.users_by_ids.read().await.get(&::core::cmp::Reverse(user_id)).cloned().into_ok()
    }

    async fn get_by_username(
        self: ::std::sync::Arc<Self>, username: ::domain::Username,
    ) -> ::axiom::result::Fallible<::core::option::Option<::domain::User>> {
        self.users_by_usernames.read().await.get(&username).cloned().into_ok()
    }

    async fn get_by_email(
        self: ::std::sync::Arc<Self>, email: ::domain::Email,
    ) -> ::axiom::result::Fallible<::core::option::Option<::domain::User>> {
        self.users_by_emails.read().await.get(&email).cloned().into_ok()
    }

    async fn contains_id(self: ::std::sync::Arc<Self>, id: ::domain::Uuid) -> ::axiom::result::Fallible<bool> {
        self.users_by_ids.read().await.contains_key(&::core::cmp::Reverse(id)).into_ok()
    }

    async fn contains_username(
        self: ::std::sync::Arc<Self>, username: ::domain::Username,
    ) -> ::axiom::result::Fallible<bool> {
        self.users_by_usernames.read().await.contains_key(&username).into_ok()
    }

    async fn contains_email(self: ::std::sync::Arc<Self>, email: ::domain::Email) -> ::axiom::result::Fallible<bool> {
        self.users_by_emails.read().await.contains_key(&email).into_ok()
    }

    async fn search(
        self: ::std::sync::Arc<Self>, filter: UserRepositorySearchFilter,
    ) -> ::axiom::result::Fallible<::std::vec::Vec<::domain::User>> {
        use self::string::StringSliceExt as _;

        let filter_query = filter.query.as_deref().map(str::trim).map(str::to_lowercase);

        let apply_filter_query = move |user: &&::domain::User| {
            if let ::core::option::Option::Some(ref query) = filter_query {
                user.username.is_subsequence(query)
                    || user.email.is_subsequence(query)
                    || user.full_name.is_subsequence(query)
            } else {
                true
            }
        };

        let filter_statuses = filter.statuses.as_ref().map(|statuses| statuses.iter().collect::<::std::collections::HashSet<_>>());

        let apply_filter_statuses = move |user: &&::domain::User| {
            if let ::core::option::Option::Some(ref statuses) = filter_statuses {
                statuses.contains(&&user.statuses.last().into())
            } else {
                true
            }
        };

        let filter_roles = filter.roles.as_ref().map(|roles| roles.iter().collect::<::std::collections::HashSet<_>>());

        let apply_filter_roles = move |user: &&::domain::User| {
            if let ::core::option::Option::Some(ref roles) = filter_roles {
                roles.contains(&&user.role.into())
            } else {
                true
            }
        };

        let apply_filter = move |user: &&::domain::User| apply_filter_query(user) && apply_filter_statuses(user) && apply_filter_roles(user);

        self.users_by_ids.read().await.values()
            .filter(apply_filter)
            .cloned()
            .collect::<::std::vec::Vec<_>>()
            .into_ok()
    }

    async fn view(self: ::std::sync::Arc<Self>) -> ::axiom::result::Fallible<::std::vec::Vec<::domain::User>> {
        self.users_by_ids.read().await.values()
            .cloned()
            .collect::<::std::vec::Vec<_>>()
            .into_ok()
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

mod hash {
    #[cfg(not(any(feature = "ahash", feature = "fxhash", feature = "metrohash", feature = "seahash")))]
    pub type BuildHasher = ::std::hash::RandomState;

    #[cfg(feature = "ahash")]
    pub type BuildHasher = ::ahash::RandomState;

    #[cfg(feature = "fxhash")]
    pub type BuildHasher = ::fxhash::FxBuildHasher;

    #[cfg(feature = "metrohash")]
    pub type BuildHasher = ::metrohash::MetroBuildHasher;

    #[cfg(feature = "seahash")]
    pub type BuildHasher = ::std::hash::BuildHasherDefault<::seahash::SeaHasher>;
}

mod string {
    pub trait StringSliceExt {
        fn is_subsequence(&self, needle: &str) -> bool;
    }

    impl StringSliceExt for str {
        fn is_subsequence(&self, needle: &str) -> bool
        {
            let mut heystack = self.chars();

            for needle_chr in needle.chars() {
                match heystack.find(|&heystack_chr| heystack_chr == needle_chr) {
                    ::core::option::Option::Some(_) => continue,
                    ::core::option::Option::None => return false,
                }
            }

            true
        }
    }
}
