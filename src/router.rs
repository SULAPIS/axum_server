use axum::routing::{get, post};

use crate::handler::category::*;

// use crate::handler;

pub fn init() -> axum::Router {
    axum::Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/user/login", post(login_user))
        .route("/user/register", post(register_user))
        .route("/order/add", post(add_order))
        .route("/order/get", get(get_order))
        .route("/order/edit", post(change_state))
}
