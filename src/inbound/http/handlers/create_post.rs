use axum::{extract::State, http::StatusCode, Json};
use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};

use crate::{
    domain::blog::{
        models::posts::{CreatePostError, CreatePostRequest, Post},
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
    fn try_into_domain(self) -> Result<CreatePostRequest, CreatePostError> {
        let req = CreatePostRequest::new(self.title, self.content)?;
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
    State(state): State<AppState<BS>>,
    Json(body): Json<CreatePostHttpRequestBody>,
) -> Result<ApiSuccess<CreatePostResponseData>, ApiError> {
    let domain_req = body.try_into_domain()?;
    state
        .blog_service
        .create_post(&domain_req)
        .await
        .map_err(ApiError::from)
        .map(|ref post| ApiSuccess::new(StatusCode::CREATED, post.into()))
}

#[cfg(test)]
mod tests {
    use anyhow::anyhow;
    use std::{
        mem,
        sync::{Arc, Mutex},
    };
    use uuid::Uuid;

    use super::*;

    #[derive(Clone)]
    struct MockBlogService {
        create_post_result: Arc<Mutex<Result<Post, CreatePostError>>>,
    }

    impl BlogService for MockBlogService {
        async fn create_post(&self, _req: &CreatePostRequest) -> Result<Post, CreatePostError> {
            let mut guard = self.create_post_result.lock();
            let mut result = Err(CreatePostError::Unknown(anyhow!("substitute error")));
            mem::swap(guard.as_deref_mut().unwrap(), &mut result);
            result
        }
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_create_post_success() {
        let title = "title".to_string();
        let content = "content".to_string();
        let post_id = Uuid::new_v4().to_string();
        let created_at = Utc::now();
        let service = MockBlogService {
            create_post_result: Arc::new(Mutex::new(Ok(Post {
                id: post_id.clone(),
                title: title.clone(),
                content: content.clone(),
                created_at,
            }))),
        };
        let state = State(AppState {
            blog_service: Arc::new(service),
        });
        let body = Json(CreatePostHttpRequestBody {
            title: title.clone(),
            content: content.clone(),
        });
        let expected = ApiSuccess::new(
            StatusCode::CREATED,
            CreatePostResponseData {
                id: post_id.clone(),
                title,
                content,
                created_at,
            },
        );
        let actual = create_post(state, body).await;
        assert!(
            actual.is_ok(),
            "expected create_post to succeed, but got {:?}",
            actual
        );
        let actual = actual.unwrap();
        assert_eq!(
            actual, expected,
            "expected ApiSuccess {:?}, but got {:?}",
            expected, actual
        );
    }
}
