use diesel::{prelude::*, r2d2::*};
use std::sync::Arc;

#[derive(Clone)]
pub struct PgClient {
    pub db_pool: Arc<Pool<ConnectionManager<PgConnection>>>,
}

impl PgClient {
    pub fn new(db_url: &str) -> Self {
        Self {
            db_pool: Arc::new(
                Pool::builder()
                    .build(ConnectionManager::<PgConnection>::new(db_url))
                    .expect("Failed to create pool."),
            ),
        }
    }

    pub fn get_conn(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
        self.db_pool.get().unwrap()
    }
}
