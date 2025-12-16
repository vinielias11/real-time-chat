use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde_json::json;
use std::sync::Arc;

use crate::AppState;
use crate::model::usuario_model::UsuarioModel;
use crate::schema::usuario_schema::UsuarioSchema;

pub async fn create(
    State(data): State<Arc<AppState>>,
    Json(body): Json<UsuarioSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let usuario_existente = sqlx::query_as!(
        UsuarioModel,
        "SELECT * FROM usuarios WHERE nome = $1",
        body.nome
    )
    .fetch_optional(&data.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )
    })?;

    if let Some(usuario) = usuario_existente {
        let response = json!({
            "status": "ok",
            "data": {
                "usuario": usuario
            }
        });

        return Ok(Json(response));
    }

    let id = uuid::Uuid::new_v4();

    let usuario = sqlx::query_as!(
        UsuarioModel,
        r#"INSERT INTO usuarios (id, nome) VALUES ($1, $2) RETURNING *"#,
        &id,
        &body.nome
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )
    })?;

    let usuario_response = json!({
        "status": "ok",
        "data": json!({
            "usuario": usuario
        })
    });

    Ok(Json(usuario_response))
}
