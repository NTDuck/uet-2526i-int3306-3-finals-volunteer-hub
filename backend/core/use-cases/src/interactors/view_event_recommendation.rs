use ::axiom::prelude::*;
use ::futures::prelude::*;

use crate::boundaries::*;
use crate::gateways::*;

#[derive(::bon::Builder)]
pub struct ViewEventRecommendationInteractor {
    event_repository: ::std::sync::Arc<dyn EventRepository + ::core::marker::Send + ::core::marker::Sync>,
    user_repository: ::std::sync::Arc<dyn UserRepository + ::core::marker::Send + ::core::marker::Sync>,

    uuid_codec: ::std::sync::Arc<dyn UuidCodec + ::core::marker::Send + ::core::marker::Sync>,
    auth_token_generator:
        ::std::sync::Arc<dyn AuthenticationTokenGenerator + ::core::marker::Send + ::core::marker::Sync>,
}

#[async_trait]
impl ViewEventRecommendationBoundary for ViewEventRecommendationInteractor {
    async fn apply(
        self: ::std::sync::Arc<Self>, request: ViewEventRecommendationRequest,
    ) -> ::axiom::result::Fallible<ViewEventRecommendationResponse> {
        match ::std::sync::Arc::clone(&self.auth_token_generator).get_payload(request.token).await? {
            ::core::option::Option::None =>
                return ::axiom::err!(ViewEventRecommendation @ AuthenticationTokenInvalid),
            ::core::option::Option::Some
            (AuthenticationTokenPayload { user_id, expiry_timestamp, .. }) => {
                if expiry_timestamp < ::axiom::time::Timestamp::now() {
                    return ::axiom::err!(ViewEventRecommendation @ AuthenticationTokenExpired);
                }

                if !::std::sync::Arc::clone(&self.user_repository).contains_id(user_id).await? {
                    return ::axiom::err!(ViewEventRecommendation @ UserNotFound);
                }
            },
        };

        let events = match request.r#type {
            crate::boundaries::ViewEventRecommendationRecommendationType::RecentlyPublished =>
                ::std::sync::Arc::clone(&self.event_repository)
                    .view_recently_approved(request.limit)
                    .await?,
            crate::boundaries::ViewEventRecommendationRecommendationType::RecentlyPosted =>
                ::std::sync::Arc::clone(&self.event_repository)
                    .view_recently_posted(request.limit)
                    .await?,
            crate::boundaries::ViewEventRecommendationRecommendationType::Trending =>
                ::std::sync::Arc::clone(&self.event_repository)
                    .view_trending(request.limit)
                    .await?,
        };

        let events = ::futures::stream::iter(events)
            .filter_map(|event| {
                let uuid_codec = ::std::sync::Arc::clone(&self.uuid_codec);

                async move {
                    ViewEventRecommendationEvent::build_from(event)
                        .with_uuid_codec(uuid_codec)
                        .try_build().await
                        .ok()
                }
            })
            .collect::<::std::vec::Vec<_>>().await;

        let response = ViewEventRecommendationOkResponse::builder().events(events).build();
        ::axiom::ok!(ViewEventRecommendation @ response)
    }
}
