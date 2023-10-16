use diesel::prelude::*;

use crate::config::db::PgClient;
use crate::model::Item;
use crate::schema::items;

// for item
impl PgClient {
    pub async fn item_list(&self) -> Option<Vec<Item>> {
        let list = items::table
            .filter(items::is_active.eq(true))
            .select(Item::as_select())
            .load(&mut self.get_conn())
            .unwrap();

        Some(list)
    }
}
