use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use serde::Deserialize;

use crate::{
    domain::blog::{
        models::posts::{DeletePostError, DeletePostRequest},
        ports::BlogService,
    },
    inbound::http::{
        http_server::AppState,
        response::{ApiError, ApiSuccess},
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct DeletePostHttpRequest {
    pub id: String,
}

impl DeletePostHttpRequest {
    pub fn try_into_domain(self) -> Result<DeletePostRequest, DeletePostError> {
        let req = DeletePostRequest::new(self.id)?;
        Ok(req)
    }
}

pub async fn delete_post<BS: BlogService>(
    State(state): State<AppState<BS>>,
    Path(id): Path<String>,
) -> Result<ApiSuccess<()>, ApiError> {
    let domain_req = DeletePostRequest::new(id)?;
    state
        .blog_service
        .delete_post(&domain_req)
        .await
        .map_err(ApiError::from)
        .map(|_| ApiSuccess::new(StatusCode::NO_CONTENT, ()))
}
