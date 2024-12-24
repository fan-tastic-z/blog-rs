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
    pub updated_at: DateTime<Utc>,
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

#[derive(Debug, Clone, Validate)]
pub struct ListPostRequest {
    #[validate(range(min = 0))]
    pub offset: u32,
    #[validate(range(min = 1, max = 50))]
    pub limit: u32,
}

impl ListPostRequest {
    pub fn new(offset: u32, limit: u32) -> Result<Self, ListPostError> {
        let req = Self { offset, limit };
        req.validate()?;
        Ok(req)
    }
}

#[derive(Debug, Clone)]
pub struct ListPostResponse {
    pub total: u64,
    pub posts: Vec<Post>,
}

#[derive(Debug, Error)]
pub enum ListPostError {
    #[error("list post request validate error")]
    ValidationError(#[from] validator::ValidationErrors),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

impl From<ListPostError> for ApiError {
    fn from(e: ListPostError) -> Self {
        match e {
            ListPostError::ValidationError(err) => ApiError::UnprocessableEntity(err.to_string()),
            ListPostError::Unknown(err) => {
                tracing::error!("{:?}\n{}", err, err.backtrace());
                ApiError::InternalServerError(err.to_string())
            }
        }
    }
}

#[derive(Debug, Clone, Validate)]
pub struct UpdatePostRequest {
    pub id: String,
    #[validate(length(min = 1, max = 50))]
    pub title: String,
    #[validate(length(min = 1))]
    pub content: String,
}

impl UpdatePostRequest {
    pub fn new(id: String, title: String, content: String) -> Result<Self, UpdatePostError> {
        let req = Self { id, title, content };
        req.validate()?;
        Ok(req)
    }
}

#[derive(Debug, Error)]
pub enum UpdatePostError {
    #[error("update post request validate error")]
    ValidationError(#[from] validator::ValidationErrors),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

impl From<UpdatePostError> for ApiError {
    fn from(e: UpdatePostError) -> Self {
        match e {
            UpdatePostError::ValidationError(err) => ApiError::UnprocessableEntity(err.to_string()),
            UpdatePostError::Unknown(err) => {
                tracing::error!("{:?}\n{}", err, err.backtrace());
                ApiError::InternalServerError(err.to_string())
            }
        }
    }
}

#[derive(Debug, Clone, Validate)]
pub struct DeletePostRequest {
    pub id: String,
}

impl DeletePostRequest {
    pub fn new(id: String) -> Result<Self, DeletePostError> {
        let req = Self { id };
        req.validate()?;
        Ok(req)
    }
}

#[derive(Debug, Error)]
pub enum DeletePostError {
    #[error("delete post request validate error")]
    ValidationError(#[from] validator::ValidationErrors),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

impl From<DeletePostError> for ApiError {
    fn from(e: DeletePostError) -> Self {
        match e {
            DeletePostError::ValidationError(err) => ApiError::UnprocessableEntity(err.to_string()),
            DeletePostError::Unknown(err) => {
                tracing::error!("{:?}\n{}", err, err.backtrace());
                ApiError::InternalServerError(err.to_string())
            }
        }
    }
}

#[derive(Debug, Clone, Validate)]
pub struct BatchDeletePostRequest {
    #[validate(length(min = 1))]
    pub ids: Vec<String>,
}

impl BatchDeletePostRequest {
    pub fn new(ids: Vec<String>) -> Result<Self, BatchDeletePostError> {
        let req = Self { ids };
        req.validate()?;
        Ok(req)
    }
}

#[derive(Debug, Error)]
pub enum BatchDeletePostError {
    #[error("batch delete post request validate error")]
    ValidationError(#[from] validator::ValidationErrors),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

impl From<BatchDeletePostError> for ApiError {
    fn from(e: BatchDeletePostError) -> Self {
        match e {
            BatchDeletePostError::ValidationError(err) => {
                ApiError::UnprocessableEntity(err.to_string())
            }
            BatchDeletePostError::Unknown(err) => {
                tracing::error!("{:?}\n{}", err, err.backtrace());
                ApiError::InternalServerError(err.to_string())
            }
        }
    }
}
