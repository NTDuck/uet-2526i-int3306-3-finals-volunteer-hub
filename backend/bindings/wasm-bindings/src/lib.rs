use ::axiom::prelude::*;
use ::infrastructures::*;
use ::use_cases::boundaries::*;
use ::use_cases::gateways::*;
use ::use_cases::interactors::*;
use ::wasm_bindgen::prelude::*;

/// A `Promise<<...>OkResponse>` returned by a non-static method is either
/// fulfilled with `<...>OkResponse` or rejected with `<...>ErrResponse[]`. A
/// `<...>ErrResponse` satisfies `{ error: <...>, message: <...>, data: { ... }
/// }`.
#[wasm_bindgen]
#[derive(::bon::Builder)]
pub struct Application {
    #[wasm_bindgen(skip)]
    create_comment_boundary:
        ::std::sync::Arc<dyn CreateEventPostCommentBoundary + ::core::marker::Send + ::core::marker::Sync>,

    #[wasm_bindgen(skip)]
    create_event_boundary: ::std::sync::Arc<dyn CreateEventBoundary + ::core::marker::Send + ::core::marker::Sync>,

    #[wasm_bindgen(skip)]
    create_post_boundary: ::std::sync::Arc<dyn CreateEventPostBoundary + ::core::marker::Send + ::core::marker::Sync>,

    #[wasm_bindgen(skip)]
    create_reaction_boundary:
        ::std::sync::Arc<dyn CreateEventPostReactionBoundary + ::core::marker::Send + ::core::marker::Sync>,

    #[wasm_bindgen(skip)]
    export_events_boundary: ::std::sync::Arc<dyn ExportEventsBoundary + ::core::marker::Send + ::core::marker::Sync>,

    #[wasm_bindgen(skip)]
    export_volunteers_boundary:
        ::std::sync::Arc<dyn ExportVolunteersBoundary + ::core::marker::Send + ::core::marker::Sync>,

    #[wasm_bindgen(skip)]
    moderate_event_boundary: ::std::sync::Arc<dyn ModerateEventBoundary + ::core::marker::Send + ::core::marker::Sync>,

    #[wasm_bindgen(skip)]
    moderate_event_registration_boundary:
        ::std::sync::Arc<dyn ModerateEventRegistrationBoundary + ::core::marker::Send + ::core::marker::Sync>,

    #[wasm_bindgen(skip)]
    moderate_user_boundary: ::std::sync::Arc<dyn ModerateUserBoundary + ::core::marker::Send + ::core::marker::Sync>,

    #[wasm_bindgen(skip)]
    remove_comment_boundary:
        ::std::sync::Arc<dyn RemoveEventPostCommentBoundary + ::core::marker::Send + ::core::marker::Sync>,

    #[wasm_bindgen(skip)]
    remove_event_boundary: ::std::sync::Arc<dyn RemoveEventBoundary + ::core::marker::Send + ::core::marker::Sync>,

    #[wasm_bindgen(skip)]
    remove_post_boundary: ::std::sync::Arc<dyn RemoveEventPostBoundary + ::core::marker::Send + ::core::marker::Sync>,

    #[wasm_bindgen(skip)]
    remove_reaction_boundary:
        ::std::sync::Arc<dyn RemoveEventPostReactionBoundary + ::core::marker::Send + ::core::marker::Sync>,

    #[wasm_bindgen(skip)]
    sign_in_boundary: ::std::sync::Arc<dyn SignInBoundary + ::core::marker::Send + ::core::marker::Sync>,

    #[wasm_bindgen(skip)]
    sign_up_boundary: ::std::sync::Arc<dyn SignUpBoundary + ::core::marker::Send + ::core::marker::Sync>,

    #[wasm_bindgen(skip)]
    subscribe_to_event_boundary:
        ::std::sync::Arc<dyn SubscribeToEventBoundary + ::core::marker::Send + ::core::marker::Sync>,

    #[wasm_bindgen(skip)]
    unsubscribe_from_event_boundary:
        ::std::sync::Arc<dyn UnsubscribeFromEventBoundary + ::core::marker::Send + ::core::marker::Sync>,

    #[wasm_bindgen(skip)]
    update_comment_boundary:
        ::std::sync::Arc<dyn UpdateEventPostCommentBoundary + ::core::marker::Send + ::core::marker::Sync>,

    #[wasm_bindgen(skip)]
    update_event_boundary: ::std::sync::Arc<dyn UpdateEventBoundary + ::core::marker::Send + ::core::marker::Sync>,

    #[wasm_bindgen(skip)]
    update_post_boundary: ::std::sync::Arc<dyn UpdateEventPostBoundary + ::core::marker::Send + ::core::marker::Sync>,

    #[wasm_bindgen(skip)]
    view_event_boundary: ::std::sync::Arc<dyn ViewEventBoundary + ::core::marker::Send + ::core::marker::Sync>,

    #[wasm_bindgen(skip)]
    view_event_channel_boundary:
        ::std::sync::Arc<dyn ViewEventChannelBoundary + ::core::marker::Send + ::core::marker::Sync>,

    #[wasm_bindgen(skip)]
    view_event_history_boundary:
        ::std::sync::Arc<dyn ViewEventHistoryBoundary + ::core::marker::Send + ::core::marker::Sync>,

    #[wasm_bindgen(skip)]
    view_event_recommendation_boundary:
        ::std::sync::Arc<dyn ViewEventRecommendationBoundary + ::core::marker::Send + ::core::marker::Sync>,

    #[wasm_bindgen(skip)]
    view_event_volunteers_boundary:
        ::std::sync::Arc<dyn ViewEventVolunteersBoundary + ::core::marker::Send + ::core::marker::Sync>,

    #[wasm_bindgen(skip)]
    view_events_boundary: ::std::sync::Arc<dyn ViewEventsBoundary + ::core::marker::Send + ::core::marker::Sync>,

    #[wasm_bindgen(skip)]
    view_published_event_boundary:
        ::std::sync::Arc<dyn ViewPublishedEventBoundary + ::core::marker::Send + ::core::marker::Sync>,

    #[wasm_bindgen(skip)]
    view_post_boundary: ::std::sync::Arc<dyn ViewEventPostBoundary + ::core::marker::Send + ::core::marker::Sync>,

    #[wasm_bindgen(skip)]
    view_published_events_boundary:
        ::std::sync::Arc<dyn ViewPublishedEventsBoundary + ::core::marker::Send + ::core::marker::Sync>,

    #[wasm_bindgen(skip)]
    view_self_profile_boundary:
        ::std::sync::Arc<dyn ViewSelfProfileBoundary + ::core::marker::Send + ::core::marker::Sync>,

    #[wasm_bindgen(skip)]
    view_user_boundary: ::std::sync::Arc<dyn ViewUserBoundary + ::core::marker::Send + ::core::marker::Sync>,

    #[wasm_bindgen(skip)]
    view_users_boundary: ::std::sync::Arc<dyn ViewUsersBoundary + ::core::marker::Send + ::core::marker::Sync>,
}

#[wasm_bindgen]
impl Application {
    #[wasm_bindgen(js_name = withContext)]
    pub async fn with_profile(context: ApplicationContext) -> Promise<Self> {
        Gateways::try_from(context)
            .map(::core::convert::Into::<Self>::into)
            .inspect_err(|error| ::tracing::error!("{error}")) // Saves hours of debugging
            .into_promise()
    }

    #[wasm_bindgen(js_name = createComment)]
    pub async fn create_comment(
        &self, request: CreateEventPostCommentRequest,
    ) -> Promise<CreateEventPostCommentOkResponse> {
        Self::proxy()
            .intercept(|request| ::std::sync::Arc::clone(&self.create_comment_boundary).apply(request))
            .apply(request)
            .await
    }

    #[wasm_bindgen(js_name = createEvent)]
    pub async fn create_event(&self, request: CreateEventRequest) -> Promise<CreateEventOkResponse> {
        Self::proxy()
            .intercept(|request| ::std::sync::Arc::clone(&self.create_event_boundary).apply(request))
            .apply(request)
            .await
    }

    #[wasm_bindgen(js_name = createPost)]
    pub async fn create_post(&self, request: CreateEventPostRequest) -> Promise<CreateEventPostOkResponse> {
        Self::proxy()
            .intercept(|request| ::std::sync::Arc::clone(&self.create_post_boundary).apply(request))
            .apply(request)
            .await
    }

    #[wasm_bindgen(js_name = createReaction)]
    pub async fn create_reaction(
        &self, request: CreateEventPostReactionRequest,
    ) -> Promise<CreateEventPostReactionOkResponse> {
        Self::proxy()
            .intercept(|request| ::std::sync::Arc::clone(&self.create_reaction_boundary).apply(request))
            .apply(request)
            .await
    }

    #[wasm_bindgen(js_name = exportEvents)]
    pub async fn export_events(&self, request: ExportEventsRequest) -> Promise<ExportEventsOkResponse> {
        Self::proxy()
            .intercept(|request| ::std::sync::Arc::clone(&self.export_events_boundary).apply(request))
            .apply(request)
            .await
    }

    #[wasm_bindgen(js_name = exportVolunteers)]
    pub async fn export_volunteers(&self, request: ExportVolunteersRequest) -> Promise<ExportVolunteersOkResponse> {
        Self::proxy()
            .intercept(|request| ::std::sync::Arc::clone(&self.export_volunteers_boundary).apply(request))
            .apply(request)
            .await
    }

    #[wasm_bindgen(js_name = moderateEvent)]
    pub async fn moderate_event(&self, request: ModerateEventRequest) -> Promise<ModerateEventOkResponse> {
        Self::proxy()
            .intercept(|request| ::std::sync::Arc::clone(&self.moderate_event_boundary).apply(request))
            .apply(request)
            .await
    }

    #[wasm_bindgen(js_name = moderateEventRegistration)]
    pub async fn moderate_event_registration(
        &self, request: ModerateEventRegistrationRequest,
    ) -> Promise<ModerateEventRegistrationOkResponse> {
        Self::proxy()
            .intercept(|request| ::std::sync::Arc::clone(&self.moderate_event_registration_boundary).apply(request))
            .apply(request)
            .await
    }

    #[wasm_bindgen(js_name = moderateUser)]
    pub async fn moderate_user(&self, request: ModerateUserRequest) -> Promise<ModerateUserOkResponse> {
        Self::proxy()
            .intercept(|request| ::std::sync::Arc::clone(&self.moderate_user_boundary).apply(request))
            .apply(request)
            .await
    }

    #[wasm_bindgen(js_name = removeComment)]
    pub async fn remove_comment(
        &self, request: RemoveEventPostCommentRequest,
    ) -> Promise<RemoveEventPostCommentOkResponse> {
        Self::proxy()
            .intercept(|request| ::std::sync::Arc::clone(&self.remove_comment_boundary).apply(request))
            .apply(request)
            .await
    }

    #[wasm_bindgen(js_name = removeEvent)]
    pub async fn remove_event(&self, request: RemoveEventRequest) -> Promise<RemoveEventOkResponse> {
        Self::proxy()
            .intercept(|request| ::std::sync::Arc::clone(&self.remove_event_boundary).apply(request))
            .apply(request)
            .await
    }

    #[wasm_bindgen(js_name = removePost)]
    pub async fn remove_post(&self, request: RemoveEventPostRequest) -> Promise<RemoveEventPostOkResponse> {
        Self::proxy()
            .intercept(|request| ::std::sync::Arc::clone(&self.remove_post_boundary).apply(request))
            .apply(request)
            .await
    }

    #[wasm_bindgen(js_name = removeReaction)]
    pub async fn remove_reaction(
        &self, request: RemoveEventPostReactionRequest,
    ) -> Promise<RemoveEventPostReactionOkResponse> {
        Self::proxy()
            .intercept(|request| ::std::sync::Arc::clone(&self.remove_reaction_boundary).apply(request))
            .apply(request)
            .await
    }

    #[wasm_bindgen(js_name = signIn)]
    pub async fn sign_in(&self, request: SignInRequest) -> Promise<SignInOkResponse> {
        Self::proxy()
            .intercept(|request| ::std::sync::Arc::clone(&self.sign_in_boundary).apply(request))
            .apply(request)
            .await
    }

    #[wasm_bindgen(js_name = signUp)]
    pub async fn sign_up(&self, request: SignUpRequest) -> Promise<SignUpOkResponse> {
        Self::proxy()
            .intercept(|request| ::std::sync::Arc::clone(&self.sign_up_boundary).apply(request))
            .apply(request)
            .await
    }

    #[wasm_bindgen(js_name = subscribeToEvent)]
    pub async fn subscribe_to_event(&self, request: SubscribeToEventRequest) -> Promise<SubscribeToEventOkResponse> {
        Self::proxy()
            .intercept(|request| ::std::sync::Arc::clone(&self.subscribe_to_event_boundary).apply(request))
            .apply(request)
            .await
    }

    #[wasm_bindgen(js_name = unsubscribeFromEvent)]
    pub async fn unsubscribe_from_event(
        &self, request: UnsubscribeFromEventRequest,
    ) -> Promise<UnsubscribeFromEventOkResponse> {
        Self::proxy()
            .intercept(|request| ::std::sync::Arc::clone(&self.unsubscribe_from_event_boundary).apply(request))
            .apply(request)
            .await
    }

    #[wasm_bindgen(js_name = updateComment)]
    pub async fn update_comment(
        &self, request: UpdateEventPostCommentRequest,
    ) -> Promise<UpdateEventPostCommentOkResponse> {
        Self::proxy()
            .intercept(|request| ::std::sync::Arc::clone(&self.update_comment_boundary).apply(request))
            .apply(request)
            .await
    }

    #[wasm_bindgen(js_name = updateEvent)]
    pub async fn update_event(&self, request: UpdateEventRequest) -> Promise<UpdateEventOkResponse> {
        Self::proxy()
            .intercept(|request| ::std::sync::Arc::clone(&self.update_event_boundary).apply(request))
            .apply(request)
            .await
    }

    #[wasm_bindgen(js_name = updatePost)]
    pub async fn update_post(&self, request: UpdateEventPostRequest) -> Promise<UpdateEventPostOkResponse> {
        Self::proxy()
            .intercept(|request| ::std::sync::Arc::clone(&self.update_post_boundary).apply(request))
            .apply(request)
            .await
    }

    #[wasm_bindgen(js_name = viewEvent)]
    pub async fn view_event(&self, request: ViewEventRequest) -> Promise<ViewEventOkResponse> {
        Self::proxy()
            .intercept(|request| ::std::sync::Arc::clone(&self.view_event_boundary).apply(request))
            .apply(request)
            .await
    }

    #[wasm_bindgen(js_name = viewEventChannel)]
    pub async fn view_event_channel(&self, request: ViewEventChannelRequest) -> Promise<ViewEventChannelOkResponse> {
        Self::proxy()
            .intercept(|request| ::std::sync::Arc::clone(&self.view_event_channel_boundary).apply(request))
            .apply(request)
            .await
    }

    #[wasm_bindgen(js_name = viewEventHistory)]
    pub async fn view_event_history(&self, request: ViewEventHistoryRequest) -> Promise<ViewEventHistoryOkResponse> {
        Self::proxy()
            .intercept(|request| ::std::sync::Arc::clone(&self.view_event_history_boundary).apply(request))
            .apply(request)
            .await
    }

    #[wasm_bindgen(js_name = viewEventRecommendation)]
    pub async fn view_event_recommendation(
        &self, request: ViewEventRecommendationRequest,
    ) -> Promise<ViewEventRecommendationOkResponse> {
        Self::proxy()
            .intercept(|request| ::std::sync::Arc::clone(&self.view_event_recommendation_boundary).apply(request))
            .apply(request)
            .await
    }

    #[wasm_bindgen(js_name = viewEventVolunteers)]
    pub async fn view_event_volunteers(
        &self, request: ViewEventVolunteersRequest,
    ) -> Promise<ViewEventVolunteersOkResponse> {
        Self::proxy()
            .intercept(|request| ::std::sync::Arc::clone(&self.view_event_volunteers_boundary).apply(request))
            .apply(request)
            .await
    }

    #[wasm_bindgen(js_name = viewEvents)]
    pub async fn view_events(&self, request: ViewEventsRequest) -> Promise<ViewEventsOkResponse> {
        Self::proxy()
            .intercept(|request| ::std::sync::Arc::clone(&self.view_events_boundary).apply(request))
            .apply(request)
            .await
    }

    #[wasm_bindgen(js_name = viewPost)]
    pub async fn view_post(&self, request: ViewEventPostRequest) -> Promise<ViewEventPostOkResponse> {
        Self::proxy()
            .intercept(|request| ::std::sync::Arc::clone(&self.view_post_boundary).apply(request))
            .apply(request)
            .await
    }

    #[wasm_bindgen(js_name = viewPublishedEvent)]
    pub async fn view_published_event(
        &self, request: ViewPublishedEventRequest,
    ) -> Promise<ViewPublishedEventOkResponse> {
        Self::proxy()
            .intercept(|request| ::std::sync::Arc::clone(&self.view_published_event_boundary).apply(request))
            .apply(request)
            .await
    }

    #[wasm_bindgen(js_name = viewPublishedEvents)]
    pub async fn view_published_events(
        &self, request: ViewPublishedEventsRequest,
    ) -> Promise<ViewPublishedEventsOkResponse> {
        Self::proxy()
            .intercept(|request| ::std::sync::Arc::clone(&self.view_published_events_boundary).apply(request))
            .apply(request)
            .await
    }

    #[wasm_bindgen(js_name = viewSelfProfile)]
    pub async fn view_self_profile(&self, request: ViewSelfProfileRequest) -> Promise<ViewSelfProfileOkResponse> {
        Self::proxy()
            .intercept(|request| ::std::sync::Arc::clone(&self.view_self_profile_boundary).apply(request))
            .apply(request)
            .await
    }

    #[wasm_bindgen(js_name = viewUser)]
    pub async fn view_user(&self, request: ViewUserRequest) -> Promise<ViewUserOkResponse> {
        Self::proxy()
            .intercept(|request| ::std::sync::Arc::clone(&self.view_user_boundary).apply(request))
            .apply(request)
            .await
    }

    #[wasm_bindgen(js_name = viewUsers)]
    pub async fn view_users(&self, request: ViewUsersRequest) -> Promise<ViewUsersOkResponse> {
        Self::proxy()
            .intercept(|request| ::std::sync::Arc::clone(&self.view_users_boundary).apply(request))
            .apply(request)
            .await
    }
}

#[::bon::bon]
impl Application {
    #[builder(finish_fn(name = apply))]
    async fn proxy<F, Fut, Request, OkResponse, ErrResponse>(
        #[builder(finish_fn)] request: Request, #[builder(setters(name = intercept))] f: F,
    ) -> Promise<OkResponse>
    where
        F: ::core::ops::FnOnce(Request) -> Fut,
        Fut: ::core::future::Future<
                Output = ::axiom::result::Fallible<::core::result::Result<OkResponse, ::std::vec::Vec<ErrResponse>>>,
            > + ::core::marker::Send,
        Request: ::core::fmt::Debug + for<'de> ::serde::Deserialize<'de>,
        OkResponse: ::core::fmt::Debug + ::serde::Serialize,
        ErrResponse: ::core::fmt::Debug + ::serde::Serialize,
    {
        use ::colored::Colorize as _;

        ::tracing::debug!("{label} {request:?}", label = "[WASM-REQ]".green());

        f(request)
            .await
            .into_promise()
            .inspect(|response| ::tracing::debug!("{label} {response:?}", label = "[WASM-RES]".green()))
    }
}

impl ::core::convert::From<Gateways> for Application {
    fn from(gateways: Gateways) -> Self {
        Application::builder()
            .create_comment_boundary(::std::sync::Arc::new(
                CreateEventPostCommentInteractor::builder()
                    .event_recommender(::std::sync::Arc::clone(&gateways.event_recommender))
                    .post_repository(::std::sync::Arc::clone(&gateways.post_repository))
                    .comment_repository(::std::sync::Arc::clone(&gateways.comment_repository))
                    .user_repository(::std::sync::Arc::clone(&gateways.user_repository))
                    .media_repository(::std::sync::Arc::clone(&gateways.media_repository))
                    .uuid_generator(::std::sync::Arc::clone(&gateways.uuid_generator))
                    .uuid_codec(::std::sync::Arc::clone(&gateways.uuid_codec))
                    .auth_token_generator(::std::sync::Arc::clone(&gateways.auth_token_generator))
                    .build(),
            ))
            .create_event_boundary(::std::sync::Arc::new(
                CreateEventInteractor::builder()
                    .event_repository(::std::sync::Arc::clone(&gateways.event_repository))
                    .user_repository(::std::sync::Arc::clone(&gateways.user_repository))
                    .media_repository(::std::sync::Arc::clone(&gateways.media_repository))
                    .uuid_generator(::std::sync::Arc::clone(&gateways.uuid_generator))
                    .auth_token_generator(::std::sync::Arc::clone(&gateways.auth_token_generator))
                    .build(),
            ))
            .create_post_boundary(::std::sync::Arc::new(
                CreateEventPostInteractor::builder()
                    .event_repository(::std::sync::Arc::clone(&gateways.event_repository))
                    .event_recommender(::std::sync::Arc::clone(&gateways.event_recommender))
                    .post_repository(::std::sync::Arc::clone(&gateways.post_repository))
                    .user_repository(::std::sync::Arc::clone(&gateways.user_repository))
                    .media_repository(::std::sync::Arc::clone(&gateways.media_repository))
                    .uuid_generator(::std::sync::Arc::clone(&gateways.uuid_generator))
                    .uuid_codec(::std::sync::Arc::clone(&gateways.uuid_codec))
                    .auth_token_generator(::std::sync::Arc::clone(&gateways.auth_token_generator))
                    .build(),
            ))
            .create_reaction_boundary(::std::sync::Arc::new(
                CreateEventPostReactionInteractor::builder()
                    .event_recommender(::std::sync::Arc::clone(&gateways.event_recommender))
                    .post_repository(::std::sync::Arc::clone(&gateways.post_repository))
                    .reaction_repository(::std::sync::Arc::clone(&gateways.reaction_repository))
                    .user_repository(::std::sync::Arc::clone(&gateways.user_repository))
                    .uuid_generator(::std::sync::Arc::clone(&gateways.uuid_generator))
                    .uuid_codec(::std::sync::Arc::clone(&gateways.uuid_codec))
                    .auth_token_generator(::std::sync::Arc::clone(&gateways.auth_token_generator))
                    .build(),
            ))
            .export_events_boundary(::std::sync::Arc::new(
                ExportEventsInteractor::builder()
                    .event_exporter(::std::sync::Arc::clone(&gateways.event_exporter))
                    .user_repository(::std::sync::Arc::clone(&gateways.user_repository))
                    .auth_token_generator(::std::sync::Arc::clone(&gateways.auth_token_generator))
                    .build(),
            ))
            .export_volunteers_boundary(::std::sync::Arc::new(
                ExportVolunteersInteractor::builder()
                    .user_exporter(::std::sync::Arc::clone(&gateways.user_exporter))
                    .user_repository(::std::sync::Arc::clone(&gateways.user_repository))
                    .auth_token_generator(::std::sync::Arc::clone(&gateways.auth_token_generator))
                    .build(),
            ))
            .moderate_event_boundary(::std::sync::Arc::new(
                ModerateEventInteractor::builder()
                    .event_repository(::std::sync::Arc::clone(&gateways.event_repository))
                    .event_recommender(::std::sync::Arc::clone(&gateways.event_recommender))
                    .user_repository(::std::sync::Arc::clone(&gateways.user_repository))
                    .uuid_codec(::std::sync::Arc::clone(&gateways.uuid_codec))
                    .auth_token_generator(::std::sync::Arc::clone(&gateways.auth_token_generator))
                    .build(),
            ))
            .moderate_event_registration_boundary(::std::sync::Arc::new(
                ModerateEventRegistrationInteractor::builder()
                    .event_registration_repository(::std::sync::Arc::clone(&gateways.event_registration_repository))
                    .user_repository(::std::sync::Arc::clone(&gateways.user_repository))
                    .uuid_codec(::std::sync::Arc::clone(&gateways.uuid_codec))
                    .auth_token_generator(::std::sync::Arc::clone(&gateways.auth_token_generator))
                    .build(),
            ))
            .moderate_user_boundary(::std::sync::Arc::new(
                ModerateUserInteractor::builder()
                    .user_repository(::std::sync::Arc::clone(&gateways.user_repository))
                    .uuid_codec(::std::sync::Arc::clone(&gateways.uuid_codec))
                    .auth_token_generator(::std::sync::Arc::clone(&gateways.auth_token_generator))
                    .build(),
            ))
            .remove_comment_boundary(::std::sync::Arc::new(
                RemoveEventPostCommentInteractor::builder()
                    .event_recommender(::std::sync::Arc::clone(&gateways.event_recommender))
                    .post_repository(::std::sync::Arc::clone(&gateways.post_repository))
                    .comment_repository(::std::sync::Arc::clone(&gateways.comment_repository))
                    .user_repository(::std::sync::Arc::clone(&gateways.user_repository))
                    .uuid_codec(::std::sync::Arc::clone(&gateways.uuid_codec))
                    .auth_token_generator(::std::sync::Arc::clone(&gateways.auth_token_generator))
                    .build(),
            ))
            .remove_event_boundary(::std::sync::Arc::new(
                RemoveEventInteractor::builder()
                    .event_repository(::std::sync::Arc::clone(&gateways.event_repository))
                    .user_repository(::std::sync::Arc::clone(&gateways.user_repository))
                    .uuid_codec(::std::sync::Arc::clone(&gateways.uuid_codec))
                    .auth_token_generator(::std::sync::Arc::clone(&gateways.auth_token_generator))
                    .build(),
            ))
            .remove_post_boundary(::std::sync::Arc::new(
                RemoveEventPostInteractor::builder()
                    .event_recommender(::std::sync::Arc::clone(&gateways.event_recommender))
                    .post_repository(::std::sync::Arc::clone(&gateways.post_repository))
                    .user_repository(::std::sync::Arc::clone(&gateways.user_repository))
                    .uuid_codec(::std::sync::Arc::clone(&gateways.uuid_codec))
                    .auth_token_generator(::std::sync::Arc::clone(&gateways.auth_token_generator))
                    .build(),
            ))
            .remove_reaction_boundary(::std::sync::Arc::new(
                RemoveEventPostReactionInteractor::builder()
                    .event_recommender(::std::sync::Arc::clone(&gateways.event_recommender))
                    .post_repository(::std::sync::Arc::clone(&gateways.post_repository))
                    .reaction_repository(::std::sync::Arc::clone(&gateways.reaction_repository))
                    .user_repository(::std::sync::Arc::clone(&gateways.user_repository))
                    .uuid_codec(::std::sync::Arc::clone(&gateways.uuid_codec))
                    .auth_token_generator(::std::sync::Arc::clone(&gateways.auth_token_generator))
                    .build(),
            ))
            .sign_in_boundary(::std::sync::Arc::new(
                SignInInteractor::builder()
                    .user_repository(::std::sync::Arc::clone(&gateways.user_repository))
                    .auth_token_generator(::std::sync::Arc::clone(&gateways.auth_token_generator))
                    .password_hasher(::std::sync::Arc::clone(&gateways.password_hasher))
                    .build(),
            ))
            .sign_up_boundary(::std::sync::Arc::new(
                SignUpInteractor::builder()
                    .user_repository(::std::sync::Arc::clone(&gateways.user_repository))
                    .media_repository(::std::sync::Arc::clone(&gateways.media_repository))
                    .uuid_generator(::std::sync::Arc::clone(&gateways.uuid_generator))
                    .password_hasher(::std::sync::Arc::clone(&gateways.password_hasher))
                    .build(),
            ))
            .subscribe_to_event_boundary(::std::sync::Arc::new(
                SubscribeToEventInteractor::builder()
                    .event_repository(::std::sync::Arc::clone(&gateways.event_repository))
                    .event_recommender(::std::sync::Arc::clone(&gateways.event_recommender))
                    .event_registration_repository(::std::sync::Arc::clone(&gateways.event_registration_repository))
                    .user_repository(::std::sync::Arc::clone(&gateways.user_repository))
                    .uuid_generator(::std::sync::Arc::clone(&gateways.uuid_generator))
                    .uuid_codec(::std::sync::Arc::clone(&gateways.uuid_codec))
                    .auth_token_generator(::std::sync::Arc::clone(&gateways.auth_token_generator))
                    .build(),
            ))
            .unsubscribe_from_event_boundary(::std::sync::Arc::new(
                UnsubscribeFromEventInteractor::builder()
                    .event_recommender(::std::sync::Arc::clone(&gateways.event_recommender))
                    .event_registration_repository(::std::sync::Arc::clone(&gateways.event_registration_repository))
                    .user_repository(::std::sync::Arc::clone(&gateways.user_repository))
                    .uuid_codec(::std::sync::Arc::clone(&gateways.uuid_codec))
                    .auth_token_generator(::std::sync::Arc::clone(&gateways.auth_token_generator))
                    .build(),
            ))
            .update_comment_boundary(::std::sync::Arc::new(
                UpdateEventPostCommentInteractor::builder()
                    .comment_repository(::std::sync::Arc::clone(&gateways.comment_repository))
                    .user_repository(::std::sync::Arc::clone(&gateways.user_repository))
                    .media_repository(::std::sync::Arc::clone(&gateways.media_repository))
                    .uuid_codec(::std::sync::Arc::clone(&gateways.uuid_codec))
                    .auth_token_generator(::std::sync::Arc::clone(&gateways.auth_token_generator))
                    .build(),
            ))
            .update_event_boundary(::std::sync::Arc::new(
                UpdateEventInteractor::builder()
                    .event_repository(::std::sync::Arc::clone(&gateways.event_repository))
                    .user_repository(::std::sync::Arc::clone(&gateways.user_repository))
                    .media_repository(::std::sync::Arc::clone(&gateways.media_repository))
                    .uuid_codec(::std::sync::Arc::clone(&gateways.uuid_codec))
                    .auth_token_generator(::std::sync::Arc::clone(&gateways.auth_token_generator))
                    .build(),
            ))
            .update_post_boundary(::std::sync::Arc::new(
                UpdateEventPostInteractor::builder()
                    .post_repository(::std::sync::Arc::clone(&gateways.post_repository))
                    .user_repository(::std::sync::Arc::clone(&gateways.user_repository))
                    .media_repository(::std::sync::Arc::clone(&gateways.media_repository))
                    .uuid_codec(::std::sync::Arc::clone(&gateways.uuid_codec))
                    .auth_token_generator(::std::sync::Arc::clone(&gateways.auth_token_generator))
                    .build(),
            ))
            .view_event_channel_boundary(::std::sync::Arc::new(
                ViewEventChannelInteractor::builder()
                    .event_repository(::std::sync::Arc::clone(&gateways.event_repository))
                    .post_repository(::std::sync::Arc::clone(&gateways.post_repository))
                    .reaction_repository(::std::sync::Arc::clone(&gateways.reaction_repository))
                    .comment_repository(::std::sync::Arc::clone(&gateways.comment_repository))
                    .user_repository(::std::sync::Arc::clone(&gateways.user_repository))
                    .uuid_codec(::std::sync::Arc::clone(&gateways.uuid_codec))
                    .timestamp_codec(::std::sync::Arc::clone(&gateways.timestamp_codec))
                    .auth_token_generator(::std::sync::Arc::clone(&gateways.auth_token_generator))
                    .build(),
            ))
            .view_event_history_boundary(::std::sync::Arc::new(
                ViewEventHistoryInteractor::builder()
                    .event_repository(::std::sync::Arc::clone(&gateways.event_repository))
                    .event_registration_repository(::std::sync::Arc::clone(&gateways.event_registration_repository))
                    .user_repository(::std::sync::Arc::clone(&gateways.user_repository))
                    .uuid_codec(::std::sync::Arc::clone(&gateways.uuid_codec))
                    .timestamp_codec(::std::sync::Arc::clone(&gateways.timestamp_codec))
                    .auth_token_generator(::std::sync::Arc::clone(&gateways.auth_token_generator))
                    .build(),
            ))
            .view_event_recommendation_boundary(::std::sync::Arc::new(
                ViewEventRecommendationInteractor::builder()
                    .event_repository(::std::sync::Arc::clone(&gateways.event_repository))
                    .event_recommender(::std::sync::Arc::clone(&gateways.event_recommender))
                    .user_repository(::std::sync::Arc::clone(&gateways.user_repository))
                    .uuid_codec(::std::sync::Arc::clone(&gateways.uuid_codec))
                    .timestamp_codec(::std::sync::Arc::clone(&gateways.timestamp_codec))
                    .auth_token_generator(::std::sync::Arc::clone(&gateways.auth_token_generator))
                    .build(),
            ))
            .view_event_volunteers_boundary(::std::sync::Arc::new(
                ViewEventVolunteersInteractor::builder()
                    .event_registration_repository(::std::sync::Arc::clone(&gateways.event_registration_repository))
                    .user_repository(::std::sync::Arc::clone(&gateways.user_repository))
                    .uuid_codec(::std::sync::Arc::clone(&gateways.uuid_codec))
                    .auth_token_generator(::std::sync::Arc::clone(&gateways.auth_token_generator))
                    .build(),
            ))
            .view_event_boundary(::std::sync::Arc::new(
                ViewEventInteractor::builder()
                    .event_repository(::std::sync::Arc::clone(&gateways.event_repository))
                    .user_repository(::std::sync::Arc::clone(&gateways.user_repository))
                    .uuid_codec(::std::sync::Arc::clone(&gateways.uuid_codec))
                    .timestamp_codec(::std::sync::Arc::clone(&gateways.timestamp_codec))
                    .auth_token_generator(::std::sync::Arc::clone(&gateways.auth_token_generator))
                    .build(),
            ))
            .view_events_boundary(::std::sync::Arc::new(
                ViewEventsInteractor::builder()
                    .event_repository(::std::sync::Arc::clone(&gateways.event_repository))
                    .user_repository(::std::sync::Arc::clone(&gateways.user_repository))
                    .uuid_codec(::std::sync::Arc::clone(&gateways.uuid_codec))
                    .timestamp_codec(::std::sync::Arc::clone(&gateways.timestamp_codec))
                    .auth_token_generator(::std::sync::Arc::clone(&gateways.auth_token_generator))
                    .build(),
            ))
            .view_post_boundary(::std::sync::Arc::new(
                ViewEventPostInteractor::builder()
                    .post_repository(::std::sync::Arc::clone(&gateways.post_repository))
                    .reaction_repository(::std::sync::Arc::clone(&gateways.reaction_repository))
                    .comment_repository(::std::sync::Arc::clone(&gateways.comment_repository))
                    .user_repository(::std::sync::Arc::clone(&gateways.user_repository))
                    .uuid_codec(::std::sync::Arc::clone(&gateways.uuid_codec))
                    .timestamp_codec(::std::sync::Arc::clone(&gateways.timestamp_codec))
                    .auth_token_generator(::std::sync::Arc::clone(&gateways.auth_token_generator))
                    .build(),
            ))
            .view_published_event_boundary(::std::sync::Arc::new(
                ViewPublishedEventInteractor::builder()
                    .event_repository(::std::sync::Arc::clone(&gateways.event_repository))
                    .user_repository(::std::sync::Arc::clone(&gateways.user_repository))
                    .uuid_codec(::std::sync::Arc::clone(&gateways.uuid_codec))
                    .timestamp_codec(::std::sync::Arc::clone(&gateways.timestamp_codec))
                    .auth_token_generator(::std::sync::Arc::clone(&gateways.auth_token_generator))
                    .build(),
            ))
            .view_published_events_boundary(::std::sync::Arc::new(
                ViewPublishedEventsInteractor::builder()
                    .event_repository(::std::sync::Arc::clone(&gateways.event_repository))
                    .user_repository(::std::sync::Arc::clone(&gateways.user_repository))
                    .uuid_codec(::std::sync::Arc::clone(&gateways.uuid_codec))
                    .timestamp_codec(::std::sync::Arc::clone(&gateways.timestamp_codec))
                    .auth_token_generator(::std::sync::Arc::clone(&gateways.auth_token_generator))
                    .build(),
            ))
            .view_self_profile_boundary(::std::sync::Arc::new(
                ViewSelfProfileInteractor::builder()
                    .user_repository(::std::sync::Arc::clone(&gateways.user_repository))
                    .uuid_codec(::std::sync::Arc::clone(&gateways.uuid_codec))
                    .timestamp_codec(::std::sync::Arc::clone(&gateways.timestamp_codec))
                    .auth_token_generator(::std::sync::Arc::clone(&gateways.auth_token_generator))
                    .build(),
            ))
            .view_user_boundary(::std::sync::Arc::new(
                ViewUserInteractor::builder()
                    .user_repository(::std::sync::Arc::clone(&gateways.user_repository))
                    .uuid_codec(::std::sync::Arc::clone(&gateways.uuid_codec))
                    .timestamp_codec(::std::sync::Arc::clone(&gateways.timestamp_codec))
                    .auth_token_generator(::std::sync::Arc::clone(&gateways.auth_token_generator))
                    .build(),
            ))
            .view_users_boundary(::std::sync::Arc::new(
                ViewUsersInteractor::builder()
                    .user_repository(::std::sync::Arc::clone(&gateways.user_repository))
                    .uuid_codec(::std::sync::Arc::clone(&gateways.uuid_codec))
                    .auth_token_generator(::std::sync::Arc::clone(&gateways.auth_token_generator))
                    .build(),
            ))
            .build()
    }
}

#[wasm_bindgen]
pub struct ApplicationContext {
    #[allow(dead_code)]
    profile: Profile,

    #[wasm_bindgen(js_name = uploadFileCallable)]
    upload_file_callable: ::js_sys::Function,
}

#[wasm_bindgen]
impl ApplicationContext {
    #[wasm_bindgen(constructor)]
    pub fn new(profile: Profile, upload_file_callable: ::js_sys::Function) -> Self {
        Self { profile, upload_file_callable }
    }
}

#[derive(::bon::Builder)]
struct Gateways {
    event_repository: ::std::sync::Arc<dyn EventRepository + ::core::marker::Send + ::core::marker::Sync>,
    event_recommender: ::std::sync::Arc<dyn EventRecommender + ::core::marker::Send + ::core::marker::Sync>,
    event_exporter: ::std::sync::Arc<dyn EventExporter + ::core::marker::Send + ::core::marker::Sync>,

    event_registration_repository:
        ::std::sync::Arc<dyn EventRegistrationRepository + ::core::marker::Send + ::core::marker::Sync>,
    post_repository: ::std::sync::Arc<dyn EventPostRepository + ::core::marker::Send + ::core::marker::Sync>,
    reaction_repository:
        ::std::sync::Arc<dyn EventPostReactionRepository + ::core::marker::Send + ::core::marker::Sync>,
    comment_repository: ::std::sync::Arc<dyn EventPostCommentRepository + ::core::marker::Send + ::core::marker::Sync>,

    user_repository: ::std::sync::Arc<dyn UserRepository + ::core::marker::Send + ::core::marker::Sync>,
    user_exporter: ::std::sync::Arc<dyn UserExporter + ::core::marker::Send + ::core::marker::Sync>,

    media_repository: ::std::sync::Arc<dyn MediaRepository + ::core::marker::Send + ::core::marker::Sync>,

    uuid_generator: ::std::sync::Arc<dyn UuidGenerator + ::core::marker::Send + ::core::marker::Sync>,
    uuid_codec: ::std::sync::Arc<dyn UuidCodec + ::core::marker::Send + ::core::marker::Sync>,
    timestamp_codec: ::std::sync::Arc<dyn TimestampCodec + ::core::marker::Send + ::core::marker::Sync>,

    auth_token_generator: ::std::sync::Arc<dyn AuthTokenGenerator + ::core::marker::Send + ::core::marker::Sync>,
    password_hasher: ::std::sync::Arc<dyn PasswordHasher + ::core::marker::Send + ::core::marker::Sync>,
}

impl ::core::convert::TryFrom<ApplicationContext> for Gateways {
    type Error = ::axiom::result::Error;

    fn try_from(context: ApplicationContext) -> ::core::result::Result<Self, Self::Error> {
        use ::hmac::Mac as _;

        ::console_error_panic_hook::set_once();
        ::tracing_wasm::try_set_as_global_default()?;

        let event_repository: ::std::sync::Arc<dyn EventRepository + ::core::marker::Send + ::core::marker::Sync> =
            ::std::sync::Arc::new(InMemoryEventRepository::builder().build());
        let user_repository: ::std::sync::Arc<dyn UserRepository + ::core::marker::Send + ::core::marker::Sync> =
            ::std::sync::Arc::new(InMemoryUserRepository::builder().build());

        let gateways = Self::builder()
            .event_repository(::std::sync::Arc::clone(&event_repository))
            .event_recommender(::std::sync::Arc::new(
                InMemoryExponentialDecayEventRecommender::builder().limit(10).λ(0.9).build(),
            ))
            .event_exporter(::std::sync::Arc::new(
                GenericEventExporter::builder()
                    .event_repository(::std::sync::Arc::clone(&event_repository))
                    .build(),
            ))
            .event_registration_repository(::std::sync::Arc::new(
                InMemoryEventRegistrationRepository::builder().build(),
            ))
            .post_repository(::std::sync::Arc::new(InMemoryEventPostRepository::builder().build()))
            .reaction_repository(::std::sync::Arc::new(InMemoryEventPostReactionRepository::builder().build()))
            .comment_repository(::std::sync::Arc::new(InMemoryEventPostCommentRepository::builder().build()))
            .user_repository(::std::sync::Arc::clone(&user_repository))
            .user_exporter(::std::sync::Arc::new(
                GenericUserExporter::builder()
                    .user_repository(::std::sync::Arc::clone(&user_repository))
                    .build(),
            ))
            // .media_repository(::std::sync::Arc::new(
            //     MockMediaRepository::builder()
            //         .url("https://i.kym-cdn.com/photos/images/original/003/136/289/782.jpg")
            //         .build(),
            // ))
            .media_repository(::std::sync::Arc::new(
                WasmMediaRepository::builder()
                    .upload_file_callable(context.upload_file_callable)
                    .build(),
            ))
            .uuid_generator(::std::sync::Arc::new(UuidV7Generator::builder().build()))
            .uuid_codec(::std::sync::Arc::new(LowerUrnUuidCodec::builder().build()))
            .timestamp_codec(::std::sync::Arc::new(Rfc2822TimestampCodec::builder().build()))
            .auth_token_generator(::std::sync::Arc::new(
                JsonWebTokenGenerator::builder()
                    .key(::hmac::Hmac::<::sha2::Sha256>::new_from_slice(::core::env!("JWT_SECRET_KEY").as_bytes())?)
                    .build(),
            ))
            .password_hasher(::std::sync::Arc::new(
                Argon2PasswordHasher::builder()
                    .context(::argon2::Argon2::new_with_secret(
                        ::core::env!("ARGON2_SECRET_KEY").as_bytes(),
                        ::argon2::Algorithm::Argon2id,
                        ::argon2::Version::V0x13,
                        ::argon2::Params::default(),
                    )?)
                    .build(),
            ))
            .build();

        // Cron jobs & daemons go here
        ::wasm_bindgen_futures::spawn_local(async {
            use ::futures::StreamExt as _;

            ::gloo::timers::future::IntervalStream::new(4444)
                .for_each(|_| async move {
                    ::tracing::debug!("PING");
                })
                .await;
        });

        gateways.into_ok()
    }
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy)]
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(::tsify::Tsify)]
#[tsify(from_wasm_abi, into_wasm_abi)]
pub enum Profile {
    Dev,
    Prod,
}

pub type Promise<T> = ::core::result::Result<T, ::wasm_bindgen::JsValue>;

trait IntoPromise<T = ()> {
    fn into_promise(self) -> Promise<T>;
}

impl<T> IntoPromise<T> for ::axiom::result::Fallible<T> {
    fn into_promise(self) -> Promise<T> {
        use ::colored::Colorize as _;

        self.inspect_err(|error| ::tracing::debug!("{label} {error}", label = "[EXP-ERR]".red()))
            .map_err(|error| ::wasm_bindgen::JsValue::from_str(&error.to_string()))
    }
}

impl<T, E> IntoPromise<T> for ::axiom::result::Fallible<::core::result::Result<T, ::std::vec::Vec<E>>>
where
    E: ::serde::Serialize,
{
    fn into_promise(self) -> Promise<T> {
        use ::colored::Colorize as _;

        self
            .inspect_err(|error| ::tracing::debug!("{label} {error}", label = "[UNEXP-ERR]".red()))
            .map_err(|error| ::wasm_bindgen::JsValue::from_str(&error.to_string()))
            .and_then(|inner| {
                inner
                    .map_err(|errors| {
                        errors
                            .into_iter()
                            .map(|error| unsafe { <::wasm_bindgen::JsValue as ::gloo::utils::format::JsValueSerdeExt>::from_serde(&error)
                                .inspect(|error| ::tracing::debug!("{label} {error:?}", label = "[EXP-ERR]".red()))
                                .unwrap_unchecked() })  // We kinda know what we're doing
                            .collect::<::std::vec::Vec<_>>()
                    })
                    .map_err(::core::convert::Into::into)
            })
    }
}
