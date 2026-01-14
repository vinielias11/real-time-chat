use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde_json::json;
use std::sync::Arc;

use crate::{AppState, model::mensagem_model::MensagemModel};

pub async fn get(
    State(data): State<Arc<AppState>>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let mensagens: Vec<MensagemModel> = sqlx::query_as::<_, MensagemModel>("SELECT * FROM mensagens ORDER BY data_criacao")
    .fetch_all(&data.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )
    })?;

    let mensagens_response = json!({
        "status": "ok",
        "data": json!({
            "mensagens": mensagens
        })
    });

    Ok(Json(mensagens_response))
}