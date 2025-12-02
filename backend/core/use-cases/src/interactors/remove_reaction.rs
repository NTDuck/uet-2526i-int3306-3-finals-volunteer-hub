use ::axiom::prelude::*;

use crate::boundaries::*;
use crate::gateways::*;

#[derive(::bon::Builder)]
pub struct RemoveEventPostReactionInteractor {
    event_recommender: ::std::sync::Arc<dyn EventRecommender + ::core::marker::Send + ::core::marker::Sync>,
    post_repository: ::std::sync::Arc<dyn EventPostRepository + ::core::marker::Send + ::core::marker::Sync>,
    reaction_repository: ::std::sync::Arc<dyn EventPostReactionRepository + ::core::marker::Send + ::core::marker::Sync>,
    user_repository: ::std::sync::Arc<dyn UserRepository + ::core::marker::Send + ::core::marker::Sync>,

    uuid_codec: ::std::sync::Arc<dyn UuidCodec + ::core::marker::Send + ::core::marker::Sync>,
    auth_token_generator:
        ::std::sync::Arc<dyn AuthenticationTokenGenerator + ::core::marker::Send + ::core::marker::Sync>,
}

#[async_trait]
impl RemoveEventPostReactionBoundary for RemoveEventPostReactionInteractor {
    async fn apply(
        self: ::std::sync::Arc<Self>, request: RemoveEventPostReactionRequest,
    ) -> ::axiom::result::Fallible<RemoveEventPostReactionResponse> {
        let actor_id = match ::std::sync::Arc::clone(&self.auth_token_generator).get_payload(request.token).await? {
            ::core::option::Option::None =>
                return ::axiom::err!(RemoveEventPostReaction @ AuthenticationTokenInvalid),
            ::core::option::Option::Some
            (AuthenticationTokenPayload { user_id, user_role: ::domain::UserRole::Volunteer | ::domain::UserRole::EventManager, expiry_timestamp }) => {
                if expiry_timestamp < ::axiom::time::Timestamp::now() {
                    return ::axiom::err!(RemoveEventPostReaction @ AuthenticationTokenExpired);
                }

                match ::std::sync::Arc::clone(&self.user_repository).get_by_id(user_id).await? {
                    ::core::option::Option::Some(::domain::User { ref statuses, .. }) => {
                        if ::core::matches!(statuses.last(), ::domain::UserStatus::Suspended { .. }) {
                            return ::axiom::err!(RemoveEventPostReaction @ UserSuspended);
                        }
                    },
                    ::core::option::Option::None =>
                        return ::axiom::err!(RemoveEventPostReaction @ UserNotFound),
                }

                user_id
            },
            ::core::option::Option::Some(AuthenticationTokenPayload { user_role, .. }) =>
                return ::axiom::err!(RemoveEventPostReaction @ UserUnauthorized { user_role: user_role.into(), allowed_user_roles: ::std::vec![RemoveEventPostReactionUserRole::Volunteer, RemoveEventPostReactionUserRole::EventManager] }),
        };

        let reaction_or_post_id = ::std::sync::Arc::clone(&self.uuid_codec).parse(request.reaction_or_post_id).await?;

        let ::core::option::Option::Some(reaction) = ::std::sync::Arc::clone(&self.reaction_repository).get_by_id(reaction_or_post_id).await?
            .try_or_else_async(|| async { ::std::sync::Arc::clone(&self.reaction_repository).get_by_post_and_user_id(reaction_or_post_id, actor_id).await }).await?
        else {
            return ::axiom::err!(RemoveEventPostReaction @ ReactionNotFound);
        };

        if reaction.author_id != actor_id {
            return ::axiom::err!(RemoveEventPostReaction @ OwnershipMismatch);
        }

        ::std::sync::Arc::clone(&self.reaction_repository).remove(reaction.id).await?;

        let ::domain::EventPost { event_id, .. } = unsafe { ::std::sync::Arc::clone(&self.post_repository).get_by_id(reaction.post_id).await?.unwrap_unchecked() };

        ::std::sync::Arc::clone(&self.event_recommender).untrack_reacted(event_id, actor_id).await?;

        ::axiom::ok!(RemoveEventPostReaction)
    }
}
