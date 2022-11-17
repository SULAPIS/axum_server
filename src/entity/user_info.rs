use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "user_info")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub user_id: i32,
    pub number: String,
    pub password: String,
    pub token: String,
    pub register: bool,
}

#[derive(Debug, Clone, Copy, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> sea_orm::RelationDef {
        panic!("没有定义关系")
    }
}

impl ActiveModelBehavior for ActiveModel {}
