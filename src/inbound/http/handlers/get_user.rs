use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    domain::blog::{
        error::Error,
        models::users::{GetUserRequest, User},
        ports::BlogService,
    },
    inbound::http::{
        http_server::AppState,
        response::{ApiError, ApiSuccess},
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct GetUserHttpRequest {
    pub username: String,
}

impl GetUserHttpRequest {
    pub fn try_into_domain(self) -> Result<GetUserRequest, Error> {
        let req = GetUserRequest::new(self.username)?;
        Ok(req)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GetUserResponseData {
    pub id: String,
    pub username: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<&User> for GetUserResponseData {
    fn from(user: &User) -> Self {
        Self {
            id: user.id.to_string(),
            username: user.username.clone(),
            email: user.email.clone(),
            phone: user.phone.clone(),
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

pub async fn get_user<BS: BlogService>(
    State(state): State<AppState<BS>>,
    Path(body): Path<GetUserHttpRequest>,
) -> Result<ApiSuccess<GetUserResponseData>, ApiError> {
    let req = body.try_into_domain()?;
    state
        .blog_service
        .get_user(&req)
        .await
        .map_err(ApiError::from)
        .map(|ref user| ApiSuccess::new(StatusCode::OK, user.into()))
}
