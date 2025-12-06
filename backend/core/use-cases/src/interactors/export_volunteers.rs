use ::axiom::prelude::*;

use crate::boundaries::*;
use crate::gateways::*;

#[derive(::bon::Builder)]
pub struct ExportVolunteersInteractor {
    user_exporter: ::std::sync::Arc<dyn UserExporter + ::core::marker::Send + ::core::marker::Sync>,
    user_repository: ::std::sync::Arc<dyn UserRepository + ::core::marker::Send + ::core::marker::Sync>,

    auth_token_generator:
        ::std::sync::Arc<dyn AuthTokenGenerator + ::core::marker::Send + ::core::marker::Sync>,
}

#[async_trait]
impl ExportVolunteersBoundary for ExportVolunteersInteractor {
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
        };

        let bytes = match request.format {
            ExportFormat::Csv =>
                ::std::sync::Arc::clone(&self.user_exporter).export_volunteers_as_csv().await?,
            ExportFormat::Json =>
                ::std::sync::Arc::clone(&self.user_exporter).export_volunteers_as_json().await?,
        };

        let response = OkResponse::builder()
            .bytes(bytes.to_vec())
            .format(request.format)
            .build();
        super::ok!(response)
    }
}

type Request = ExportVolunteersRequest;
type Response = ExportVolunteersResponse;
type OkResponse = ExportVolunteersOkResponse;
type ErrResponse = ExportVolunteersErrResponse;
type ExportFormat = ExportVolunteersExportFormat;
type UserRole = ExportVolunteersUserRole;