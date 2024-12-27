use axum::{extract::State, http::StatusCode, Extension, Json};
use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};

use crate::{
    domain::blog::{
        error::Error,
        models::{
            posts::{CreatePostRequest, Post},
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
pub struct CreatePostHttpRequestBody {
    pub title: String,
    pub content: String,
}

impl CreatePostHttpRequestBody {
    fn try_into_domain(self, username: &str) -> Result<CreatePostRequest, Error> {
        let req = CreatePostRequest::new(self.title, self.content, username.to_string())?;
        Ok(req)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CreatePostResponseData {
    pub id: String,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

impl From<&Post> for CreatePostResponseData {
    fn from(post: &Post) -> Self {
        Self {
            id: post.id.to_string(),
            title: post.title.clone(),
            content: post.content.clone(),
            created_at: post.created_at,
        }
    }
}

pub async fn create_post<BS: BlogService>(
    Extension(user): Extension<User>,
    State(state): State<AppState<BS>>,
    Json(body): Json<CreatePostHttpRequestBody>,
) -> Result<ApiSuccess<CreatePostResponseData>, ApiError> {
    let domain_req = body.try_into_domain(&user.username)?;
    state
        .blog_service
        .create_post(&domain_req)
        .await
        .map_err(ApiError::from)
        .map(|ref post| ApiSuccess::new(StatusCode::CREATED, post.into()))
}
