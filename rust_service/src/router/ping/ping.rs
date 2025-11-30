use axum::response::IntoResponse;
use axum::{Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{Level, event};

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub message: String,
}

/// Good to remember:
/// * We need axum extractors `Json(arg): Json<parse_as_this>`
/// * We need our struct (`Message` in this case) to implement Serialize and Debug.
pub async fn ping_handler(Json(message): Json<Message>) -> impl IntoResponse {
    event!(Level::INFO, "got a ping request {:?}", message);

    (StatusCode::OK, Json(json!({"response": "pong"})))
}
