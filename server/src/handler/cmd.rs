use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CmdRegister {
    pub account: String,
    pub nickname: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]

pub struct CmdLogin {
    pub account: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CmdToken {
    pub refresh_token: String,
}
