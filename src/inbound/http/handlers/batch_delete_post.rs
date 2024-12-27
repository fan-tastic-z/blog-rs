use axum::{extract::State, http::StatusCode, Extension, Json};
use serde::Deserialize;

use crate::{
    domain::blog::{
        error::Error,
        models::{posts::BatchDeletePostRequest, users::User},
        ports::BlogService,
    },
    inbound::http::{
        http_server::AppState,
        response::{ApiError, ApiSuccess},
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct BatchDeletePostHttpRequest {
    pub ids: Vec<String>,
}

impl BatchDeletePostHttpRequest {
    pub fn try_into_domain(self, username: &str) -> Result<BatchDeletePostRequest, Error> {
        let req = BatchDeletePostRequest::new(self.ids, username.to_string())?;
        Ok(req)
    }
}

pub async fn batch_delete_post<BS: BlogService>(
    Extension(user): Extension<User>,
    State(state): State<AppState<BS>>,
    Json(body): Json<BatchDeletePostHttpRequest>,
) -> Result<ApiSuccess<()>, ApiError> {
    let domain_req = body.try_into_domain(&user.username)?;
    state
        .blog_service
        .batch_delete_post(&domain_req)
        .await
        .map_err(ApiError::from)
        .map(|_| ApiSuccess::new(StatusCode::NO_CONTENT, ()))
}
