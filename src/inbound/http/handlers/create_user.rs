use axum::{extract::State, http::StatusCode, Json};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    domain::blog::{
        error::Error,
        models::users::{CreateUserRequest, User},
        ports::BlogService,
    },
    inbound::http::{
        http_server::AppState,
        response::{ApiError, ApiSuccess},
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CreateUserHttpRequest {
    pub username: String,
    pub password: String,
    pub email: Option<String>,
    pub phone: Option<String>,
}

impl CreateUserHttpRequest {
    pub fn try_into_domain(self) -> Result<CreateUserRequest, Error> {
        let req = CreateUserRequest::new(self.username, self.email, self.phone, self.password)?;
        Ok(req)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CreateUserResponseData {
    pub id: String,
    pub username: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl From<&User> for CreateUserResponseData {
    fn from(user: &User) -> Self {
        Self {
            id: user.id.to_string(),
            username: user.username.clone(),
            email: user.email.clone(),
            phone: user.phone.clone(),
            created_at: user.created_at,
        }
    }
}

pub async fn create_user<BS: BlogService>(
    State(state): State<AppState<BS>>,
    Json(body): Json<CreateUserHttpRequest>,
) -> Result<ApiSuccess<CreateUserResponseData>, ApiError> {
    let domain_req = body.try_into_domain()?;
    state
        .blog_service
        .create_user(&domain_req)
        .await
        .map_err(ApiError::from)
        .map(|ref user| ApiSuccess::new(StatusCode::CREATED, user.into()))
}
