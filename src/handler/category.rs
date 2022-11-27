use super::get_conn;
use crate::{
    entity::{category, order_detail, order_info, user_info},
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
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, ModelTrait, QueryFilter,
    QueryOrder, Set,
};
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
        HeaderValue::from_static("application/json;charset=utf-8"),
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
                            "avatar":"https://static.runoob.com/images/demo/demo2.jpg",
                            "decline_rate" :0 })
                } else {
                    code = 2;
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
    let mut error = "".to_string();
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("content-type"),
        HeaderValue::from_static("application/json;charset=utf-8"),
    );
    let conn = get_conn(&state);

    let num = frm.number.clone();
    let pwd = frm.password.clone();
    let user = user_info::ActiveModel {
        number: Set(frm.number),
        password: Set(frm.password),
        token: Set(jsonwebtoken::generate_token(num, pwd).unwrap()),
        ..Default::default()
    };

    let add_user: Result<user_info::Model, sea_orm::DbErr> = user.insert(conn).await;
    let value: Value = match add_user {
        Ok(u) => {
            json!({ "token" :u.token,
                "user_id" :u.user_id,
                "order_num" :0,
                "order_amount" :0,
                "decline_rate" :0
            })
        }
        Err(e) => {
            code = 0;
            error = e.to_string();
            json!({})
        }
    };
    let res = ReturnJSON {
        code,
        error,
        data: value,
    };

    (headers, Json(res))
}

pub async fn add_order(
    Extension(state): Extension<Arc<AppState>>,
    header: HeaderMap,
    Json(frm): Json<Value>,
) -> (HeaderMap, Json<ReturnJSON>) {
    let mut code = 1;
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("content-type"),
        HeaderValue::from_static("application/json;charset=utf-8"),
    );
    let conn = get_conn(&state);
    let user_id = get_user_id(&header, conn).await;
    let data = frm["data"].clone();
    let car_type: i32 = frm["cartype"].to_string().parse().unwrap();

    let value = match car_type {
        0 => {
            let ton: f32 = data["ton"].to_string().parse().unwrap();
            let c_type = data["type"].to_string().replace("\"", "");
            let rent: f32 = data["rent"].to_string().parse().unwrap();
            let address = data["address"].to_string().replace("\"", "");
            let time = data["time"].to_string().replace("\"", "");

            let order = order_info::ActiveModel {
                user_id: Set(user_id),
                cartype: Set(0),
                ton: Set(ton),
                c_type: Set(c_type),
                rent: Set(rent),
                address: Set(address),
                c_time: Set(time),
                ..Default::default()
            };

            order.insert(conn).await.unwrap();
            json!({})
        }
        1 => {
            let c_type = data["type"].to_string().replace("\"", "");
            let rent: f32 = data["rent"].to_string().parse().unwrap();
            let address = data["address"].to_string().replace("\"", "");
            let time = data["time"].to_string().replace("\"", "");

            let order = order_info::ActiveModel {
                user_id: Set(user_id),
                cartype: Set(1),
                c_type: Set(c_type),
                rent: Set(rent),
                address: Set(address),
                c_time: Set(time),
                ..Default::default()
            };

            order.insert(conn).await.unwrap();
            json!({})
        }
        2 => {
            let c_type = data["type"].to_string().replace("\"", "");
            let address = data["address"].to_string().replace("\"", "");
            let time = data["time"].to_string().replace("\"", "");

            let order = order_info::ActiveModel {
                user_id: Set(user_id),
                cartype: Set(2),
                c_type: Set(c_type),
                address: Set(address),
                c_time: Set(time),
                ..Default::default()
            };

            order.insert(conn).await.unwrap();
            json!({})
        }
        3 => {
            let c_type = data["type"].to_string().replace("\"", "");
            let address = data["address"].to_string().replace("\"", "");
            let time = data["time"].to_string().replace("\"", "");

            let order = order_info::ActiveModel {
                user_id: Set(user_id),
                cartype: Set(3),
                c_type: Set(c_type),
                address: Set(address),
                c_time: Set(time),
                ..Default::default()
            };

            order.insert(conn).await.unwrap();
            json!({})
        }
        _ => unreachable!(),
    };
    let res = ReturnJSON {
        code: code,
        error: "".to_string(),
        data: value,
    };

    (headers, Json(res))
}

pub async fn get_order(
    Extension(state): Extension<Arc<AppState>>,
    header: HeaderMap,
) -> (HeaderMap, Json<Value>) {
    let mut code = 1;
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("content-type"),
        HeaderValue::from_static("application/json;charset=utf-8"),
    );
    let conn = get_conn(&state);
    let user_id = get_user_id(&header, conn).await;
    let res = order_info::Entity::find()
        .filter(order_info::Column::UserId.eq(user_id))
        .all(conn)
        .await
        .unwrap();
    let mut data = Vec::new();
    for m in res {
        let v = match m.cartype {
            0 => {
                let r = f32::trunc(m.rent * 100.0) / 100.0;
                let t = f32::trunc(m.ton * 100.0) / 100.0;
                json!({
                    "order_id":m.order_id,
                    "cartype":0,
                    "data":{
                        "ton":format!("{:.2}",t),
                        "type":m.c_type,
                        "rent":format!("{:.2}",r),
                        "address":m.address,
                        "time":m.c_time
                    },
                    "state":m.c_state
                })
            }
            1 => {
                let r = f32::trunc(m.rent * 100.0) / 100.0;
                json!({
                    "order_id":m.order_id,
                    "cartype":1,
                    "data":{
                        "type":m.c_type,
                        "rent":format!("{:.2}",r),
                        "address":m.address,
                        "time":m.c_time
                    },
                    "state":m.c_state
                })
            }
            2 => {
                json!({
                    "order_id":m.order_id,
                    "cartype":2,
                    "data":{
                        "type":m.c_type,
                        "address":m.address,
                        "time":m.c_time
                    },
                    "state":m.c_state
                })
            }
            3 => {
                json!({
                    "order_id":m.order_id,
                    "cartype":3,
                    "data":{
                        "type":m.c_type,
                        "address":m.address,
                        "time":m.c_time
                    },
                    "state":m.c_state
                })
            }
            _ => unreachable!(),
        };
        data.push(v);
    }
    let v = json!({
        "code":1,
        "data":data
   } );

    (headers, Json(v))
}

pub async fn change_state(
    Extension(state): Extension<Arc<AppState>>,
    Json(var): Json<Value>,
) -> (HeaderMap, Json<Value>) {
    let mut code = 1;
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("content-type"),
        HeaderValue::from_static("application/json;charset=utf-8"),
    );

    let order_id: i32 = var["order_id"].to_string().parse().unwrap();
    let order_state: i32 = var["state"].to_string().parse().unwrap();
    let conn = get_conn(&state);
    let mut res: order_info::ActiveModel = order_info::Entity::find()
        .filter(order_info::Column::OrderId.eq(order_id))
        .one(conn)
        .await
        .unwrap()
        .unwrap()
        .into();
    res.c_state = Set(order_state);
    res.update(conn).await.unwrap();

    let v = json!({
        "code":1,
        "data":{}
   } );

    (headers, Json(v))
}

pub async fn save_order_details(
    Extension(state): Extension<Arc<AppState>>,
    header: HeaderMap,
    Json(var): Json<Value>,
) -> (HeaderMap, Json<Value>) {
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("content-type"),
        HeaderValue::from_static("application/json;charset=utf-8"),
    );
    let conn = get_conn(&state);
    let user_id = get_user_id(&header, conn).await;

    let details = var.to_string();
    let order = order_detail::ActiveModel {
        user_id: Set(user_id),
        detail: Set(details),
        ..Default::default()
    };
    let o = order.insert(conn).await.unwrap();
    let v: Value = serde_json::from_str(&o.detail).unwrap();
    let value = json!({
        "code":1,
        "order_id":o.order_id,
        "state":o.c_state,
        "detail":v

    });
    (headers, Json(value))
}

pub async fn get_order_details(
    Extension(state): Extension<Arc<AppState>>,
    header: HeaderMap,
) -> (HeaderMap, Json<Value>) {
    let mut code = 1;
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("content-type"),
        HeaderValue::from_static("application/json;charset=utf-8"),
    );
    let conn = get_conn(&state);
    let user_id = get_user_id(&header, conn).await;
    let res = order_detail::Entity::find()
        .filter(order_detail::Column::UserId.eq(user_id))
        .all(conn)
        .await
        .unwrap();
    let mut data = Vec::new();

    for o in res {
        let v: Value = serde_json::from_str(&o.detail).unwrap();
        data.push(json!(
            {
                "order_id":o.order_id,
                "state":o.c_state,
                "detail":v
            }
        ));
    }

    let v = json!({
        "code":1,
        "data":data
   } );

    (headers, Json(v))
}

pub async fn change_state_details(
    Extension(state): Extension<Arc<AppState>>,
    Json(var): Json<Value>,
) -> (HeaderMap, Json<Value>) {
    let mut code = 1;
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("content-type"),
        HeaderValue::from_static("application/json;charset=utf-8"),
    );

    let order_id: i32 = var["order_id"].to_string().parse().unwrap();
    let order_state: i32 = var["state"].to_string().parse().unwrap();
    let conn = get_conn(&state);
    let mut res: order_detail::ActiveModel = order_detail::Entity::find()
        .filter(order_detail::Column::OrderId.eq(order_id))
        .one(conn)
        .await
        .unwrap()
        .unwrap()
        .into();
    res.c_state = Set(order_state);
    let o = res.update(conn).await.unwrap();

    let v: Value = serde_json::from_str(&o.detail).unwrap();
    let value = json!({
        "code":1,
        "order_id":o.order_id,
        "state":o.c_state,
        "detail":v

    });
    (headers, Json(value))
}

pub async fn get_supply_order(
    Extension(state): Extension<Arc<AppState>>,
    header: HeaderMap,
) -> (HeaderMap, Json<Value>) {
    let mut code = 1;
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("content-type"),
        HeaderValue::from_static("application/json;charset=utf-8"),
    );
    let conn = get_conn(&state);
    let user_id = get_user_id(&header, conn).await;
    let res = order_detail::Entity::find()
        .filter(order_detail::Column::UserId.ne(user_id))
        .filter(order_detail::Column::User2Id.eq(user_id))
        .all(conn)
        .await
        .unwrap();

    let mut data = Vec::new();

    for o in res {
        let v: Value = serde_json::from_str(&o.detail).unwrap();
        let number = get_user_number(o.user_id, conn).await;
        data.push(json!(
            {
                "order_id":o.order_id,
                "state":o.c_state,
                "detail":v,
                "number":number
            }
        ));
    }

    let v = json!({
            "code":1,
            "data":data
       } );

    (headers, Json(v))
}

pub async fn get_supply_order_null(
    Extension(state): Extension<Arc<AppState>>,
    header: HeaderMap,
) -> (HeaderMap, Json<Value>) {
    let mut code = 1;
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("content-type"),
        HeaderValue::from_static("application/json;charset=utf-8"),
    );
    let conn = get_conn(&state);
    let user_id = get_user_id(&header, conn).await;
    let res_null = order_detail::Entity::find()
        .filter(order_detail::Column::UserId.ne(user_id))
        .filter(order_detail::Column::User2Id.is_null())
        .all(conn)
        .await
        .unwrap();

    let mut data_null = Vec::new();

    for o in res_null {
        let v: Value = serde_json::from_str(&o.detail).unwrap();
        let number = get_user_number(o.user_id, conn).await;
        data_null.push(json!(
            {
                "order_id":o.order_id,
                "state":o.c_state,
                "detail":v,
                "number":number
            }
        ));
    }

    let v = json!({
            "code":1,
            "data":data_null
       } );

    (headers, Json(v))
}

pub async fn receive_order(
    Extension(state): Extension<Arc<AppState>>,
    header: HeaderMap,
    Json(var): Json<Value>,
) -> (HeaderMap, Json<Value>) {
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("content-type"),
        HeaderValue::from_static("application/json;charset=utf-8"),
    );
    let conn = get_conn(&state);
    let user_id = get_user_id(&header, conn).await;
    let order_id: i32 = var["order_id"].to_string().parse().unwrap();

    let mut res: order_detail::ActiveModel = order_detail::Entity::find()
        .filter(order_detail::Column::OrderId.eq(order_id))
        .one(conn)
        .await
        .unwrap()
        .unwrap()
        .into();
    res.user2_id = Set(Some(user_id));
    res.c_state = Set(1);
    let o = res.update(conn).await.unwrap();

    let v: Value = serde_json::from_str(&o.detail).unwrap();
    let value = json!({
        "code":1,
        "order_id":o.order_id,
        "state":o.c_state,
        "detail":v

    });
    (headers, Json(value))
}

pub async fn cancel_order(
    Extension(state): Extension<Arc<AppState>>,
    Json(var): Json<Value>,
) -> (HeaderMap, Json<Value>) {
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("content-type"),
        HeaderValue::from_static("application/json;charset=utf-8"),
    );
    let conn = get_conn(&state);
    let order_id: i32 = var["order_id"].to_string().parse().unwrap();

    let res: Result<Option<order_detail::Model>, DbErr> = order_detail::Entity::find()
        .filter(order_detail::Column::OrderId.eq(order_id))
        .one(conn)
        .await;
    let value = match res {
        Ok(res) => match res {
            Some(res) => {
                res.delete(conn).await.unwrap();
                json!({
                    "code":1

                })
            }
            None => {
                json!({
                    "code":1

                })
            }
        },
        Err(_) => {
            json!({
                "code":1

            })
        }
    };

    (headers, Json(value))
}

async fn get_user_id(headers: &HeaderMap, conn: &DatabaseConnection) -> i32 {
    let token = headers.get("token").unwrap().to_str().unwrap();
    let user: user_info::Model = user_info::Entity::find()
        .filter(user_info::Column::Token.eq(token))
        .one(conn)
        .await
        .unwrap()
        .unwrap();
    user.user_id
}
async fn get_user_number(user_id: i32, conn: &DatabaseConnection) -> String {
    user_info::Entity::find()
        .filter(user_info::Column::UserId.eq(user_id))
        .one(conn)
        .await
        .unwrap()
        .unwrap()
        .number
}
