use ::async_trait::async_trait;

use crate::boundaries::*;
use crate::gateways::*;

#[derive(::bon::Builder)]
pub struct ViewEventRecommendationInteractor {
    event_repository: ::std::sync::Arc<dyn EventRepository + ::core::marker::Send + ::core::marker::Sync>,

    uuid_codec: ::std::sync::Arc<dyn UuidCodec + ::core::marker::Send + ::core::marker::Sync>,
    auth_token_generator:
        ::std::sync::Arc<dyn AuthenticationTokenGenerator + ::core::marker::Send + ::core::marker::Sync>,
}

#[async_trait]
impl ViewEventRecommendationBoundary for ViewEventRecommendationInteractor {
    async fn apply(
        self: ::std::sync::Arc<Self>, request: ViewEventRecommendationRequest,
    ) -> ::axiom::result::Fallible<ViewEventRecommendationResponse> {
        if !::std::sync::Arc::clone(&self.auth_token_generator)
            .verify(request.token)
            .await?
        {
            return ::axiom::result::Fallible::Ok(ViewEventRecommendationResponse::Err(::std::vec![
                ViewEventRecommendationErrResponse::AuthenticationTokenInvalid,
            ]));
        }

        // Rust's type inference fails here
        let events: ::std::vec::Vec<::domain::Event> = match request.r#type {
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

        let events = ::futures::future::try_join_all(events.into_iter().map(|event| {
            let uuid_codec = ::std::sync::Arc::clone(&self.uuid_codec);

            async move {
                ::futures::future::ok::<_, ::axiom::result::Error>(
                    crate::boundaries::ViewEventRecommendationEvent::builder()
                        .id(uuid_codec.format(event.id).await?)
                        .status(*event.statuses.last())
                        .name(event.name)
                        .categories(event.categories.into_vec())
                        .build(),
                )
                .await
            }
        }))
        .await?;

        let response = ViewEventRecommendationOkResponse::builder().events(events).build();

        ::axiom::result::Fallible::Ok(ViewEventRecommendationResponse::Ok(response))
    }
}
