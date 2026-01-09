-- Add up migration script here
CREATE TABLE IF NOT EXISTS mensagens (
    id UUID PRIMARY KEY NOT NULL,
    conteudo TEXT NOT NULL,
    id_usuario_from UUID NOT NULL,
    nome_usuario_from TEXT NOT NULL,
    id_usuario_to UUID,
    cor_criacao TEXT NOT NULL,
    data_criacao TIMESTAMPTZ NULL DEFAULT NOW()
)