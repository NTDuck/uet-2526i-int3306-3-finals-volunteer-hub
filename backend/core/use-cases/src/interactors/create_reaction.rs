use ::axiom::prelude::*;

use crate::boundaries::*;
use crate::gateways::*;

#[derive(::bon::Builder)]
pub struct CreateEventPostReactionInteractor {
    post_repository: ::std::sync::Arc<dyn EventPostRepository + ::core::marker::Send + ::core::marker::Sync>,
    reaction_repository: ::std::sync::Arc<dyn EventPostReactionRepository + ::core::marker::Send + ::core::marker::Sync>,
    user_repository: ::std::sync::Arc<dyn UserRepository + ::core::marker::Send + ::core::marker::Sync>,

    uuid_codec: ::std::sync::Arc<dyn UuidCodec + ::core::marker::Send + ::core::marker::Sync>,
    uuid_generator: ::std::sync::Arc<dyn UuidGenerator + ::core::marker::Send + ::core::marker::Sync>,
    auth_token_generator:
        ::std::sync::Arc<dyn AuthenticationTokenGenerator + ::core::marker::Send + ::core::marker::Sync>,
}

#[async_trait]
impl CreateEventPostReactionBoundary for CreateEventPostReactionInteractor {
    async fn apply(
        self: ::std::sync::Arc<Self>, request: CreateEventPostReactionRequest,
    ) -> ::axiom::result::Fallible<CreateEventPostReactionResponse> {
        let actor_id = match ::std::sync::Arc::clone(&self.auth_token_generator).get_payload(request.token).await? {
            ::core::option::Option::None =>
                return ::axiom::err!(CreateEventPostReaction @ AuthenticationTokenInvalid),
            ::core::option::Option::Some
            (AuthenticationTokenPayload { user_id, user_role: ::domain::UserRole::Volunteer | ::domain::UserRole::EventManager, expiry_timestamp }) => {
                if expiry_timestamp < ::axiom::time::Timestamp::now() {
                    return ::axiom::err!(CreateEventPostReaction @ AuthenticationTokenExpired);
                }

                match ::std::sync::Arc::clone(&self.user_repository).get_by_id(user_id).await? {
                    ::core::option::Option::Some(user) => {
                        if ::core::matches!(user.statuses.last(), ::domain::UserStatus::Suspended { .. }) {
                            return ::axiom::err!(CreateEventPostReaction @ UserSuspended);
                        }
                    },
                    ::core::option::Option::None =>
                        return ::axiom::err!(CreateEventPostReaction @ UserNotFound),
                }

                user_id
            },
            ::core::option::Option::Some(AuthenticationTokenPayload { user_role, .. }) =>
                return ::axiom::err!(CreateEventPostReaction @ UserUnauthorized { user_role: user_role.into() }),
        };

        let post_id = ::std::sync::Arc::clone(&self.uuid_codec).parse(request.post_id).await?;

        if !::std::sync::Arc::clone(&self.post_repository).contains_id(post_id).await? {
            return ::axiom::err!(CreateEventPostReaction @ PostNotFound);
        }

        let reaction_id = loop {
            let uuid = ::std::sync::Arc::clone(&self.uuid_generator).generate().await?;

            if !::std::sync::Arc::clone(&self.user_repository).contains_id(uuid).await? {
                break uuid;
            }
        };

        let reaction = ::domain::EventPostReaction::builder()
            .id(reaction_id)
            .post_id(post_id)
            .author_id(actor_id)
            .build();

        ::std::sync::Arc::clone(&self.reaction_repository).save(reaction).await?;

        ::axiom::ok!(CreateEventPostReaction)
    }
}
