use ::axiom::prelude::*;
use ::futures::prelude::*;

use crate::boundaries::*;
use crate::gateways::*;

#[derive(::bon::Builder)]
pub struct ViewEventVolunteersInteractor {
    event_registration_repository: ::std::sync::Arc<dyn EventRegistrationRepository + ::core::marker::Send + ::core::marker::Sync>,
    user_repository: ::std::sync::Arc<dyn UserRepository + ::core::marker::Send + ::core::marker::Sync>,

    uuid_codec: ::std::sync::Arc<dyn UuidCodec + ::core::marker::Send + ::core::marker::Sync>,
    auth_token_generator:
        ::std::sync::Arc<dyn AuthenticationTokenGenerator + ::core::marker::Send + ::core::marker::Sync>,
}

#[async_trait]
impl ViewEventVolunteersBoundary for ViewEventVolunteersInteractor {
    async fn apply(
        self: ::std::sync::Arc<Self>, request: ViewEventVolunteersRequest,
    ) -> ::axiom::result::Fallible<ViewEventVolunteersResponse> {
        match ::std::sync::Arc::clone(&self.auth_token_generator).get_payload(request.token).await? {
            ::core::option::Option::None =>
                return ::axiom::err!(ViewEventVolunteers @ AuthenticationTokenInvalid),
            ::core::option::Option::Some
            (AuthenticationTokenPayload { user_id, user_role: ::domain::UserRole::EventManager, expiry_timestamp }) => {
                if expiry_timestamp < ::axiom::time::Timestamp::now() {
                    return ::axiom::err!(ViewEventVolunteers @ AuthenticationTokenExpired);
                }

                if !::std::sync::Arc::clone(&self.user_repository).contains_id(user_id).await? {
                    return ::axiom::err!(ViewEventVolunteers @ UserNotFound);
                }
            },
            ::core::option::Option::Some(AuthenticationTokenPayload { user_role, .. }) =>
                return ::axiom::err!(ViewEventVolunteers @ UserUnauthorized { user_role: user_role.into() }),
        };

        let event_id = ::std::sync::Arc::clone(&self.uuid_codec).parse(request.event_id).await?;

        let event_registrations = ::std::sync::Arc::clone(&self.event_registration_repository).view_by_event_id(event_id).await?;

        let volunteers = ::futures::stream::iter(event_registrations)
            .map(|event_registration| (event_registration.volunteer_id, *event_registration.statuses.last()))
            .filter_map(|(volunteer_id, event_registration_status)| {
                let user_repository = ::std::sync::Arc::clone(&self.user_repository);

                async move {
                    user_repository.get_by_id(volunteer_id).await.ok()?
                        .filter(|volunteer| ::core::matches!(volunteer.role, ::domain::UserRole::Volunteer))
                        .map(|volunteer| (volunteer, event_registration_status))
                }
            })
            .filter_map(|(volunteer, event_registration_status)| {
                let uuid_codec = ::std::sync::Arc::clone(&self.uuid_codec);

                async move {
                    ViewEventVolunteersVolunteer::build_from(volunteer, event_registration_status)
                        .with_uuid_codec(uuid_codec)
                        .try_build().await
                        .ok()
                }
            })
            .collect::<::std::vec::Vec<_>>().await;

        let response = ViewEventVolunteersOkResponse::builder().volunteers(volunteers).build();
        ::axiom::ok!(ViewEventVolunteers @ response)
    }
}
