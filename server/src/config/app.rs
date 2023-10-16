use super::{Config, db::PgClient};

#[derive(Clone)]
pub struct AppState {
    pub db: PgClient,
    pub config: Config,
}

impl AppState {
    pub fn new() -> Self {
        let config = Config::init();

        AppState {
            db: PgClient::new(&config.database_url),
            config,
        }
    }
}
