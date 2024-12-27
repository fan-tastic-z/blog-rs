use crate::{
    domain::blog::{error::Error, ports::BlogService},
    inbound::http::{http_server::AppState, response::ApiError},
    utils::jwt,
};
use axum::{
    extract::{Request, State},
    http::HeaderMap,
    middleware::Next,
    response::Response,
};
use serde::{Deserialize, Serialize};

const TOKEN_PREFIX: &str = "Bearer ";
const AUTH_HEADER: &str = "authorization";

#[derive(Debug, Deserialize, Serialize)]
pub struct JwtWithUser {
    pub claims: jwt::UserClaims,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JWT {
    pub claims: jwt::UserClaims,
}

pub async fn auth_middleware<BS: BlogService>(
    State(state): State<AppState<BS>>,
    request: Request,
    next: Next,
) -> Result<Response, ApiError> {
    let headers = request.headers();
    let token = extract_token(headers).map_err(ApiError::from)?;
    let _ = jwt_validate(&token, &state.config.auth.secret).map_err(ApiError::from)?;
    Ok(next.run(request).await)
}

fn jwt_validate(token: &str, secret: &str) -> Result<jwt::UserClaims, Error> {
    let jwt = jwt::JWT::new(secret);
    let claims = jwt.validate(token)?;
    Ok(claims.claims)
}

fn extract_token(headers: &HeaderMap) -> Result<String, Error> {
    let res = headers
        .get(AUTH_HEADER)
        .ok_or(Error::Unauthorized(format!(
            "header {AUTH_HEADER} token not found"
        )))?
        .to_str()
        .map_err(|err| Error::Unauthorized(err.to_string()))?
        .strip_prefix(TOKEN_PREFIX)
        .ok_or_else(|| Error::Unauthorized(format!("error strip {AUTH_HEADER} value")))?
        .to_string();
    Ok(res)
}
