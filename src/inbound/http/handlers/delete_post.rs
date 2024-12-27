use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension,
};
use serde::Deserialize;

use crate::{
    domain::blog::{
        error::Error,
        models::{posts::DeletePostRequest, users::User},
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
    pub fn try_into_domain(self, username: &str) -> Result<DeletePostRequest, Error> {
        let req = DeletePostRequest::new(self.id, username.to_string())?;
        Ok(req)
    }
}

pub async fn delete_post<BS: BlogService>(
    Extension(user): Extension<User>,
    State(state): State<AppState<BS>>,
    Path(body): Path<DeletePostHttpRequest>,
) -> Result<ApiSuccess<()>, ApiError> {
    let domain_req = body.try_into_domain(&user.username)?;
    state
        .blog_service
        .delete_post(&domain_req)
        .await
        .map_err(ApiError::from)
        .map(|_| ApiSuccess::new(StatusCode::NO_CONTENT, ()))
}
