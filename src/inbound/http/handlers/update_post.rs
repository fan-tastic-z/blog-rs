use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    domain::blog::{
        error::Error,
        models::{
            posts::{Post, UpdatePostRequest},
            users::User,
        },
        ports::BlogService,
    },
    inbound::http::{
        http_server::AppState,
        response::{ApiError, ApiSuccess},
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct UpdatePostHttpRequestBody {
    pub title: String,
    pub content: String,
}

impl UpdatePostHttpRequestBody {
    fn try_into_domain(self, id: String, username: &str) -> Result<UpdatePostRequest, Error> {
        let req = UpdatePostRequest::new(id, self.title, self.content, username.to_string())?;
        Ok(req)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UpdatePostResponseData {
    pub id: String,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<&Post> for UpdatePostResponseData {
    fn from(post: &Post) -> Self {
        Self {
            id: post.id.to_string(),
            title: post.title.clone(),
            content: post.content.clone(),
            created_at: post.created_at,
            updated_at: post.updated_at,
        }
    }
}

pub async fn update_post<BS: BlogService>(
    Extension(user): Extension<User>,
    State(state): State<AppState<BS>>,
    Path(id): Path<String>,
    Json(body): Json<UpdatePostHttpRequestBody>,
) -> Result<ApiSuccess<UpdatePostResponseData>, ApiError> {
    let domain_req = body.try_into_domain(id, &user.username)?;
    state
        .blog_service
        .update_post(&domain_req)
        .await
        .map_err(ApiError::from)
        .map(|ref post| ApiSuccess::new(StatusCode::OK, post.into()))
}
