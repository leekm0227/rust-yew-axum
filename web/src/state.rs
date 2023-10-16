use serde::{Deserialize, Serialize};
use yewdux::store::Store;

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Store)]
#[store(storage = "local")]
pub struct AppState {
    pub token: Option<String>,
    pub nickname: Option<String>,
}

impl AppState {
    pub fn is_login(&self) -> bool {
        self.token.is_some()
    }

    pub fn login(&mut self, token: String) {
        self.token = Some(token);
    }

    pub fn logout(&mut self) {
        self.token = None;
    }
}
