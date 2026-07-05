use axum::{Json, extract::State, http::StatusCode};
use simplefin_data::version::Versions;

use crate::ServerState;

#[utoipa::path(
    method(get),
    path = "/info",
    summary = "Used by Applications to find out what versions of the SimpleFIN Protocol the server supports",
    responses(
        (status = Status::OK, description = "Successful response", content_type = "application/json", body = Versions),
    ),
)]
#[axum::debug_handler]
pub async fn get_info(
    State(ServerState(server)): State<ServerState>,
) -> Result<Json<Versions>, StatusCode> {
    Ok(Json(Versions {
        versions: server.versions().await,
    }))
}
