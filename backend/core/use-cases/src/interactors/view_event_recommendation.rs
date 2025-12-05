use ::axiom::prelude::*;
use ::futures::prelude::*;

use crate::boundaries::*;
use crate::gateways::*;

#[derive(::bon::Builder)]
pub struct ViewEventRecommendationInteractor {
    event_repository: ::std::sync::Arc<dyn EventRepository + ::core::marker::Send + ::core::marker::Sync>,
    event_recommender: ::std::sync::Arc<dyn EventRecommender + ::core::marker::Send + ::core::marker::Sync>,
    user_repository: ::std::sync::Arc<dyn UserRepository + ::core::marker::Send + ::core::marker::Sync>,

    uuid_codec: ::std::sync::Arc<dyn UuidCodec + ::core::marker::Send + ::core::marker::Sync>,
    timestamp_codec: ::std::sync::Arc<dyn TimestampCodec + ::core::marker::Send + ::core::marker::Sync>,
    auth_token_generator:
        ::std::sync::Arc<dyn AuthTokenGenerator + ::core::marker::Send + ::core::marker::Sync>,
}

#[async_trait]
impl ViewEventRecommendationBoundary for ViewEventRecommendationInteractor {
    async fn apply(
        self: ::std::sync::Arc<Self>, request: Request,
    ) -> ::axiom::result::Fallible<Response> {
        let actor_id = match ::std::sync::Arc::clone(&self.auth_token_generator)
            .get_payload(request.token)
            .await?
        {
            ::core::option::Option::None => return super::err!(AuthenticationTokenInvalid),
            ::core::option::Option::Some(AuthenticationTokenPayload { user_id, expiry_timestamp, .. }) => {
                if expiry_timestamp < ::axiom::time::Timestamp::now() {
                    return super::err!(AuthenticationTokenExpired);
                }

                if !::std::sync::Arc::clone(&self.user_repository).contains_id(user_id).await? {
                    return super::err!(UserNotFound);
                }

                user_id
            },
        };

        let events_ids = match request.r#type {
            RecommendationType::RecentlyPublished =>
                ::std::sync::Arc::clone(&self.event_recommender)
                    .view_recently_approved_ids()
                    .await?,
            RecommendationType::RecentlyPosted =>
                ::std::sync::Arc::clone(&self.event_recommender).view_recently_posted_ids().await?,
            RecommendationType::Trending =>
                ::std::sync::Arc::clone(&self.event_recommender).view_trending_ids().await?,
            RecommendationType::Personalized =>
                ::std::sync::Arc::clone(&self.event_recommender)
                    .view_personalized_ids(actor_id)
                    .await?,
        };

        let events = events_ids
            .into_stream()
            .then(|event_id| {
                let event_repository = ::std::sync::Arc::clone(&self.event_repository);

                async move {
                    ::std::sync::Arc::clone(&event_repository).get_by_id(event_id).await
                }
            })
            .filter_map(|transposable| async move { transposable.transpose() })
            .and_then(|event| {
                let uuid_codec = ::std::sync::Arc::clone(&self.uuid_codec);
                let timestamp_codec = ::std::sync::Arc::clone(&self.timestamp_codec);

                async move {
                    Event::build_from(event)
                        .with_uuid_codec(::std::sync::Arc::clone(&uuid_codec))
                        .with_timestamp_codec(::std::sync::Arc::clone(&timestamp_codec))
                        .try_build()
                        .await
                }
            })
            .try_collect::<::std::vec::Vec<_>>()
            .await?;

        let response = OkResponse::builder().events(events).build();
        super::ok!(response)
    }
}

type Request = ViewEventRecommendationRequest;
type Response = ViewEventRecommendationResponse;
type OkResponse = ViewEventRecommendationOkResponse;
type ErrResponse = ViewEventRecommendationErrResponse;
type RecommendationType = ViewEventRecommendationRecommendationType;
type Event = ViewEventRecommendationEvent;
