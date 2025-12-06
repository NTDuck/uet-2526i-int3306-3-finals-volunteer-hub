use ::axiom::prelude::*;

use crate::boundaries::*;
use crate::gateways::*;

#[derive(::bon::Builder)]
pub struct ModerateUserInteractor {
    user_repository: ::std::sync::Arc<dyn UserRepository + ::core::marker::Send + ::core::marker::Sync>,

    uuid_codec: ::std::sync::Arc<dyn UuidCodec + ::core::marker::Send + ::core::marker::Sync>,
    auth_token_generator: ::std::sync::Arc<dyn AuthTokenGenerator + ::core::marker::Send + ::core::marker::Sync>,
}

#[async_trait]
impl ModerateUserBoundary for ModerateUserInteractor {
    async fn apply(self: ::std::sync::Arc<Self>, request: Request) -> ::axiom::result::Fallible<Response> {
        let actor_id = match ::std::sync::Arc::clone(&self.auth_token_generator)
            .get_payload(request.token)
            .await?
        {
            ::core::option::Option::None => return super::err!(AuthenticationTokenInvalid),
            ::core::option::Option::Some(AuthTokenPayload {
                user_id,
                user_role: ::domain::UserRole::Administrator,
                expiry_timestamp,
            }) => {
                if expiry_timestamp < ::axiom::time::Timestamp::now() {
                    return super::err!(AuthenticationTokenExpired);
                }

                if !::std::sync::Arc::clone(&self.user_repository).contains_id(user_id).await? {
                    return super::err!(UserNotFound);
                }

                user_id
            },
            ::core::option::Option::Some(AuthTokenPayload { user_role, .. }) =>
                return super::err!(UserUnauthorized {
                    user_role: user_role.into(),
                    allowed_user_roles: ::std::vec![UserRole::Administrator]
                }),
        };

        let user_id = ::std::sync::Arc::clone(&self.uuid_codec).parse(request.user_id).await?;

        let ::core::option::Option::Some(mut user) =
            ::std::sync::Arc::clone(&self.user_repository).get_by_id(user_id).await?
        else {
            return super::err!(UserNotFound);
        };

        if !::core::matches!(user.role, ::domain::UserRole::Volunteer | ::domain::UserRole::EventManager) {
            return super::err!(UserRoleNotEligible {
                user_role: user.role.into(),
                allowed_user_roles: ::std::vec![UserRole::Volunteer, UserRole::EventManager,],
            });
        }

        let user_status = *user.statuses.last();

        match request.user_status {
            NewUserStatus::Suspended => {
                if !::core::matches!(
                    user_status,
                    ::domain::UserStatus::Created { .. }
                        | ::domain::UserStatus::Updated { .. }
                        | ::domain::UserStatus::Suspended { .. }
                ) {
                    return super::err!(UserStatusNotEligible {
                        user_status: user_status.into(),
                        allowed_user_statuses: ::std::vec![
                            UserStatus::Created,
                            UserStatus::Updated,
                            UserStatus::Suspended,
                        ],
                    });
                }

                user.statuses.push(::domain::UserStatus::Suspended {
                    suspended_by_administrator_id: actor_id,
                    suspended_at: ::axiom::time::Timestamp::now(),
                });
            },

            NewUserStatus::Unsuspended => {
                if !::core::matches!(user_status, ::domain::UserStatus::Suspended { .. }) {
                    return super::err!(UserStatusNotEligible {
                        user_status: user_status.into(),
                        allowed_user_statuses: ::std::vec![UserStatus::Suspended],
                    });
                }

                user.statuses.push(::domain::UserStatus::Unsuspended {
                    unsuspended_by_administrator_id: actor_id,
                    unsuspended_at: ::axiom::time::Timestamp::now(),
                });
            },
        }

        ::std::sync::Arc::clone(&self.user_repository).save(user).await?;

        super::ok!(())
    }
}

type Request = ModerateUserRequest;
type Response = ModerateUserResponse;
type ErrResponse = ModerateUserErrResponse;
type UserRole = ModerateUserUserRole;
type UserStatus = ModerateUserUserStatus;
type NewUserStatus = ModerateUserNewUserStatus;
