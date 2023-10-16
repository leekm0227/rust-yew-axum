use axum::{
    middleware::from_fn,
    routing::{get, post},
    Router,
};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::{config::app::AppState, core::jwt, handler};

pub fn init_router(app_state: AppState) -> Router {
    let auth_route = Router::new()
        .route("/register", post(handler::auth::register))
        .route("/login", post(handler::auth::login))
        .route("/token", post(handler::auth::token));

    let item_route = Router::new().route("/", get(handler::item::item_list));

    Router::new()
        .route("/", get(|| async {}))
        .route("/ws", get(handler::ws::ws_handle))
        .nest("/auth", auth_route)
        .nest("/items", item_route)
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .layer(from_fn(jwt))
        .with_state(app_state)
}
