use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    domain::blog::{error::Error, models::users::LoginRequest, ports::BlogService},
    inbound::http::{
        http_server::AppState,
        response::{ApiError, ApiSuccess},
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct LoginHttpRequest {
    pub username: String,
    pub password: String,
}

impl LoginHttpRequest {
    pub fn try_into_domain(
        self,
        jwt_secret: String,
        expiration: u64,
    ) -> Result<LoginRequest, Error> {
        let req = LoginRequest::new(self.username, self.password, jwt_secret, expiration)?;
        req.validate()?;
        Ok(req)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LoginHttpResponseData {
    pub token: String,
}

pub async fn login<BS: BlogService>(
    State(state): State<AppState<BS>>,
    Json(body): Json<LoginHttpRequest>,
) -> Result<ApiSuccess<LoginHttpResponseData>, ApiError> {
    let jwt_expiration = state.config.auth.expiration;
    let jwt_secret = state.config.auth.secret.clone();
    let domain_req = body.try_into_domain(jwt_secret, jwt_expiration)?;
    state
        .blog_service
        .login(&domain_req)
        .await
        .map_err(ApiError::from)
        .map(|token| ApiSuccess::new(StatusCode::OK, LoginHttpResponseData { token }))
}
