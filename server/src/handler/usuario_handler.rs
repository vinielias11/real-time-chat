use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde_json::json;

use crate::AppState;
use crate::model::usuario_model::UsuarioModel;
use crate::schema::usuario_schema::UsuarioSchema;

pub async fn hello_world() -> impl IntoResponse {
    let json_response = json!({
        "status": "ok",
        "message": "Hello, World!"
    });
    
    return Json(json_response);
}

pub async fn create(State(data): State<Arc<AppState>>, Json(body): Json<UsuarioSchema>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = uuid::Uuid::new_v4();
    
    let usuario = sqlx::query_as!(
        UsuarioModel,
        r#"INSERT INTO usuarios (id, nome) VALUES ($1, $2) RETURNING *"#,
        &id,
        &body.nome
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| e.to_string());

    if let Err(err) = usuario {
        if err.to_string().contains("duplicate key value") {
            let error_response = serde_json::json!({
                "status": "erro",
                "message": "Usuário já existe",
            });

            return Err((StatusCode::CONFLICT, Json(error_response)));
        }
        

        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": "erro", "message": format!("{:?}", err)})),
        ));
    }

    let usuario_response = json!({
        "status": "ok",
        "data": json!({
            "usuario": usuario
        })
    });

    Ok(Json(usuario_response))    
}