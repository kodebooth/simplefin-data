use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use simplefin_data::token::Token;

use crate::ServerState;

#[utoipa::path(
    method(post),
    path = "/claim/{token}",
    params(
        ("token", description = "A one-time use code embedded within the SimpleFIN Token."),
    ),
    summary = "An application receives a SimpleFIN Token from a user. SimpleFIN Tokens are Base64-encoded URLs. A decoded SimpleFIN Token will point to this resource.",
    responses(
        (status = Status::OK, description = "Successful response", content_type = "application/text", body = String),
        (status = Status::FORBIDDEN, description = "The claim token either does not exist or has already been used claimed by someone else. Receiving this could mean that the user’s transaction information has been compromised."),
    ),
)]
#[axum::debug_handler]
pub async fn post_claim(
    State(ServerState(server)): State<ServerState>,
    Path(token): Path<Token>,
) -> Result<String, StatusCode> {
    server
        .claim_token(token)
        .await
        .map(|url| url.to_string())
        .map_err(|e| e.into_response().status())
}
