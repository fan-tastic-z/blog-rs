use chrono::{DateTime, Utc};
use validator::Validate;

use crate::domain::blog::error::Error;

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
    pub username: String,
}

impl CreatePostRequest {
    pub fn new(title: String, content: String, username: String) -> Result<Self, Error> {
        let req = Self {
            title,
            content,
            username,
        };
        req.validate()?;
        Ok(req)
    }
}

#[derive(Debug, Clone, Validate)]
pub struct ListPostRequest {
    #[validate(range(min = 0))]
    pub offset: u32,
    #[validate(range(min = 1, max = 50))]
    pub limit: u32,
    pub username: String,
}

impl ListPostRequest {
    pub fn new(offset: u32, limit: u32, username: String) -> Result<Self, Error> {
        let req = Self {
            offset,
            limit,
            username,
        };
        req.validate()?;
        Ok(req)
    }
}

#[derive(Debug, Clone)]
pub struct ListPostResponse {
    pub total: u64,
    pub posts: Vec<Post>,
}

#[derive(Debug, Clone, Validate)]
pub struct UpdatePostRequest {
    pub id: String,
    #[validate(length(min = 1, max = 50))]
    pub title: String,
    #[validate(length(min = 1))]
    pub content: String,
    pub username: String,
}

impl UpdatePostRequest {
    pub fn new(
        id: String,
        title: String,
        content: String,
        username: String,
    ) -> Result<Self, Error> {
        let req = Self {
            id,
            title,
            content,
            username,
        };
        req.validate()?;
        Ok(req)
    }
}

#[derive(Debug, Clone, Validate)]
pub struct DeletePostRequest {
    pub id: String,
    pub username: String,
}

impl DeletePostRequest {
    pub fn new(id: String, username: String) -> Result<Self, Error> {
        let req = Self { id, username };
        req.validate()?;
        Ok(req)
    }
}

#[derive(Debug, Clone, Validate)]
pub struct BatchDeletePostRequest {
    #[validate(length(min = 1))]
    pub ids: Vec<String>,
    pub username: String,
}

impl BatchDeletePostRequest {
    pub fn new(ids: Vec<String>, username: String) -> Result<Self, Error> {
        let req = Self { ids, username };
        req.validate()?;
        Ok(req)
    }
}
