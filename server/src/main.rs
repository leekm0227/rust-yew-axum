use dotenv::dotenv;

mod config;
mod core;
mod handler;
mod model;
mod repository;
mod route;
mod schema;
mod util;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let app = route::init_router(config::app::AppState::new());
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
