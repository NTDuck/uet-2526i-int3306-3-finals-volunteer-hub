use ::axiom::prelude::*;
use ::futures::prelude::*;

use crate::boundaries::*;
use crate::gateways::*;

#[derive(::bon::Builder)]
pub struct ViewUsersInteractor {
    user_repository: ::std::sync::Arc<dyn UserRepository + ::core::marker::Send + ::core::marker::Sync>,

    uuid_codec: ::std::sync::Arc<dyn UuidCodec + ::core::marker::Send + ::core::marker::Sync>,
    auth_token_generator:
        ::std::sync::Arc<dyn AuthTokenGenerator + ::core::marker::Send + ::core::marker::Sync>,
}

#[async_trait]
impl ViewUsersBoundary for ViewUsersInteractor {
    async fn apply(
        self: ::std::sync::Arc<Self>, request: Request,
    ) -> ::axiom::result::Fallible<Response> {
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
                return super::err!(UserUnauthorized { user_role: user_role.into(), allowed_user_roles: ::std::vec![UserRole::Administrator] }),
        }

        let users = match request.filter {
            ::core::option::Option::None => ::std::sync::Arc::clone(&self.user_repository).view().await?,
            ::core::option::Option::Some(filter) =>
                ::std::sync::Arc::clone(&self.user_repository).search(filter.into()).await?,
        };

        let users = users
            .into_stream()
            .then(|user| {
                let uuid_codec = ::std::sync::Arc::clone(&self.uuid_codec);

                async move { User::build_from(user).with_uuid_codec(uuid_codec).try_build().await }
            })
            .try_collect::<::std::vec::Vec<_>>()
            .await?;

        let response = OkResponse::builder().users(users).build();
        super::ok!(response)
    }
}

type Request = ViewUsersRequest;
type Response = ViewUsersResponse;
type OkResponse = ViewUsersOkResponse;
type ErrResponse = ViewUsersErrResponse;
type UserRole = ViewUsersUserRole;
type User = ViewUsersUser;
