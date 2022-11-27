use axum::routing::{get, post};

use crate::handler::category::*;

// use crate::handler;

pub fn init() -> axum::Router {
    axum::Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/user/login", post(login_user))
        .route("/user/register", post(register_user))
        .route("/orders/add", post(add_order))
        .route("/orders/get", get(get_order))
        .route("/orders/edit", post(change_state))
        .route("/order/add", post(save_order_details))
        .route("/order/get", get(get_order_details))
        .route("/order/edit", post(change_state_details))
        .route("/supply/my", get(get_supply_order))
        .route("/supply/none", get(get_supply_order_null))
        .route("/supply/receive", post(receive_order))
        .route("/order/cancel", post(cancel_order))
}
