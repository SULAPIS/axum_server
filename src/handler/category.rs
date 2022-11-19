use super::get_conn;
use crate::{
    entity::{category, user_info},
    form::form_entity::*,
    jsonwebtoken,
    state::AppState,
};

use axum::{
    body::Body,
    http::{header::HeaderName, HeaderMap, HeaderValue},
    response::Json,
    routing::get,
    Extension, Form, Router,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter, QueryOrder, Set};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReturnJSON {
    pub code: u32,
    pub data: Value,
    pub error: String,
}

/// POST
pub async fn login_user(
    Extension(state): Extension<Arc<AppState>>,
    Json(frm): Json<UserNumPwd>,
) -> (HeaderMap, Json<ReturnJSON>) {
    let mut code = 1;
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("content-type"),
        HeaderValue::from_static("text/plain;charset=utf-8"),
    );
    let conn = get_conn(&state);
    let user_info_res: Result<Option<user_info::Model>, DbErr> = user_info::Entity::find()
        .filter(user_info::Column::Number.eq(frm.number))
        .one(conn)
        .await;
    let value = match user_info_res {
        Ok(user_info) => match user_info {
            None => {
                code = 0;
                json!({})
            }
            Some(user) => {
                if user.password == frm.password {
                    json!({ "token" :user.token,
                            "user_id" :user.user_id,
                            "order_num" :0,
                            "order_amount" :0,
                            "decline_rate" :0 })
                } else {
                    code - 0;
                    json!({ "token":"","user_id":""})
                }
            }
        },
        Err(e) => {
            let res = ReturnJSON {
                code: 0,
                error: e.to_string(),
                data: json!({}),
            };

            return (headers, Json(res));
        }
    };

    let res = ReturnJSON {
        code: code,
        error: "".to_string(),
        data: value,
    };

    (headers, Json(res))
}

/// POST
pub async fn register_user(
    Extension(state): Extension<Arc<AppState>>,
    Json(frm): Json<UserNumPwd>,
) -> (HeaderMap, Json<ReturnJSON>) {
    let mut code = 1;
    let mut error = "";
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("content-type"),
        HeaderValue::from_static("text/plain;charset=utf-8"),
    );
    let conn = get_conn(&state);

    // let user = user_info::ActiveModel {
    //     number: Set(frm.number),
    //     password: Set(frm.password),
    //     token: Set(jsonwebtoken::generate_token(frm.number.clone(), frm.password).unwrap()),
    //     ..Default::default()
    // };

    // let add_user: Result<user_info::Model, sea_orm::DbErr> = user.insert(conn).await;
    // match add_user {
    //     Ok(u) => {}
    // }
    todo!()
}
