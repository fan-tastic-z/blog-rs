use crate::{
    domain::blog::{
        error::Error,
        models::users::{GetUserByIdRequest, User},
        ports::BlogService,
    },
    inbound::http::{http_server::AppState, response::ApiError},
    utils::jwt,
};
use axum::{
    extract::{Request, State},
    http::HeaderMap,
    middleware::Next,
    response::Response,
};

const TOKEN_PREFIX: &str = "Bearer ";
const AUTH_HEADER: &str = "authorization";

pub async fn auth_middleware<BS: BlogService>(
    State(state): State<AppState<BS>>,
    mut request: Request,
    next: Next,
) -> Result<Response, ApiError> {
    let headers = request.headers();
    let token = extract_token(headers).map_err(ApiError::from)?;
    let user_claims = jwt_validate(&token, &state.config.auth.secret).map_err(ApiError::from)?;
    let user = get_user(state.clone(), &user_claims.pid)
        .await
        .map_err(ApiError::from)?;

    request.extensions_mut().insert(user);
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

async fn get_user<BS: BlogService>(state: AppState<BS>, user_id: &str) -> Result<User, Error> {
    let user = state
        .blog_service
        .get_user_by_id(&GetUserByIdRequest::new(user_id.to_string())?)
        .await
        .map_err(|_| Error::Unauthorized("invalid token".to_string()))?;
    Ok(user)
}
