use axum::{extract::State, response::Redirect};

use crate::ServerState;

#[utoipa::path(
    method(get),
    path = "/create",
    summary = "An application directs a user to this URL to initiate a bank-app connection",
    responses(
        (status = Status::SEE_OTHER, description = "Successful response"),
    ),
)]
#[axum::debug_handler]
pub async fn get_create(State(ServerState(server)): State<ServerState>) -> Redirect {
    Redirect::to(&server.create_redirect().await.to_string())
}
