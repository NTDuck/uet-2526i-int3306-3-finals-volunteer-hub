#![allow(clippy::type_complexity)]

#[cfg(any(
    all(feature = "ahash", any(feature = "fxhash", feature = "metrohash", feature = "seahash")),
    all(feature = "fxhash", any(feature = "ahash", feature = "metrohash", feature = "seahash")),
    all(feature = "metrohash", any(feature = "ahash", feature = "fxhash", feature = "seahash")),
    all(feature = "seahash", any(feature = "ahash", feature = "fxhash", feature = "metrohash")),
))]
::core::compile_error!("The following feature flags are mutually exclusive: `ahash`, `fxhash`, `metrohash`, `seahash`");

use ::axiom::prelude::*;
use ::use_cases::gateways::*;
// use ::rayon::prelude::*;

/// Since implementations of `::domain::Uuid` preserves order, consider using
/// `::std::vec::Vec<_>` for performance gains.

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
        self.events_by_ids
            .read()
            .await
            .get(&::core::cmp::Reverse(event_id))
            .cloned()
            .into_ok()
    }

    async fn contains_id(self: ::std::sync::Arc<Self>, event_id: ::domain::Uuid) -> ::axiom::result::Fallible<bool> {
        self.events_by_ids
            .read()
            .await
            .contains_key(&::core::cmp::Reverse(event_id))
            .into_ok()
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

        let filter_statuses = filter
            .statuses
            .as_ref()
            .map(|statuses| statuses.iter().collect::<::std::collections::HashSet<_>>());

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
                ::core::ops::Range {
                    start: ::core::option::Option::Some(start),
                    end: ::core::option::Option::Some(end),
                } => event_timestamp >= start && event_timestamp <= end,
                ::core::ops::Range {
                    start: ::core::option::Option::Some(start),
                    end: ::core::option::Option::None,
                } => event_timestamp <= start,
                ::core::ops::Range {
                    start: ::core::option::Option::None,
                    end: ::core::option::Option::Some(end),
                } => event_timestamp >= end,
                _ => true,
            }
        };

        let apply_filter = move |event: &&::domain::Event| {
            apply_filter_query(event) && apply_filter_statuses(event) && apply_filter_timestamps(event)
        };

        self.events_by_ids
            .read()
            .await
            .values()
            .filter(apply_filter)
            .cloned()
            .collect::<::std::vec::Vec<_>>()
            .into_ok()
    }

    async fn view(self: ::std::sync::Arc<Self>) -> ::axiom::result::Fallible<::std::vec::Vec<::domain::Event>> {
        self.events_by_ids
            .read()
            .await
            .values()
            .cloned()
            .collect::<::std::vec::Vec<_>>()
            .into_ok()
    }
}

// TODO: Implement cache & daemon
// TODO: Switch to `BinaryHeap`
#[derive(::bon::Builder)]
pub struct InMemoryExponentialDecayEventRecommender {
    pub limit: ::core::primitive::u64,
    pub λ: ::core::primitive::f64,

    #[builder(default, with = |value: ::std::collections::HashMap<::domain::Uuid, ::axiom::time::Timestamp, self::hash::BuildHasher>| ::tokio::sync::RwLock::new(value))]
    pub approved_timestamps_by_event_ids: ::tokio::sync::RwLock<
        ::std::collections::HashMap<::domain::Uuid, ::axiom::time::Timestamp, self::hash::BuildHasher>,
    >,

    #[builder(default, with = |value: ::std::collections::HashMap<::domain::Uuid, ::std::vec::Vec<(::domain::Uuid, ::axiom::time::Timestamp)>, self::hash::BuildHasher>| ::tokio::sync::RwLock::new(value))]
    pub posted_user_ids_and_timestamps_by_event_ids: ::tokio::sync::RwLock<
        ::std::collections::HashMap<
            ::domain::Uuid,
            ::std::vec::Vec<(::domain::Uuid, ::axiom::time::Timestamp)>,
            self::hash::BuildHasher,
        >,
    >,

    #[builder(default, with = |value: ::std::collections::HashMap<::domain::Uuid, ::std::vec::Vec<(::domain::Uuid, ::axiom::time::Timestamp)>, self::hash::BuildHasher>| ::tokio::sync::RwLock::new(value))]
    pub subscribed_user_ids_and_timestamps_by_event_ids: ::tokio::sync::RwLock<
        ::std::collections::HashMap<
            ::domain::Uuid,
            ::std::vec::Vec<(::domain::Uuid, ::axiom::time::Timestamp)>,
            self::hash::BuildHasher,
        >,
    >,

    #[builder(default, with = |value: ::std::collections::HashMap<::domain::Uuid, ::std::vec::Vec<(::domain::Uuid, ::axiom::time::Timestamp)>, self::hash::BuildHasher>| ::tokio::sync::RwLock::new(value))]
    pub reacted_user_ids_and_timestamps_by_event_ids: ::tokio::sync::RwLock<
        ::std::collections::HashMap<
            ::domain::Uuid,
            ::std::vec::Vec<(::domain::Uuid, ::axiom::time::Timestamp)>,
            self::hash::BuildHasher,
        >,
    >,

    #[builder(default, with = |value: ::std::collections::HashMap<::domain::Uuid, ::std::vec::Vec<(::domain::Uuid, ::axiom::time::Timestamp)>, self::hash::BuildHasher>| ::tokio::sync::RwLock::new(value))]
    pub commented_user_ids_and_timestamps_by_event_ids: ::tokio::sync::RwLock<
        ::std::collections::HashMap<
            ::domain::Uuid,
            ::std::vec::Vec<(::domain::Uuid, ::axiom::time::Timestamp)>,
            self::hash::BuildHasher,
        >,
    >,
}

#[async_trait]
impl EventRecommender for InMemoryExponentialDecayEventRecommender {
    async fn track_approved(self: ::std::sync::Arc<Self>, event_id: ::domain::Uuid) -> ::axiom::result::Fallible {
        self.approved_timestamps_by_event_ids
            .write()
            .await
            .insert(event_id, ::axiom::time::Timestamp::now());

        ::axiom::result::Fallible::Ok(())
    }

    async fn untrack_approved(self: ::std::sync::Arc<Self>, event_id: ::domain::Uuid) -> ::axiom::result::Fallible {
        self.approved_timestamps_by_event_ids.write().await.remove(&event_id);

        ::axiom::result::Fallible::Ok(())
    }

    async fn track_posted(
        self: ::std::sync::Arc<Self>, event_id: ::domain::Uuid, user_id: ::domain::Uuid,
    ) -> ::axiom::result::Fallible {
        self.posted_user_ids_and_timestamps_by_event_ids
            .write()
            .await
            .entry(event_id)
            .or_default()
            .push((user_id, ::axiom::time::Timestamp::now()));

        ::axiom::result::Fallible::Ok(())
    }

    async fn untrack_posted(
        self: ::std::sync::Arc<Self>, event_id: ::domain::Uuid, user_id_: ::domain::Uuid,
    ) -> ::axiom::result::Fallible {
        if let ::std::collections::hash_map::Entry::Occupied(mut entry) =
            self.posted_user_ids_and_timestamps_by_event_ids.write().await.entry(event_id)
        {
            entry.get_mut().retain(|(user_id, _)| *user_id != user_id_);
            if entry.get().is_empty() {
                entry.remove();
            }
        }

        ::axiom::result::Fallible::Ok(())
    }

    async fn track_subscribed(
        self: ::std::sync::Arc<Self>, event_id: ::domain::Uuid, user_id: ::domain::Uuid,
    ) -> ::axiom::result::Fallible {
        self.subscribed_user_ids_and_timestamps_by_event_ids
            .write()
            .await
            .entry(event_id)
            .or_default()
            .push((user_id, ::axiom::time::Timestamp::now()));

        ::axiom::result::Fallible::Ok(())
    }

    async fn track_reacted(
        self: ::std::sync::Arc<Self>, event_id: ::domain::Uuid, user_id: ::domain::Uuid,
    ) -> ::axiom::result::Fallible {
        self.reacted_user_ids_and_timestamps_by_event_ids
            .write()
            .await
            .entry(event_id)
            .or_default()
            .push((user_id, ::axiom::time::Timestamp::now()));

        ::axiom::result::Fallible::Ok(())
    }

    async fn track_commented(
        self: ::std::sync::Arc<Self>, event_id: ::domain::Uuid, user_id: ::domain::Uuid,
    ) -> ::axiom::result::Fallible {
        self.commented_user_ids_and_timestamps_by_event_ids
            .write()
            .await
            .entry(event_id)
            .or_default()
            .push((user_id, ::axiom::time::Timestamp::now()));

        ::axiom::result::Fallible::Ok(())
    }

    async fn untrack_subscribed(
        self: ::std::sync::Arc<Self>, event_id: ::domain::Uuid, user_id_: ::domain::Uuid,
    ) -> ::axiom::result::Fallible {
        if let ::std::collections::hash_map::Entry::Occupied(mut entry) = self
            .subscribed_user_ids_and_timestamps_by_event_ids
            .write()
            .await
            .entry(event_id)
        {
            entry.get_mut().retain(|(user_id, _)| *user_id != user_id_);
            if entry.get().is_empty() {
                entry.remove();
            }
        }

        ::axiom::result::Fallible::Ok(())
    }

    async fn untrack_reacted(
        self: ::std::sync::Arc<Self>, event_id: ::domain::Uuid, user_id_: ::domain::Uuid,
    ) -> ::axiom::result::Fallible {
        if let ::std::collections::hash_map::Entry::Occupied(mut entry) =
            self.reacted_user_ids_and_timestamps_by_event_ids.write().await.entry(event_id)
        {
            entry.get_mut().retain(|(user_id, _)| *user_id != user_id_);
            if entry.get().is_empty() {
                entry.remove();
            }
        }

        ::axiom::result::Fallible::Ok(())
    }

    async fn untrack_commented(
        self: ::std::sync::Arc<Self>, event_id: ::domain::Uuid, user_id_: ::domain::Uuid,
    ) -> ::axiom::result::Fallible {
        if let ::std::collections::hash_map::Entry::Occupied(mut entry) = self
            .commented_user_ids_and_timestamps_by_event_ids
            .write()
            .await
            .entry(event_id)
        {
            entry.get_mut().retain(|(user_id, _)| *user_id != user_id_);
            if entry.get().is_empty() {
                entry.remove();
            }
        }

        ::axiom::result::Fallible::Ok(())
    }

    async fn view_recently_approved_ids(
        self: ::std::sync::Arc<Self>,
    ) -> ::axiom::result::Fallible<::std::vec::Vec<::domain::Uuid>> {
        let mut event_ids_and_scores = self
            .approved_timestamps_by_event_ids
            .read()
            .await
            .iter()
            .map(|(&event_id, &timestamp)| (event_id, self.score(timestamp)))
            .collect::<::std::vec::Vec<_>>();

        event_ids_and_scores.sort_by(|(_, lhs_score), (_, rhs_score)| rhs_score.total_cmp(lhs_score));

        event_ids_and_scores
            .into_iter()
            .map(|(event_id, _)| event_id)
            .take(self.limit as ::core::primitive::usize)
            .collect::<::std::vec::Vec<_>>()
            .into_ok()
    }

    async fn view_recently_posted_ids(
        self: ::std::sync::Arc<Self>,
    ) -> ::axiom::result::Fallible<::std::vec::Vec<::domain::Uuid>> {
        let mut scores_by_event_ids = ::std::collections::HashMap::<_, _, self::hash::BuildHasher>::with_hasher(
            ::core::default::Default::default(),
        );

        self.posted_user_ids_and_timestamps_by_event_ids
            .read()
            .await
            .iter()
            .map(|(&event_id, user_ids_and_timestamps)| {
                (
                    event_id,
                    user_ids_and_timestamps
                        .iter()
                        .map(|(_, timestamp)| self.score(*timestamp))
                        .sum::<::core::primitive::f64>(),
                )
            })
            .for_each(|(event_id, scores_)| {
                scores_by_event_ids
                    .entry(event_id)
                    .and_modify(|scores| *scores += scores_)
                    .or_insert(scores_);
            });

        let mut event_ids_and_scores = scores_by_event_ids.into_iter().collect::<::std::vec::Vec<_>>();

        event_ids_and_scores.sort_by(|(_, lhs_score), (_, rhs_score)| rhs_score.total_cmp(lhs_score));

        event_ids_and_scores
            .into_iter()
            .map(|(event_id, _)| event_id)
            .take(self.limit as ::core::primitive::usize)
            .collect::<::std::vec::Vec<_>>()
            .into_ok()
    }

    async fn view_trending_ids(
        self: ::std::sync::Arc<Self>,
    ) -> ::axiom::result::Fallible<::std::vec::Vec<::domain::Uuid>> {
        let mut scores_by_event_ids = ::std::collections::HashMap::<_, _, self::hash::BuildHasher>::with_hasher(
            ::core::default::Default::default(),
        );

        self.posted_user_ids_and_timestamps_by_event_ids
            .read()
            .await
            .iter()
            .chain(self.subscribed_user_ids_and_timestamps_by_event_ids.read().await.iter())
            .chain(self.reacted_user_ids_and_timestamps_by_event_ids.read().await.iter())
            .chain(self.commented_user_ids_and_timestamps_by_event_ids.read().await.iter())
            .map(|(&event_id, user_ids_and_timestamps)| {
                (
                    event_id,
                    user_ids_and_timestamps
                        .iter()
                        .map(|(_, timestamp)| self.score(*timestamp))
                        .sum::<::core::primitive::f64>(),
                )
            })
            .for_each(|(event_id, scores_)| {
                scores_by_event_ids
                    .entry(event_id)
                    .and_modify(|scores| *scores += scores_)
                    .or_insert(scores_);
            });

        let mut event_ids_and_scores = scores_by_event_ids.into_iter().collect::<::std::vec::Vec<_>>();

        event_ids_and_scores.sort_by(|(_, lhs_score), (_, rhs_score)| rhs_score.total_cmp(lhs_score));

        event_ids_and_scores
            .into_iter()
            .map(|(event_id, _)| event_id)
            .take(self.limit as ::core::primitive::usize)
            .collect::<::std::vec::Vec<_>>()
            .into_ok()
    }

    async fn view_personalized_ids(
        self: ::std::sync::Arc<Self>, user_id_: ::domain::Uuid,
    ) -> ::axiom::result::Fallible<::std::vec::Vec<::domain::Uuid>> {
        let mut scores_by_event_ids = ::std::collections::HashMap::<_, _, self::hash::BuildHasher>::with_hasher(
            ::core::default::Default::default(),
        );

        self.posted_user_ids_and_timestamps_by_event_ids
            .read()
            .await
            .iter()
            .chain(self.subscribed_user_ids_and_timestamps_by_event_ids.read().await.iter())
            .chain(self.reacted_user_ids_and_timestamps_by_event_ids.read().await.iter())
            .chain(self.commented_user_ids_and_timestamps_by_event_ids.read().await.iter())
            .map(|(&event_id, user_ids_and_timestamps)| {
                (
                    event_id,
                    user_ids_and_timestamps
                        .iter()
                        .filter(|(user_id, _)| *user_id == user_id_)
                        .map(|(_, timestamp)| self.score(*timestamp))
                        .sum::<::core::primitive::f64>(),
                )
            })
            .for_each(|(event_id, scores_)| {
                scores_by_event_ids
                    .entry(event_id)
                    .and_modify(|scores| *scores += scores_)
                    .or_insert(scores_);
            });

        let mut event_ids_and_scores = scores_by_event_ids.into_iter().collect::<::std::vec::Vec<_>>();

        event_ids_and_scores.sort_by(|(_, lhs_score), (_, rhs_score)| rhs_score.total_cmp(lhs_score));

        event_ids_and_scores
            .into_iter()
            .map(|(event_id, _)| event_id)
            .take(self.limit as ::core::primitive::usize)
            .collect::<::std::vec::Vec<_>>()
            .into_ok()
    }
}

impl InMemoryExponentialDecayEventRecommender {
    fn score(&self, timestamp: ::axiom::time::Timestamp) -> ::core::primitive::f64 {
        let secs =
            ::axiom::time::Timestamp::now().signed_duration_since(timestamp).num_seconds() as ::core::primitive::f64;

        (-self.λ * secs).exp()
    }
}

#[derive(::bon::Builder)]
pub struct GenericEventExporter {
    event_repository: ::std::sync::Arc<dyn EventRepository + ::core::marker::Send + ::core::marker::Sync>,
}

#[async_trait]
impl EventExporter for GenericEventExporter {
    async fn export_as_csv(self: ::std::sync::Arc<Self>) -> ::axiom::result::Fallible<::axiom::bytes::Bytes> {
        let mut writer = ::csv::Writer::from_writer(::std::vec::Vec::new());

        writer.write_record(["id", "statuses", "name", "description", "categories", "location", "image-url"])?;

        ::std::sync::Arc::clone(&self.event_repository)
            .view()
            .await?
            .into_iter()
            .map(::core::convert::Into::<self::serde::Event>::into)
            .try_for_each(|event| writer.serialize(event))?;

        writer.flush()?;

        writer.into_inner()?.into_t::<::axiom::bytes::Bytes>().into_ok()
    }

    async fn export_as_json(self: ::std::sync::Arc<Self>) -> ::axiom::result::Fallible<::axiom::bytes::Bytes> {
        let events = ::std::sync::Arc::clone(&self.event_repository)
            .view()
            .await?
            .into_iter()
            .map(::core::convert::Into::<self::serde::Event>::into)
            .collect::<::std::vec::Vec<_>>();

        ::serde_json::to_string(&events)?.into_t::<::axiom::bytes::Bytes>().into_ok()
    }
}

#[derive(::bon::Builder)]
pub struct InMemoryEventRegistrationRepository {
    #[builder(default, with = |value: ::std::collections::BTreeMap<::core::cmp::Reverse<::domain::Uuid>, ::domain::EventRegistration>| ::tokio::sync::RwLock::new(value))]
    registrations_by_ids: ::tokio::sync::RwLock<
        ::std::collections::BTreeMap<::core::cmp::Reverse<::domain::Uuid>, ::domain::EventRegistration>,
    >,

    #[builder(default, with = |value: ::std::collections::HashMap<(::domain::Uuid, ::domain::Uuid), ::domain::EventRegistration, self::hash::BuildHasher>| ::tokio::sync::RwLock::new(value))]
    registrations_by_event_and_volunteer_ids: ::tokio::sync::RwLock<
        ::std::collections::HashMap<
            (::domain::Uuid, ::domain::Uuid),
            ::domain::EventRegistration,
            self::hash::BuildHasher,
        >,
    >,
}

#[async_trait]
impl EventRegistrationRepository for InMemoryEventRegistrationRepository {
    async fn save(
        self: ::std::sync::Arc<Self>, registration: ::domain::EventRegistration,
    ) -> ::axiom::result::Fallible {
        self.registrations_by_ids
            .write()
            .await
            .insert(::core::cmp::Reverse(registration.id), registration.clone());
        self.registrations_by_event_and_volunteer_ids
            .write()
            .await
            .insert((registration.event_id, registration.volunteer_id), registration.clone());

        ::axiom::result::Fallible::Ok(())
    }

    async fn get_by_id(
        self: ::std::sync::Arc<Self>, id: ::domain::Uuid,
    ) -> ::axiom::result::Fallible<::core::option::Option<::domain::EventRegistration>> {
        self.registrations_by_ids
            .read()
            .await
            .get(&::core::cmp::Reverse(id))
            .cloned()
            .into_ok()
    }

    async fn get_by_event_and_volunteer_id(
        self: ::std::sync::Arc<Self>, event_id: ::domain::Uuid, user_id: ::domain::Uuid,
    ) -> ::axiom::result::Fallible<::core::option::Option<::domain::EventRegistration>> {
        self.registrations_by_event_and_volunteer_ids
            .read()
            .await
            .get(&(event_id, user_id))
            .cloned()
            .into_ok()
    }

    async fn view_by_event_id(
        self: ::std::sync::Arc<Self>, event_id_: ::domain::Uuid,
    ) -> ::axiom::result::Fallible<::std::vec::Vec<::domain::EventRegistration>> {
        self.registrations_by_event_and_volunteer_ids
            .read()
            .await
            .iter()
            .filter(|((event_id, _), _)| *event_id == event_id_)
            .map(|(_, event_registration)| event_registration)
            .cloned()
            .collect::<::std::vec::Vec<_>>()
            .into_ok()
    }

    async fn view_by_volunteer_id(
        self: ::std::sync::Arc<Self>, volunteer_id_: ::domain::Uuid,
    ) -> ::axiom::result::Fallible<::std::vec::Vec<::domain::EventRegistration>> {
        self.registrations_by_event_and_volunteer_ids
            .read()
            .await
            .iter()
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
    event_posts_by_ids:
        ::tokio::sync::RwLock<::std::collections::BTreeMap<::core::cmp::Reverse<::domain::Uuid>, ::domain::EventPost>>,
}

#[async_trait]
impl EventPostRepository for InMemoryEventPostRepository {
    async fn save(self: ::std::sync::Arc<Self>, post: ::domain::EventPost) -> ::axiom::result::Fallible {
        self.event_posts_by_ids
            .write()
            .await
            .insert(::core::cmp::Reverse(post.id), post.clone());

        ::axiom::result::Fallible::Ok(())
    }

    async fn remove(self: ::std::sync::Arc<Self>, post_id: ::domain::Uuid) -> ::axiom::result::Fallible {
        self.event_posts_by_ids.write().await.remove(&::core::cmp::Reverse(post_id));

        ::axiom::result::Fallible::Ok(())
    }

    async fn get_by_id(
        self: ::std::sync::Arc<Self>, post_id: ::domain::Uuid,
    ) -> ::axiom::result::Fallible<::core::option::Option<::domain::EventPost>> {
        self.event_posts_by_ids
            .read()
            .await
            .get(&::core::cmp::Reverse(post_id))
            .cloned()
            .into_ok()
    }

    async fn view_by_event_id(
        self: ::std::sync::Arc<Self>, event_id_: ::domain::Uuid,
    ) -> ::axiom::result::Fallible<::std::vec::Vec<::domain::EventPost>> {
        self.event_posts_by_ids
            .read()
            .await
            .values()
            .filter(|&&::domain::EventPost { event_id, .. }| event_id == event_id_)
            .cloned()
            .collect::<::std::vec::Vec<_>>()
            .into_ok()
    }
}

#[derive(::bon::Builder)]
pub struct InMemoryEventPostReactionRepository {
    #[builder(default, with = |value: ::std::collections::BTreeMap<::core::cmp::Reverse<::domain::Uuid>, ::domain::EventPostReaction>| ::tokio::sync::RwLock::new(value))]
    reactions_by_ids: ::tokio::sync::RwLock<
        ::std::collections::BTreeMap<::core::cmp::Reverse<::domain::Uuid>, ::domain::EventPostReaction>,
    >,

    #[builder(default, with = |value: ::std::collections::HashMap<(::domain::Uuid, ::domain::Uuid), ::domain::EventPostReaction, self::hash::BuildHasher>| ::tokio::sync::RwLock::new(value))]
    reactions_by_post_and_user_ids: ::tokio::sync::RwLock<
        ::std::collections::HashMap<
            (::domain::Uuid, ::domain::Uuid),
            ::domain::EventPostReaction,
            self::hash::BuildHasher,
        >,
    >,
}

#[async_trait]
impl EventPostReactionRepository for InMemoryEventPostReactionRepository {
    async fn save(self: ::std::sync::Arc<Self>, reaction: ::domain::EventPostReaction) -> ::axiom::result::Fallible {
        self.reactions_by_ids
            .write()
            .await
            .insert(::core::cmp::Reverse(reaction.id), reaction.clone());
        self.reactions_by_post_and_user_ids
            .write()
            .await
            .insert((reaction.post_id, reaction.author_id), reaction.clone());

        ::axiom::result::Fallible::Ok(())
    }

    async fn remove(self: ::std::sync::Arc<Self>, reaction_id: ::domain::Uuid) -> ::axiom::result::Fallible {
        self.reactions_by_ids.write().await.remove(&::core::cmp::Reverse(reaction_id));
        self.reactions_by_post_and_user_ids
            .write()
            .await
            .remove(&(reaction_id, reaction_id));

        ::axiom::result::Fallible::Ok(())
    }

    async fn get_by_id(
        self: ::std::sync::Arc<Self>, reaction_id: ::domain::Uuid,
    ) -> ::axiom::result::Fallible<::core::option::Option<::domain::EventPostReaction>> {
        self.reactions_by_ids
            .read()
            .await
            .get(&::core::cmp::Reverse(reaction_id))
            .cloned()
            .into_ok()
    }

    async fn get_by_post_and_user_id(
        self: ::std::sync::Arc<Self>, post_id: ::domain::Uuid, user_id: ::domain::Uuid,
    ) -> ::axiom::result::Fallible<::core::option::Option<::domain::EventPostReaction>> {
        self.reactions_by_post_and_user_ids
            .read()
            .await
            .get(&(post_id, user_id))
            .cloned()
            .into_ok()
    }

    async fn contains_post_and_user_id(
        self: ::std::sync::Arc<Self>, post_id: ::domain::Uuid, user_id: ::domain::Uuid,
    ) -> ::axiom::result::Fallible<bool> {
        self.reactions_by_post_and_user_ids
            .read()
            .await
            .contains_key(&(post_id, user_id))
            .into_ok()
    }

    async fn view_by_post_id(
        self: ::std::sync::Arc<Self>, post_id_: ::domain::Uuid,
    ) -> ::axiom::result::Fallible<::std::vec::Vec<::domain::EventPostReaction>> {
        self.reactions_by_ids
            .read()
            .await
            .values()
            .filter(|&&::domain::EventPostReaction { post_id, .. }| post_id == post_id_)
            .cloned()
            .collect::<::std::vec::Vec<_>>()
            .into_ok()
    }

    async fn count_by_post_id(
        self: ::std::sync::Arc<Self>, post_id_: ::domain::Uuid,
    ) -> ::axiom::result::Fallible<::core::primitive::u64> {
        (self
            .reactions_by_ids
            .read()
            .await
            .values()
            .filter(|&&::domain::EventPostReaction { post_id, .. }| post_id == post_id_)
            .count() as ::core::primitive::u64)
            .into_ok()
    }
}

#[derive(::bon::Builder)]
pub struct InMemoryEventPostCommentRepository {
    #[builder(default, with = |value: ::std::collections::BTreeMap<::core::cmp::Reverse<::domain::Uuid>, ::domain::EventPostComment>| ::tokio::sync::RwLock::new(value))]
    comments_by_ids: ::tokio::sync::RwLock<
        ::std::collections::BTreeMap<::core::cmp::Reverse<::domain::Uuid>, ::domain::EventPostComment>,
    >,

    #[builder(default, with = |value: ::std::collections::HashMap<(::domain::Uuid, ::domain::Uuid), ::domain::EventPostComment, self::hash::BuildHasher>| ::tokio::sync::RwLock::new(value))]
    comments_by_post_and_user_ids: ::tokio::sync::RwLock<
        ::std::collections::HashMap<
            (::domain::Uuid, ::domain::Uuid),
            ::domain::EventPostComment,
            self::hash::BuildHasher,
        >,
    >,
}

#[async_trait]
impl EventPostCommentRepository for InMemoryEventPostCommentRepository {
    async fn save(self: ::std::sync::Arc<Self>, comment: ::domain::EventPostComment) -> ::axiom::result::Fallible {
        self.comments_by_ids
            .write()
            .await
            .insert(::core::cmp::Reverse(comment.id), comment.clone());
        self.comments_by_post_and_user_ids
            .write()
            .await
            .insert((comment.post_id, comment.author_id), comment.clone());

        ::axiom::result::Fallible::Ok(())
    }

    async fn remove(self: ::std::sync::Arc<Self>, comment_id: ::domain::Uuid) -> ::axiom::result::Fallible {
        self.comments_by_ids.write().await.remove(&::core::cmp::Reverse(comment_id));
        self.comments_by_post_and_user_ids
            .write()
            .await
            .remove(&(comment_id, comment_id));

        ::axiom::result::Fallible::Ok(())
    }

    async fn get_by_id(
        self: ::std::sync::Arc<Self>, comment_id: ::domain::Uuid,
    ) -> ::axiom::result::Fallible<::core::option::Option<::domain::EventPostComment>> {
        self.comments_by_ids
            .read()
            .await
            .get(&::core::cmp::Reverse(comment_id))
            .cloned()
            .into_ok()
    }

    async fn contains_id(self: ::std::sync::Arc<Self>, comment_id: ::domain::Uuid) -> ::axiom::result::Fallible<bool> {
        self.comments_by_ids
            .read()
            .await
            .contains_key(&::core::cmp::Reverse(comment_id))
            .into_ok()
    }

    async fn view_by_post_and_user_id(
        self: ::std::sync::Arc<Self>, post_id_: ::domain::Uuid, user_id_: ::domain::Uuid,
    ) -> ::axiom::result::Fallible<::std::vec::Vec<::domain::EventPostComment>> {
        self.comments_by_post_and_user_ids
            .read()
            .await
            .values()
            .filter(|&&::domain::EventPostComment { post_id, author_id, .. }| {
                post_id == post_id_ && author_id == user_id_
            })
            .cloned()
            .collect::<::std::vec::Vec<_>>()
            .into_ok()
    }

    async fn view_by_post_id(
        self: ::std::sync::Arc<Self>, post_id_: ::domain::Uuid,
    ) -> ::axiom::result::Fallible<::std::vec::Vec<::domain::EventPostComment>> {
        self.comments_by_ids
            .read()
            .await
            .values()
            .filter(|&&::domain::EventPostComment { post_id, .. }| post_id == post_id_)
            .cloned()
            .collect::<::std::vec::Vec<_>>()
            .into_ok()
    }

    async fn count_by_post_id(
        self: ::std::sync::Arc<Self>, post_id_: ::domain::Uuid,
    ) -> ::axiom::result::Fallible<::core::primitive::u64> {
        (self
            .comments_by_ids
            .read()
            .await
            .values()
            .filter(|&&::domain::EventPostComment { post_id, .. }| post_id == post_id_)
            .count() as ::core::primitive::u64)
            .into_ok()
    }
}

#[derive(::bon::Builder)]
pub struct InMemoryUserRepository {
    #[builder(default, with = |value: ::std::collections::BTreeMap<::core::cmp::Reverse<::domain::Uuid>, ::domain::User>| ::tokio::sync::RwLock::new(value))]
    users_by_ids:
        ::tokio::sync::RwLock<::std::collections::BTreeMap<::core::cmp::Reverse<::domain::Uuid>, ::domain::User>>,

    #[builder(default, with = |value: ::std::collections::HashMap<::domain::Username, ::domain::User, self::hash::BuildHasher>| ::tokio::sync::RwLock::new(value))]
    users_by_usernames:
        ::tokio::sync::RwLock<::std::collections::HashMap<::domain::Username, ::domain::User, self::hash::BuildHasher>>,

    #[builder(default, with = |value: ::std::collections::HashMap<::domain::Email, ::domain::User, self::hash::BuildHasher>| ::tokio::sync::RwLock::new(value))]
    users_by_emails:
        ::tokio::sync::RwLock<::std::collections::HashMap<::domain::Email, ::domain::User, self::hash::BuildHasher>>,
}

#[async_trait]
impl UserRepository for InMemoryUserRepository {
    async fn save(self: ::std::sync::Arc<Self>, user: ::domain::User) -> ::axiom::result::Fallible {
        self.users_by_ids
            .write()
            .await
            .insert(::core::cmp::Reverse(user.id), user.clone());
        self.users_by_usernames
            .write()
            .await
            .insert(user.username.clone(), user.clone());
        self.users_by_emails.write().await.insert(user.email.clone(), user.clone());

        ::axiom::result::Fallible::Ok(())
    }

    async fn get_by_id(
        self: ::std::sync::Arc<Self>, user_id: ::domain::Uuid,
    ) -> ::axiom::result::Fallible<::core::option::Option<::domain::User>> {
        self.users_by_ids
            .read()
            .await
            .get(&::core::cmp::Reverse(user_id))
            .cloned()
            .into_ok()
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

        let filter_statuses = filter
            .statuses
            .as_ref()
            .map(|statuses| statuses.iter().collect::<::std::collections::HashSet<_>>());

        let apply_filter_statuses = move |user: &&::domain::User| {
            if let ::core::option::Option::Some(ref statuses) = filter_statuses {
                statuses.contains(&&user.statuses.last().into())
            } else {
                true
            }
        };

        let filter_roles = filter
            .roles
            .as_ref()
            .map(|roles| roles.iter().collect::<::std::collections::HashSet<_>>());

        let apply_filter_roles = move |user: &&::domain::User| {
            if let ::core::option::Option::Some(ref roles) = filter_roles {
                roles.contains(&&user.role.into())
            } else {
                true
            }
        };

        let apply_filter = move |user: &&::domain::User| {
            apply_filter_query(user) && apply_filter_statuses(user) && apply_filter_roles(user)
        };

        self.users_by_ids
            .read()
            .await
            .values()
            .filter(apply_filter)
            .cloned()
            .collect::<::std::vec::Vec<_>>()
            .into_ok()
    }

    async fn view(self: ::std::sync::Arc<Self>) -> ::axiom::result::Fallible<::std::vec::Vec<::domain::User>> {
        self.users_by_ids
            .read()
            .await
            .values()
            .cloned()
            .collect::<::std::vec::Vec<_>>()
            .into_ok()
    }
}

#[derive(::bon::Builder)]
pub struct GenericUserExporter {
    user_repository: ::std::sync::Arc<dyn UserRepository + ::core::marker::Send + ::core::marker::Sync>,
}

#[async_trait]
impl UserExporter for GenericUserExporter {
    async fn export_volunteers_as_csv(
        self: ::std::sync::Arc<Self>,
    ) -> ::axiom::result::Fallible<::axiom::bytes::Bytes> {
        let mut writer = ::csv::Writer::from_writer(::std::vec::Vec::new());

        writer.write_record(["id", "role", "statuses", "username", "email", "full-name", "avatar-url"])?;

        ::std::sync::Arc::clone(&self.user_repository)
            .view()
            .await?
            .into_iter()
            .filter(|::domain::User { role, .. }| ::core::matches!(role, ::domain::UserRole::Volunteer))
            .map(::core::convert::Into::<self::serde::User>::into)
            .try_for_each(|user| writer.serialize(user))?;

        writer.flush()?;

        writer.into_inner()?.into_t::<::axiom::bytes::Bytes>().into_ok()
    }

    async fn export_volunteers_as_json(
        self: ::std::sync::Arc<Self>,
    ) -> ::axiom::result::Fallible<::axiom::bytes::Bytes> {
        let users = ::std::sync::Arc::clone(&self.user_repository)
            .view()
            .await?
            .into_iter()
            .filter(|::domain::User { role, .. }| ::core::matches!(role, ::domain::UserRole::Volunteer))
            .map(::core::convert::Into::<self::serde::User>::into)
            .collect::<::std::vec::Vec<_>>();

        ::serde_json::to_string(&users)?.into_t::<::axiom::bytes::Bytes>().into_ok()
    }
}

#[derive(::bon::Builder)]
pub struct MockMediaRepository {
    #[builder(into)]
    url: ::axiom::string::String,
}

#[async_trait]
impl MediaRepository for MockMediaRepository {
    async fn verify(self: ::std::sync::Arc<Self>, _bytes: ::axiom::bytes::Bytes) -> ::axiom::result::Fallible<bool> {
        true.into_ok()
    }

    async fn save(
        self: ::std::sync::Arc<Self>, _bytes: ::axiom::bytes::Bytes,
    ) -> ::axiom::result::Fallible<::axiom::string::String> {
        self.url.clone().into_ok()
    }

    async fn remove(self: ::std::sync::Arc<Self>, _url: ::axiom::string::String) -> ::axiom::result::Fallible {
        ::axiom::result::Fallible::Ok(())
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
        ::uuid::Uuid::now_v7().into_bytes().into_t::<::domain::Uuid>().into_ok()
    }
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
        let mut buffer = [0u8; 45];

        ::uuid::Uuid::from_bytes(*uuid)
            .as_urn()
            .encode_lower(&mut buffer)
            .to_string()
            .into_t::<::axiom::string::String>()
            .into_ok()
    }

    async fn parse(
        self: ::std::sync::Arc<Self>, urn: ::axiom::string::String,
    ) -> ::axiom::result::Fallible<::domain::Uuid> {
        ::uuid::Uuid::parse_str(&urn)?.into_bytes().into_t::<::domain::Uuid>().into_ok()
    }
}

pub struct Rfc2822TimestampCodec;

#[::bon::bon]
impl Rfc2822TimestampCodec {
    #[builder(builder_type(vis = "pub"))]
    fn new() -> Self {
        Self
    }
}

#[async_trait]
impl TimestampCodec for Rfc2822TimestampCodec {
    async fn format(
        self: ::std::sync::Arc<Self>, timestamp: ::axiom::time::Timestamp,
    ) -> ::axiom::result::Fallible<::axiom::string::String> {
        timestamp.to_rfc2822().into_t::<::axiom::string::String>().into_ok()
    }

    async fn parse(
        self: ::std::sync::Arc<Self>, timestamp: ::axiom::string::String,
    ) -> ::axiom::result::Fallible<::axiom::time::Timestamp> {
        ::chrono::DateTime::parse_from_rfc2822(&timestamp)?
            .with_timezone(&::chrono::Utc)
            .into_t::<::axiom::time::Timestamp>()
            .into_ok()
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
        self: ::std::sync::Arc<Self>, payload: ::use_cases::gateways::AuthTokenPayload,
    ) -> ::axiom::result::Fallible<::axiom::string::String> {
        use ::jwt::SignWithKey as _;

        payload
            .into_t::<self::serde::AuthTokenPayload>()
            .sign_with_key(&self.key)?
            .into_t::<::axiom::string::String>()
            .into_ok()
    }

    async fn get_payload(
        self: ::std::sync::Arc<Self>, token: ::axiom::string::String,
    ) -> ::axiom::result::Fallible<::core::option::Option<::use_cases::gateways::AuthTokenPayload>> {
        ::jwt::VerifyWithKey::<self::serde::AuthTokenPayload>::verify_with_key(&*token, &self.key)
            .ok()
            .map(::core::convert::Into::into)
            .into_ok()
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
        self.context
            .hash_password(password.as_bytes(), &salt)?
            .to_string()
            .into_t::<::axiom::string::String>()
            .into_ok()
    }

    async fn verify(
        self: ::std::sync::Arc<Self>, password: ::domain::Password, digest: ::domain::PasswordDigest,
    ) -> ::axiom::result::Fallible<bool> {
        use ::argon2::PasswordVerifier as _;

        let digest = ::argon2::password_hash::PasswordHash::new(&digest)?;
        self.context.verify_password(password.as_bytes(), &digest).is_ok().into_ok()
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

mod serde {
    #[derive(::serde::Serialize, ::serde::Deserialize, ::bon::Builder)]
    #[serde(rename_all = "camelCase")]
    #[builder(on(_, into))]
    pub struct AuthTokenPayload {
        user_id: Uuid,
        user_role: UserRole,
        expiry_timestamp: ::axiom::time::Timestamp,
    }

    impl ::core::convert::From<AuthTokenPayload> for ::use_cases::gateways::AuthTokenPayload {
        fn from(value: AuthTokenPayload) -> Self {
            Self::builder()
                .user_id(value.user_id)
                .user_role(value.user_role)
                .expiry_timestamp(value.expiry_timestamp)
                .build()
        }
    }

    impl ::core::convert::From<::use_cases::gateways::AuthTokenPayload> for AuthTokenPayload {
        fn from(value: ::use_cases::gateways::AuthTokenPayload) -> Self {
            Self::builder()
                .user_id(value.user_id)
                .user_role(value.user_role)
                .expiry_timestamp(value.expiry_timestamp)
                .build()
        }
    }

    #[derive(::serde::Serialize, ::bon::Builder)]
    #[serde(rename_all = "camelCase")]
    #[builder(on(_, into))]
    pub struct Event {
        pub id: Uuid,

        pub statuses: ::std::vec::Vec<EventStatus>,

        pub name: ::axiom::string::String,
        pub description: ::axiom::string::String,
        pub categories: ::std::vec::Vec<::axiom::string::String>,
        pub location: ::axiom::string::String,

        pub image_url: ::axiom::string::String,
    }

    impl ::core::convert::From<::domain::Event> for Event {
        fn from(value: ::domain::Event) -> Self {
            Self::builder()
                .id(value.id)
                .statuses(
                    value
                        .statuses
                        .into_iter()
                        .map(::core::convert::Into::into)
                        .collect::<::std::vec::Vec<_>>(),
                )
                .name(value.name)
                .description(value.description)
                .categories(
                    value
                        .categories
                        .into_iter()
                        .map(::core::convert::Into::into)
                        .collect::<::std::vec::Vec<_>>(),
                )
                .location(value.location)
                .image_url(value.image_url)
                .build()
        }
    }

    #[derive(::serde::Serialize, ::serde::Deserialize)]
    #[serde(rename_all = "kebab-case", rename_all_fields = "kebab-case")]
    pub enum EventStatus {
        Created {
            created_by_manager_id: Uuid,
            created_at: ::axiom::time::Timestamp,
        },
        Updated {
            updated_by_manager_id: Uuid,
            updated_at: ::axiom::time::Timestamp,
        },
        Approved {
            approved_by_administrator_id: Uuid,
            approved_at: ::axiom::time::Timestamp,
        },
        Rejected {
            rejected_by_administrator_id: Uuid,
            rejected_at: ::axiom::time::Timestamp,
        },
    }

    impl ::core::convert::From<::domain::EventStatus> for EventStatus {
        fn from(value: ::domain::EventStatus) -> Self {
            match value {
                ::domain::EventStatus::Created { created_by_manager_id, created_at } => Self::Created {
                    created_by_manager_id: created_by_manager_id.into(),
                    created_at,
                },
                ::domain::EventStatus::Updated { updated_by_manager_id, updated_at } => Self::Updated {
                    updated_by_manager_id: updated_by_manager_id.into(),
                    updated_at,
                },
                ::domain::EventStatus::Approved {
                    approved_by_administrator_id,
                    approved_at,
                } => Self::Approved {
                    approved_by_administrator_id: approved_by_administrator_id.into(),
                    approved_at,
                },
                ::domain::EventStatus::Rejected {
                    rejected_by_administrator_id,
                    rejected_at,
                } => Self::Rejected {
                    rejected_by_administrator_id: rejected_by_administrator_id.into(),
                    rejected_at,
                },
            }
        }
    }

    impl ::core::convert::From<EventStatus> for ::domain::EventStatus {
        fn from(value: EventStatus) -> Self {
            match value {
                EventStatus::Created { created_by_manager_id, created_at } => Self::Created {
                    created_by_manager_id: created_by_manager_id.into(),
                    created_at,
                },
                EventStatus::Updated { updated_by_manager_id, updated_at } => Self::Updated {
                    updated_by_manager_id: updated_by_manager_id.into(),
                    updated_at,
                },
                EventStatus::Approved {
                    approved_by_administrator_id,
                    approved_at,
                } => Self::Approved {
                    approved_by_administrator_id: approved_by_administrator_id.into(),
                    approved_at,
                },
                EventStatus::Rejected {
                    rejected_by_administrator_id,
                    rejected_at,
                } => Self::Rejected {
                    rejected_by_administrator_id: rejected_by_administrator_id.into(),
                    rejected_at,
                },
            }
        }
    }

    #[derive(::serde::Serialize, ::bon::Builder)]
    #[serde(rename_all = "camelCase")]
    #[builder(on(_, into))]
    pub struct User {
        pub id: Uuid,

        pub role: UserRole,
        pub statuses: ::std::vec::Vec<UserStatus>,

        pub username: ::axiom::string::String,
        pub email: ::axiom::string::String,
        pub full_name: ::axiom::string::String,

        #[builder(required)]
        pub avatar_url: ::core::option::Option<::axiom::string::String>,
    }

    impl ::core::convert::From<::domain::User> for User {
        fn from(value: ::domain::User) -> Self {
            Self::builder()
                .id(value.id)
                .role(value.role)
                .statuses(
                    value
                        .statuses
                        .into_iter()
                        .map(::core::convert::Into::into)
                        .collect::<::std::vec::Vec<_>>(),
                )
                .username(value.username)
                .email(value.email)
                .full_name(value.full_name)
                .avatar_url(value.avatar_url)
                .build()
        }
    }

    #[derive(::serde::Serialize, ::serde::Deserialize)]
    #[serde(untagged, rename_all = "kebab-case")]
    pub enum UserRole {
        Volunteer,
        EventManager,
        Administrator,
    }

    impl ::core::convert::From<::domain::UserRole> for UserRole {
        fn from(value: ::domain::UserRole) -> Self {
            match value {
                ::domain::UserRole::Volunteer => Self::Volunteer,
                ::domain::UserRole::EventManager => Self::EventManager,
                ::domain::UserRole::Administrator => Self::Administrator,
            }
        }
    }

    impl ::core::convert::From<UserRole> for ::domain::UserRole {
        fn from(value: UserRole) -> Self {
            match value {
                UserRole::Volunteer => Self::Volunteer,
                UserRole::EventManager => Self::EventManager,
                UserRole::Administrator => Self::Administrator,
            }
        }
    }

    #[derive(::serde::Serialize, ::serde::Deserialize)]
    #[serde(rename_all = "kebab-case", rename_all_fields = "kebab-case")]
    pub enum UserStatus {
        Created {
            created_at: ::axiom::time::Timestamp,
        },
        Updated {
            updated_at: ::axiom::time::Timestamp,
        },
        Suspended {
            suspended_by_administrator_id: Uuid,
            suspended_at: ::axiom::time::Timestamp,
        },
        Unsuspended {
            unsuspended_by_administrator_id: Uuid,
            unsuspended_at: ::axiom::time::Timestamp,
        },
    }

    impl ::core::convert::From<::domain::UserStatus> for UserStatus {
        fn from(value: ::domain::UserStatus) -> Self {
            match value {
                ::domain::UserStatus::Created { created_at } => Self::Created { created_at },
                ::domain::UserStatus::Updated { updated_at } => Self::Updated { updated_at },
                ::domain::UserStatus::Suspended {
                    suspended_by_administrator_id,
                    suspended_at,
                } => Self::Suspended {
                    suspended_by_administrator_id: suspended_by_administrator_id.into(),
                    suspended_at,
                },
                ::domain::UserStatus::Unsuspended {
                    unsuspended_by_administrator_id,
                    unsuspended_at,
                } => Self::Unsuspended {
                    unsuspended_by_administrator_id: unsuspended_by_administrator_id.into(),
                    unsuspended_at,
                },
            }
        }
    }

    impl ::core::convert::From<UserStatus> for ::domain::UserStatus {
        fn from(value: UserStatus) -> Self {
            match value {
                UserStatus::Created { created_at } => Self::Created { created_at },
                UserStatus::Updated { updated_at } => Self::Updated { updated_at },
                UserStatus::Suspended {
                    suspended_by_administrator_id,
                    suspended_at,
                } => Self::Suspended {
                    suspended_by_administrator_id: suspended_by_administrator_id.into(),
                    suspended_at,
                },
                UserStatus::Unsuspended {
                    unsuspended_by_administrator_id,
                    unsuspended_at,
                } => Self::Unsuspended {
                    unsuspended_by_administrator_id: unsuspended_by_administrator_id.into(),
                    unsuspended_at,
                },
            }
        }
    }

    #[derive(::serde::Serialize, ::serde::Deserialize)]
    #[serde(transparent)]
    pub struct Uuid([u8; 16]);

    #[::bon::bon]
    impl Uuid {
        #[builder]
        pub fn new(value: [u8; 16]) -> Self {
            Self(value)
        }
    }

    impl ::core::ops::Deref for Uuid {
        type Target = [u8; 16];

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl ::core::convert::From<::domain::Uuid> for Uuid {
        fn from(value: ::domain::Uuid) -> Self {
            Self::builder().value(*value).build()
        }
    }

    impl ::core::convert::From<Uuid> for ::domain::Uuid {
        fn from(value: Uuid) -> Self {
            Self::builder().value(*value).build()
        }
    }
}

mod string {
    pub trait StringSliceExt {
        fn is_subsequence(&self, needle: &str) -> bool;
    }

    impl StringSliceExt for str {
        fn is_subsequence(&self, needle: &str) -> bool {
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
