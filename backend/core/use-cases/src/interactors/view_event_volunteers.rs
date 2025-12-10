use ::axiom::prelude::*;
use ::futures::prelude::*;

use crate::boundaries::*;
use crate::gateways::*;

#[derive(::bon::Builder)]
pub struct ViewEventVolunteersInteractor {
    event_registration_repository:
        ::std::sync::Arc<dyn EventRegistrationRepository + ::core::marker::Send + ::core::marker::Sync>,
    user_repository: ::std::sync::Arc<dyn UserRepository + ::core::marker::Send + ::core::marker::Sync>,

    uuid_codec: ::std::sync::Arc<dyn UuidCodec + ::core::marker::Send + ::core::marker::Sync>,
    auth_token_generator: ::std::sync::Arc<dyn AuthTokenGenerator + ::core::marker::Send + ::core::marker::Sync>,
}

#[async_trait]
impl ViewEventVolunteersBoundary for ViewEventVolunteersInteractor {
    async fn apply(self: ::std::sync::Arc<Self>, request: Request) -> ::axiom::result::Fallible<Response> {
        match ::std::sync::Arc::clone(&self.auth_token_generator)
            .get_payload(request.token)
            .await?
        {
            ::core::option::Option::None => return super::err!(AuthenticationTokenInvalid),
            ::core::option::Option::Some(AuthTokenPayload {
                user_id,
                user_role: ::domain::UserRole::EventManager,
                expiry_timestamp,
            }) => {
                if expiry_timestamp < ::axiom::time::Timestamp::now() {
                    return super::err!(AuthenticationTokenExpired);
                }

                if !::std::sync::Arc::clone(&self.user_repository).contains_id(user_id).await? {
                    return super::err!(UserNotFound);
                }
            },
            ::core::option::Option::Some(AuthTokenPayload { user_role, .. }) =>
                return super::err!(UserUnauthorized {
                    user_role: user_role.into(),
                    allowed_user_roles: ::std::vec![UserRole::EventManager]
                }),
        };

        let event_id = ::std::sync::Arc::clone(&self.uuid_codec).parse(request.event_id).await?;

        let event_registrations = ::std::sync::Arc::clone(&self.event_registration_repository)
            .view_by_event_id(event_id)
            .await?;

        let volunteers = event_registrations
            .into_stream()
            .then(|event_registration| {
                let user_repository = ::std::sync::Arc::clone(&self.user_repository);

                async move {
                    user_repository
                        .get_by_id(event_registration.volunteer_id)
                        .await?
                        .filter(|volunteer| ::core::matches!(volunteer.role, ::domain::UserRole::Volunteer))
                        .map(|volunteer| (volunteer, event_registration))
                        .into_ok()
                }
            })
            .filter_map(|transposable| async move { transposable.transpose() })
            .and_then(|(volunteer, event_registration)| {
                let uuid_codec = ::std::sync::Arc::clone(&self.uuid_codec);

                async move {
                    Volunteer::build_from(volunteer, event_registration)
                        .with_uuid_codec(uuid_codec)
                        .try_build()
                        .await
                }
            })
            .try_collect::<::std::vec::Vec<_>>()
            .await?;

        let response = OkResponse::builder().volunteers(volunteers).build();
        super::ok!(response)
    }
}

type Request = ViewEventVolunteersRequest;
type Response = ViewEventVolunteersResponse;
type OkResponse = ViewEventVolunteersOkResponse;
type ErrResponse = ViewEventVolunteersErrResponse;
type UserRole = ViewEventVolunteersUserRole;
type Volunteer = ViewEventVolunteersVolunteer;
