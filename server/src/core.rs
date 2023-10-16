use axum::{
    http::{HeaderMap, Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
};

use crate::util;

pub async fn jwt<B>(
    header_map: HeaderMap,
    mut request: Request<B>,
    next: Next<B>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if header_map.contains_key("token") {
        let token = header_map
            .get("token")
            .unwrap()
            .clone()
            .to_str()
            .unwrap_or_default()
            .to_string();

        let claims = util::jwt::get_claims(token.as_str());
        if claims.is_some() {
            request.extensions_mut().insert(claims.unwrap().iss);
        }
    }

    Ok(next.run(request).await)
}
