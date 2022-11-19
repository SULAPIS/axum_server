use axum::routing::{get, post};

use crate::handler::category::{login_user, register_user};

// use crate::handler;

pub fn init() -> axum::Router {
    axum::Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/user/login", post(login_user))
        .route("/user/register", post(register_user))
}
