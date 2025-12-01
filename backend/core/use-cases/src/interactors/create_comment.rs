use ::axiom::prelude::*;

use crate::boundaries::*;
use crate::gateways::*;

#[derive(::bon::Builder)]
pub struct CreateEventPostCommentPostCommentInteractor {
    event_repository: ::std::sync::Arc<dyn EventRepository + ::core::marker::Send + ::core::marker::Sync>,
    user_repository: ::std::sync::Arc<dyn UserRepository + ::core::marker::Send + ::core::marker::Sync>,

    uuid_generator: ::std::sync::Arc<dyn UuidGenerator + ::core::marker::Send + ::core::marker::Sync>,
    auth_token_generator:
        ::std::sync::Arc<dyn AuthenticationTokenGenerator + ::core::marker::Send + ::core::marker::Sync>,
}

impl CreateEventPostCommentPostCommentBoundary for CreateEventPostCommentPostCommentInteractor {
    async fn apply(
        self: ::std::sync::Arc<Self>, request: CreateEventPostCommentPostCommentRequest,
    ) -> ::axiom::result::Fallible<CreateEventPostCommentPostCommentResponse> {
        let actor_id = match ::std::sync::Arc::clone(&self.auth_token_generator).get_payload(request.token).await? {
            ::core::option::Option::None =>
                return ::axiom::err!(CreateEventPostComment @ AuthenticationTokenInvalid),
            ::core::option::Option::Some
            (AuthenticationTokenPayload { user_id, user_role: ::domain::UserRole::EventManager, expiry_timestamp }) => {
                if expiry_timestamp < ::axiom::time::Timestamp::now() {
                    return ::axiom::err!(CreateEventPostComment @ AuthenticationTokenExpired);
                }

                match ::std::sync::Arc::clone(&self.user_repository).get_by_id(user_id).await? {
                    ::core::option::Option::Some(user) => {
                        if ::core::matches!(user.statuses.last(), ::domain::UserStatus::Suspended { .. }) {
                            return ::axiom::err!(CreateEventPostComment @ UserSuspended);
                        }
                    },
                    ::core::option::Option::None =>
                        return ::axiom::err!(CreateEventPostComment @ UserNotFound),
                }

                user_id
            },
            ::core::option::Option::Some(AuthenticationTokenPayload { user_role, .. }) =>
                return ::axiom::err!(CreateEventPostComment @ UserUnauthorized { user_role: user_role.into() }),
        };
    }
}
