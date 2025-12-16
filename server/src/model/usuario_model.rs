use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct UsuarioModel {
    pub id: Uuid,
    pub nome: String,
}
