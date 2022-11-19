use axum::routing::{get, post};

use crate::handler::category::login_user;

// use crate::handler;

pub fn init() -> axum::Router {
    axum::Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/user/login", post(login_user))
}
