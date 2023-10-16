use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

use crate::schema::*;

#[derive(Deserialize, Debug, Identifiable, Queryable, Insertable, Selectable, AsChangeset)]
#[diesel(primary_key(user_id))]
#[diesel(table_name = users)]
pub struct User {
    pub user_id: i32,
    pub account: String,
    pub password: String,
    pub nickname: Option<String>,
    pub coin: i32,
    pub refresh_token: Option<String>,
    pub last_login_time: Option<SystemTime>,
    pub update_time: Option<SystemTime>,
    pub create_time: Option<SystemTime>,
}

#[derive(
    Serialize, Deserialize, Debug, Identifiable, Queryable, Insertable, Selectable, AsChangeset,
)]
#[diesel(primary_key(item_id))]
#[diesel(table_name = items)]
pub struct Item {
    pub item_id: i32,
    pub item_name: String,
    pub item_type: i32,
    pub price: i32,
    #[serde(skip_serializing)]
    pub is_active: bool,
    #[serde(skip_serializing)]
    pub update_time: Option<SystemTime>,
    #[serde(skip_serializing)]
    pub create_time: Option<SystemTime>,
}
