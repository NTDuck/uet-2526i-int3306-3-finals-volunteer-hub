use ::axiom::prelude::*;

use crate::boundaries::*;
use crate::gateways::*;

#[derive(::bon::Builder)]
pub struct RemoveEventPostInteractor {
    post_repository: ::std::sync::Arc<dyn EventPostRepository + ::core::marker::Send + ::core::marker::Sync>,
    user_repository: ::std::sync::Arc<dyn UserRepository + ::core::marker::Send + ::core::marker::Sync>,

    uuid_codec: ::std::sync::Arc<dyn UuidCodec + ::core::marker::Send + ::core::marker::Sync>,
    uuid_generator: ::std::sync::Arc<dyn UuidGenerator + ::core::marker::Send + ::core::marker::Sync>,
    auth_token_generator:
        ::std::sync::Arc<dyn AuthenticationTokenGenerator + ::core::marker::Send + ::core::marker::Sync>,
}

#[async_trait]
impl RemoveEventPostBoundary for RemoveEventPostInteractor {
    async fn apply(
        self: ::std::sync::Arc<Self>, request: RemoveEventPostRequest,
    ) -> ::axiom::result::Fallible<RemoveEventPostResponse> {
        let actor_id = match ::std::sync::Arc::clone(&self.auth_token_generator).get_payload(request.token).await? {
            ::core::option::Option::None =>
                return ::axiom::err!(RemoveEventPost @ AuthenticationTokenInvalid),
            ::core::option::Option::Some
            (AuthenticationTokenPayload { user_id, user_role: ::domain::UserRole::Volunteer | ::domain::UserRole::EventManager, expiry_timestamp }) => {
                if expiry_timestamp < ::axiom::time::Timestamp::now() {
                    return ::axiom::err!(RemoveEventPost @ AuthenticationTokenExpired);
                }

                match ::std::sync::Arc::clone(&self.user_repository).get_by_id(user_id).await? {
                    ::core::option::Option::Some(::domain::User { ref statuses, .. }) => {
                        if ::core::matches!(statuses.last(), ::domain::UserStatus::Suspended { .. }) {
                            return ::axiom::err!(RemoveEventPost @ UserSuspended);
                        }
                    },
                    ::core::option::Option::None =>
                        return ::axiom::err!(RemoveEventPost @ UserNotFound),
                }

                user_id
            },
            ::core::option::Option::Some(AuthenticationTokenPayload { user_role, .. }) =>
                return ::axiom::err!(RemoveEventPost @ UserUnauthorized { user_role: user_role.into() }),
        };

        let post_id = ::std::sync::Arc::clone(&self.uuid_codec).parse(request.post_id).await?;

        match ::std::sync::Arc::clone(&self.post_repository).get_by_id(post_id).await? {
            ::core::option::Option::Some(::domain::EventPost { user_id, .. }) => {
                if user_id != actor_id {
                    return ::axiom::err!(RemoveEventPost @ OwnershipMismatch);
                }
            },
            ::core::option::Option::None => return ::axiom::err!(RemoveEventPost @ PostNotFound),
        };

        ::std::sync::Arc::clone(&self.post_repository).remove(post_id).await?;

        ::axiom::ok!(RemoveEventPost)
    }
}
