use chrono::{DateTime, Utc};
use thiserror::Error;
use validator::Validate;

use crate::inbound::http::response::ApiError;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Validate)]
pub struct CreatePostRequest {
    #[validate(length(min = 1, max = 50))]
    pub title: String,
    #[validate(length(min = 1))]
    pub content: String,
}

impl CreatePostRequest {
    pub fn new(title: String, content: String) -> Result<Self, CreatePostError> {
        let req = Self { title, content };
        req.validate()?;
        Ok(req)
    }
}

#[derive(Debug, Error)]
pub enum CreatePostError {
    #[error("create post request validate error")]
    ValidationError(#[from] validator::ValidationErrors),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

impl From<CreatePostError> for ApiError {
    fn from(e: CreatePostError) -> Self {
        match e {
            CreatePostError::ValidationError(e) => ApiError::UnprocessableEntity(e.to_string()),
            CreatePostError::Unknown(e) => {
                tracing::error!("{:?}\n{}", e, e.backtrace());
                ApiError::InternalServerError(e.to_string())
            }
        }
    }
}
