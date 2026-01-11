use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde_json::json;
use std::sync::Arc;

use crate::AppState;

pub async fn get(State(data): State<Arc<AppState>>) {
    
}