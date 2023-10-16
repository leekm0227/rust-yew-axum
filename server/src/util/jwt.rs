use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::model::User;

const JWT_SECRET: &[u8] = "jwt_secret_key".as_bytes();
const JWT_EXPIRED_IN: i64 = 60;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub iss: i32,
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

pub fn get_refresh_token() -> String {
    Uuid::new_v4().to_string().replace("-", "")
}

pub fn get_jwt(user: User) -> String {
    let now = Utc::now();
    let claims = Claims {
        iss: user.user_id,
        sub: user.nickname.unwrap_or_default(),
        exp: (now + Duration::minutes(JWT_EXPIRED_IN)).timestamp() as usize,
        iat: now.timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET),
    )
    .unwrap()
}

pub fn get_claims(token: &str) -> Option<Claims> {
    let result = decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::default(),
    );

    match result {
        Ok(data) => Some(data.claims),
        Err(_) => None,
    }
}
