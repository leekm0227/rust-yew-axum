use axum::{extract::State, Json};
use serde_json::{json, Value};

use crate::config::app::AppState;

pub async fn item_list(State(app): State<AppState>) -> Json<Value> {
    let item_list = app.db.item_list().await;
    Json(json!({ "success": true, "items":item_list}))
}
