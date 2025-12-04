use ::axiom::prelude::*;

use crate::boundaries::*;
use crate::gateways::*;

#[derive(::bon::Builder)]
pub struct RemoveEventPostInteractor {
    event_recommender: ::std::sync::Arc<dyn EventRecommender + ::core::marker::Send + ::core::marker::Sync>,
    post_repository: ::std::sync::Arc<dyn EventPostRepository + ::core::marker::Send + ::core::marker::Sync>,
    user_repository: ::std::sync::Arc<dyn UserRepository + ::core::marker::Send + ::core::marker::Sync>,

    uuid_codec: ::std::sync::Arc<dyn UuidCodec + ::core::marker::Send + ::core::marker::Sync>,
    auth_token_generator:
        ::std::sync::Arc<dyn AuthTokenGenerator + ::core::marker::Send + ::core::marker::Sync>,
}

#[async_trait]
impl RemoveEventPostBoundary for RemoveEventPostInteractor {
    async fn apply(
        self: ::std::sync::Arc<Self>, request: Request,
    ) -> ::axiom::result::Fallible<Response> {
        let actor_id = match ::std::sync::Arc::clone(&self.auth_token_generator)
            .get_payload(request.token)
            .await?
        {
            ::core::option::Option::None => return super::err!(AuthenticationTokenInvalid),
            ::core::option::Option::Some(AuthenticationTokenPayload {
                user_id,
                user_role: ::domain::UserRole::Volunteer | ::domain::UserRole::EventManager,
                expiry_timestamp,
            }) => {
                if expiry_timestamp < ::axiom::time::Timestamp::now() {
                    return super::err!(AuthenticationTokenExpired);
                }

                match ::std::sync::Arc::clone(&self.user_repository).get_by_id(user_id).await? {
                    ::core::option::Option::Some(::domain::User { statuses, .. }) => {
                        if ::core::matches!(statuses[..], [.., ::domain::UserStatus::Suspended { .. }]) {
                            return super::err!(UserSuspended);
                        }
                    },
                    ::core::option::Option::None => return super::err!(UserNotFound),
                }

                user_id
            },
            ::core::option::Option::Some(AuthenticationTokenPayload { user_role, .. }) =>
                return super::err!(UserUnauthorized { user_role: user_role.into(), allowed_user_roles: ::std::vec![UserRole::Volunteer, UserRole::EventManager] }),
        };

        let post_id = ::std::sync::Arc::clone(&self.uuid_codec).parse(request.post_id).await?;

        match ::std::sync::Arc::clone(&self.post_repository).get_by_id(post_id).await? {
            ::core::option::Option::Some(::domain::EventPost { author_id, .. }) =>
                if author_id != actor_id {
                    return super::err!(OwnershipMismatch);
                },
            ::core::option::Option::None => return super::err!(PostNotFound),
        };

        ::std::sync::Arc::clone(&self.post_repository).remove(post_id).await?;

        ::std::sync::Arc::clone(&self.event_recommender)
            .untrack_posted(post_id, actor_id)
            .await?;

        super::ok!(())
    }
}

type Request = RemoveEventPostRequest;
type Response = RemoveEventPostResponse;
type ErrResponse = RemoveEventPostErrResponse;
type UserRole = RemoveEventPostUserRole;
