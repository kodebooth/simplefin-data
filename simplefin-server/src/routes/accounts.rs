use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use simplefin_data::accountset::{AccountSet, AccountsQuery};

use crate::ServerState;

#[utoipa::path(
    method(get),
    path = "/accounts",
    summary = "Retrieve account and transaction data.",
    responses(
        (status = Status::OK, description = "Successful response", content_type = "application/json", body = AccountSet),
        (status = Status::PAYMENT_REQUIRED, description = "Payment required"),
        (status = Status::FORBIDDEN, description = "Authentication failed. This could be because access has been revoked or if the credentials are incorrect."),
    ),
)]
#[axum::debug_handler]
pub async fn get_accounts(
    State(ServerState(server)): State<ServerState>,
    Query(query): Query<AccountsQuery>,
) -> Result<Json<AccountSet>, StatusCode> {
    server
        .get_accounts(query)
        .await
        .map(Json)
        .map_err(|e| e.into_response().status())
}
