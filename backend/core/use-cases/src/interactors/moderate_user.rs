use ::axiom::prelude::*;

use crate::boundaries::*;
use crate::gateways::*;

#[derive(::bon::Builder)]
pub struct ModerateUserInteractor {
    user_repository: ::std::sync::Arc<dyn UserRepository + ::core::marker::Send + ::core::marker::Sync>,

    uuid_codec: ::std::sync::Arc<dyn UuidCodec + ::core::marker::Send + ::core::marker::Sync>,
    auth_token_generator: ::std::sync::Arc<dyn AuthenticationTokenGenerator + ::core::marker::Send + ::core::marker::Sync>,
}

#[async_trait]
impl ModerateUserBoundary for ModerateUserInteractor {
    async fn apply(
        self: ::std::sync::Arc<Self>, request: ModerateUserRequest,
    ) -> ::axiom::result::Fallible<ModerateUserResponse> {
        let actor_id = match ::std::sync::Arc::clone(&self.auth_token_generator).get_payload(request.token).await? {
            ::core::option::Option::None =>
                return ::axiom::err!(ModerateUser @ AuthenticationTokenInvalid),
            ::core::option::Option::Some
            (AuthenticationTokenPayload { user_id, user_role: ::domain::UserRole::Administrator, expiry_timestamp }) => {
                if expiry_timestamp < ::axiom::time::Timestamp::now() {
                    return ::axiom::err!(ModerateUser @ AuthenticationTokenExpired);
                }

                if !::std::sync::Arc::clone(&self.user_repository).contains_id(user_id).await? {
                    return ::axiom::err!(ModerateUser @ UserNotFound);
                }

                user_id
            },
            ::core::option::Option::Some(AuthenticationTokenPayload { user_role, .. }) =>
                return ::axiom::err!(ModerateUser @ UserUnauthorized { user_role: user_role.into(), allowed_user_roles: ::std::vec![ModerateUserUserRole::Administrator] }),
        };

        let user_id = ::std::sync::Arc::clone(&self.uuid_codec).parse(request.user_id).await?;

        let ::core::option::Option::Some(mut user) = ::std::sync::Arc::clone(&self.user_repository).get_by_id(user_id).await? else {
            return ::axiom::err!(ModerateUser @ UserNotFound);
        };

        if !::core::matches!(user.role, ::domain::UserRole::Volunteer | ::domain::UserRole::EventManager) {
            return ::axiom::err!(ModerateUser @ UserRoleNotEligible {
                user_role: user.role.into(),
                allowed_user_roles: ::std::vec![
                    ModerateUserUserRole::Volunteer,
                    ModerateUserUserRole::EventManager,
                ],
            });
        }

        let user_status = *user.statuses.last();

        match request.user_status {
            ModerateUserNewUserStatus::Suspended => {
                if !::core::matches!(user_status, ::domain::UserStatus::Created | ::domain::UserStatus::Suspended { .. }) {
                    return ::axiom::err!(ModerateUser @ UserStatusNotEligible {
                        user_status: user_status.into(),
                        allowed_user_statuses: ::std::vec![
                            ModerateUserUserStatus::Created,
                            ModerateUserUserStatus::Suspended,
                        ],
                    });
                }

                user.statuses.push(::domain::UserStatus::Suspended { suspended_by_administrator_id: actor_id, suspended_at: ::axiom::time::Timestamp::now() });
            },

            ModerateUserNewUserStatus::Unsuspended => {
                if !::core::matches!(user_status, ::domain::UserStatus::Suspended { .. }) {
                    return ::axiom::err!(ModerateUser @ UserStatusNotEligible {
                        user_status: user_status.into(),
                        allowed_user_statuses: ::std::vec![ModerateUserUserStatus::Suspended],
                    });
                }

                user.statuses.push(::domain::UserStatus::Unsuspended { unsuspended_by_administrator_id: actor_id, unsuspended_at: ::axiom::time::Timestamp::now() });
            },
        }

        ::std::sync::Arc::clone(&self.user_repository).save(user).await?;

        ::axiom::ok!(ModerateUser)
    }
}
