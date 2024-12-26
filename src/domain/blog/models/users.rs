use chrono::{DateTime, Utc};
use validator::Validate;

use crate::{domain::blog::error::Error, utils};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Validate)]
pub struct CreateUserRequest {
    #[validate(length(min = 1, max = 50))]
    pub username: String,
    #[validate(email)]
    pub email: Option<String>,
    pub phone: Option<String>,
    #[validate(length(min = 8, max = 18))]
    pub password: String,
}

impl CreateUserRequest {
    pub fn new(
        username: String,
        email: Option<String>,
        phone: Option<String>,
        password: String,
    ) -> Result<Self, Error> {
        let mut req = Self {
            username,
            email,
            phone,
            password,
        };
        req.validate()?;
        let password = utils::compute_password_hash(&req.password)?;
        req.password = password;
        Ok(req)
    }
}

#[derive(Debug, Clone, Validate)]
pub struct GetUserRequest {
    #[validate(length(min = 1, max = 50))]
    pub username: String,
}

impl GetUserRequest {
    pub fn new(username: String) -> Result<Self, Error> {
        let req = Self { username };
        req.validate()?;
        Ok(req)
    }
}
