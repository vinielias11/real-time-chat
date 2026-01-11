use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct MensagemSchema {
    pub conteudo: String,
    pub id_usuario_from: Uuid,
    pub nome_usuario_from: String,
    pub id_usuario_to: Option<Uuid>,
    pub cor_criacao: String
}
