use ::axiom::prelude::*;

use crate::boundaries::*;
use crate::gateways::*;

#[derive(::bon::Builder)]
pub struct ViewUserInteractor {
    user_repository: ::std::sync::Arc<dyn UserRepository + ::core::marker::Send + ::core::marker::Sync>,

    uuid_codec: ::std::sync::Arc<dyn UuidCodec + ::core::marker::Send + ::core::marker::Sync>,
    timestamp_codec: ::std::sync::Arc<dyn TimestampCodec + ::core::marker::Send + ::core::marker::Sync>,
    auth_token_generator: ::std::sync::Arc<dyn AuthTokenGenerator + ::core::marker::Send + ::core::marker::Sync>,
}

#[async_trait]
impl ViewUserBoundary for ViewUserInteractor {
    async fn apply(self: ::std::sync::Arc<Self>, request: Request) -> ::axiom::result::Fallible<Response> {
        match ::std::sync::Arc::clone(&self.auth_token_generator)
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
            },
            ::core::option::Option::Some(AuthTokenPayload { user_role, .. }) =>
                return super::err!(UserUnauthorized {
                    user_role: user_role.into(),
                    allowed_user_roles: ::std::vec![UserRole::Administrator]
                }),
        }

        let user_id = ::std::sync::Arc::clone(&self.uuid_codec).parse(request.user_id).await?;
        let ::core::option::Option::Some(user) =
            ::std::sync::Arc::clone(&self.user_repository).get_by_id(user_id).await?
        else {
            return super::err!(UserNotFound);
        };

        let user = User::build_from(user)
            .with_uuid_codec(::std::sync::Arc::clone(&self.uuid_codec))
            .with_timestamp_codec(::std::sync::Arc::clone(&self.timestamp_codec))
            .try_build()
            .await?;

        let response = OkResponse::builder().user(user).build();
        super::ok!(response)
    }
}

type Request = ViewUserRequest;
type Response = ViewUserResponse;
type OkResponse = ViewUserOkResponse;
type ErrResponse = ViewUserErrResponse;
type UserRole = ViewUserUserRole;
type User = ViewUserUser;
