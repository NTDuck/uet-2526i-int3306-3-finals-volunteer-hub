use ::axiom::prelude::*;

use crate::boundaries::*;
use crate::gateways::*;

#[derive(::bon::Builder)]
pub struct ExportVolunteersInteractor {
    user_exporter: ::std::sync::Arc<dyn UserExporter + ::core::marker::Send + ::core::marker::Sync>,
    user_repository: ::std::sync::Arc<dyn UserRepository + ::core::marker::Send + ::core::marker::Sync>,

    auth_token_generator: ::std::sync::Arc<dyn AuthenticationTokenGenerator + ::core::marker::Send + ::core::marker::Sync>,
}

#[async_trait]
impl ExportVolunteersBoundary for ExportVolunteersInteractor {
    async fn apply(
        self: ::std::sync::Arc<Self>, request: ExportVolunteersRequest,
    ) -> ::axiom::result::Fallible<ExportVolunteersResponse> {
        match ::std::sync::Arc::clone(&self.auth_token_generator).get_payload(request.token).await? {
            ::core::option::Option::None =>
                return ::axiom::err!(ExportVolunteers @ AuthenticationTokenInvalid),
            ::core::option::Option::Some
            (AuthenticationTokenPayload { user_id, user_role: ::domain::UserRole::Administrator, expiry_timestamp }) => {
                if expiry_timestamp < ::axiom::time::Timestamp::now() {
                    return ::axiom::err!(ExportVolunteers @ AuthenticationTokenExpired);
                }

                if !::std::sync::Arc::clone(&self.user_repository).contains_id(user_id).await? {
                    return ::axiom::err!(ExportVolunteers @ UserNotFound);
                }
            },
            ::core::option::Option::Some(AuthenticationTokenPayload { user_role, .. }) =>
                return ::axiom::err!(ExportVolunteers @ UserUnauthorized { user_role: user_role.into() }),
        };

        let bytes = match request.format {
            ExportVolunteersExportFormat::Csv => ::std::sync::Arc::clone(&self.user_exporter).export_volunteers_as_csv().await?,
            ExportVolunteersExportFormat::Json => ::std::sync::Arc::clone(&self.user_exporter).export_volunteers_as_json().await?,
        };

        let response = ExportVolunteersOkResponse::builder().bytes(bytes).format(request.format).build();
        ::axiom::ok!(ExportVolunteers @ response)
    }
}
