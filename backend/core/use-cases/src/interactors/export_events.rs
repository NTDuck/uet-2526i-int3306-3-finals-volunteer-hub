use ::axiom::prelude::*;

use crate::boundaries::*;
use crate::gateways::*;

#[derive(::bon::Builder)]
pub struct ExportEventsInteractor {
    event_exporter: ::std::sync::Arc<dyn EventExporter + ::core::marker::Send + ::core::marker::Sync>,
    user_repository: ::std::sync::Arc<dyn UserRepository + ::core::marker::Send + ::core::marker::Sync>,

    auth_token_generator: ::std::sync::Arc<dyn AuthenticationTokenGenerator + ::core::marker::Send + ::core::marker::Sync>,
}

#[async_trait]
impl ExportEventsBoundary for ExportEventsInteractor {
    async fn apply(
        self: ::std::sync::Arc<Self>, request: ExportEventsRequest,
    ) -> ::axiom::result::Fallible<ExportEventsResponse> {
        match ::std::sync::Arc::clone(&self.auth_token_generator).get_payload(request.token).await? {
            ::core::option::Option::None =>
                return ::axiom::err!(ExportEvents @ AuthenticationTokenInvalid),
            ::core::option::Option::Some
            (AuthenticationTokenPayload { user_id, user_role: ::domain::UserRole::Administrator, expiry_timestamp }) => {
                if expiry_timestamp < ::axiom::time::Timestamp::now() {
                    return ::axiom::err!(ExportEvents @ AuthenticationTokenExpired);
                }

                if !::std::sync::Arc::clone(&self.user_repository).contains_id(user_id).await? {
                    return ::axiom::err!(ExportEvents @ UserNotFound);
                }
            },
            ::core::option::Option::Some(AuthenticationTokenPayload { user_role, .. }) =>
                return ::axiom::err!(ExportEvents @ UserUnauthorized { user_role: user_role.into(), allowed_user_roles: ::std::vec![ExportEventsUserRole::Administrator] }),
        };

        let bytes = match request.format {
            ExportEventsExportFormat::Csv => ::std::sync::Arc::clone(&self.event_exporter).export_as_csv().await?,
            ExportEventsExportFormat::Json => ::std::sync::Arc::clone(&self.event_exporter).export_as_json().await?,
        };

        let response = ExportEventsOkResponse::builder().bytes(bytes).format(request.format).build();
        ::axiom::ok!(ExportEvents @ response)
    }
}
