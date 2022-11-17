mod config;
mod entity;
mod form;
mod handler;
mod jsonwebtoken;
mod router;
mod state;
use axum::Extension;

use dotenv::dotenv;
use sea_orm::Database;
use std::sync::Arc;
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenv().ok();

    let cfg = config::Config::from_env().unwrap();

    let conn = Database::connect(&cfg.db.dsn).await.unwrap();

    tracing::info!("Web server {}", &cfg.web.addr);

    let app = router::init().layer(Extension(Arc::new(state::AppState { conn })));
    axum::Server::bind(&cfg.web.addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
