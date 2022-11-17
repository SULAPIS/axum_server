use super::get_conn;
use crate::{
    entity::{category, user_info},
    form::form_entity::*,
    state::AppState,
};
use axum::{
    body::Body,
    http::{header::HeaderName, HeaderMap, HeaderValue},
    response::Json,
    routing::get,
    Extension, Form, Router,
};
use sea_orm::{ColumnTrait, DbErr, EntityTrait, QueryFilter, QueryOrder};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReturnJSON {
    pub code: u32,
    pub data: Value,
    pub error: String,
}

pub async fn index(Extension(state): Extension<Arc<AppState>>) -> Json<Value> {
    let conn = get_conn(&state);
    let categies: Vec<category::Model> = category::Entity::find()
        .order_by_asc(category::Column::Id)
        .all(conn)
        .await
        .unwrap();

    let res = serde_json::to_value(categies).unwrap();

    Json(res)
}

/// POST
pub async fn login_user(
    Extension(state): Extension<Arc<AppState>>,
    Json(frm): Json<UserLogin>,
) -> (HeaderMap, Json<ReturnJSON>) {
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
                json!({"register":false})
            }
            Some(user) => {
                if user.password == frm.password {
                    json!({"register":true,"token":user.token,"user_id":user.user_id})
                } else {
                    json!({"register":true,"token":"","user_id":""})
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
        code: 1,
        error: "".to_string(),
        data: value,
    };

    (headers, Json(res))
}
