use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct MensagemModel {
    pub id: Uuid,
    pub conteudo: String,
    pub id_usuario_from: Uuid,
    pub nome_usuario_from: String,
    pub id_usuario_to: Option<Uuid>,
    pub cor_criacao: String,
    pub data_criacao: DateTime<Utc>
}