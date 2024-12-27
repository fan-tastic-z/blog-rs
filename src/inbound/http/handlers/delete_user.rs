use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use serde::Deserialize;

use crate::{
    domain::blog::{error::Error, models::users::DeleteUserRequest, ports::BlogService},
    inbound::http::{
        http_server::AppState,
        response::{ApiError, ApiSuccess},
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct DeleteUserHttpRequest {
    pub username: String,
}

impl DeleteUserHttpRequest {
    pub fn try_into_domain(self) -> Result<DeleteUserRequest, Error> {
        let req = DeleteUserRequest::new(self.username)?;
        Ok(req)
    }
}

pub async fn delete_user<BS: BlogService>(
    State(state): State<AppState<BS>>,
    Path(id): Path<String>,
) -> Result<ApiSuccess<()>, ApiError> {
    let domain_req = DeleteUserRequest::new(id)?;
    state
        .blog_service
        .delete_user(&domain_req)
        .await
        .map_err(ApiError::from)
        .map(|_| ApiSuccess::new(StatusCode::NO_CONTENT, ()))
}
