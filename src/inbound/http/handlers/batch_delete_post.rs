use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;

use crate::{
    domain::blog::{error::Error, models::posts::BatchDeletePostRequest, ports::BlogService},
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
    pub fn try_into_domain(self) -> Result<BatchDeletePostRequest, Error> {
        let req = BatchDeletePostRequest::new(self.ids)?;
        Ok(req)
    }
}

pub async fn batch_delete_post<BS: BlogService>(
    State(state): State<AppState<BS>>,
    Json(body): Json<BatchDeletePostHttpRequest>,
) -> Result<ApiSuccess<()>, ApiError> {
    let domain_req = body.try_into_domain()?;
    state
        .blog_service
        .batch_delete_post(&domain_req)
        .await
        .map_err(ApiError::from)
        .map(|_| ApiSuccess::new(StatusCode::NO_CONTENT, ()))
}
