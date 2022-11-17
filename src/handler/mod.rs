use sea_orm::DatabaseConnection;

use crate::state::AppState;

pub mod category;
fn get_conn<'a>(state: &'a AppState) -> &'a DatabaseConnection {
    &state.conn
}
