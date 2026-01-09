use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MensagemSchema {
    pub conteudo: String
}
