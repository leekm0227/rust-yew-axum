use axum::{extract::State, Json};
use serde_json::{json, Value};

use crate::config::app::AppState;
use crate::util;

use super::cmd::*;

pub async fn register(State(app): State<AppState>, Json(cmd): Json<CmdRegister>) -> Json<Value> {
    // check duplicate account
    let is_dup_account = app.db.find_by_account(&cmd.account).await;
    if is_dup_account.is_some() {
        return Json(json!({ "success": false, "error":"중복된 아이디 입니다"}));
    }

    // regist
    let _ = app
        .db
        .register(cmd.account, cmd.nickname, util::hash(cmd.password))
        .await
        .unwrap();
    Json(json!({ "success": true }))
}

pub async fn login(State(app): State<AppState>, Json(cmd): Json<CmdLogin>) -> Json<Value> {
    // try login
    let refresh_token = util::jwt::get_refresh_token();
    let user = app
        .db
        .login(cmd.account, util::hash(cmd.password), refresh_token.clone())
        .await;

    if user.is_none() {
        return Json(json!({ "success": false, "error":"아이디/비밀번호를 확인해주세요"}));
    };

    // gen token
    let token = util::jwt::get_jwt(user.unwrap());
    Json(json!({ "success": true, "token": token, "refresh_token":refresh_token}))
}

pub async fn token(State(app): State<AppState>, Json(cmd): Json<CmdToken>) -> Json<Value> {
    // check refresh token
    let user = app.db.find_by_refresh_token(&cmd.refresh_token).await;
    if user.is_none() {
        return Json(json!({ "success": false, "error":"invalid refresh token"}));
    };

    // gen token
    let token = util::jwt::get_jwt(user.unwrap());
    Json(json!({ "success": true, "token": token}))
}
