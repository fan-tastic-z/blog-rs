use axum::{
    extract::{Query, State},
    http::StatusCode,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    domain::blog::{
        models::posts::{ListPostError, ListPostRequest, ListPostResponse, Post},
        ports::BlogService,
    },
    inbound::http::{
        http_server::AppState,
        response::{ApiError, ApiSuccess},
    },
};

#[derive(Debug, Clone, Deserialize)]
pub struct ListPostHttpRequestBody {
    pub offset: u32,
    pub limit: u32,
}

impl ListPostHttpRequestBody {
    fn try_into_domain(self) -> Result<ListPostRequest, ListPostError> {
        let req = ListPostRequest::new(self.offset, self.limit)?;
        Ok(req)
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct PostInfo {
    pub post_id: String,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<&Post> for PostInfo {
    fn from(post: &Post) -> Self {
        Self {
            post_id: post.id.to_string(),
            title: post.title.clone(),
            content: post.content.clone(),
            created_at: post.created_at,
            updated_at: post.updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ListPostHttpResponseBody {
    pub total: u64,
    pub posts: Vec<PostInfo>,
}

impl From<&ListPostResponse> for ListPostHttpResponseBody {
    fn from(res: &ListPostResponse) -> Self {
        Self {
            total: res.total,
            posts: res.posts.iter().map(PostInfo::from).collect(),
        }
    }
}

pub async fn list_post<BS: BlogService>(
    State(state): State<AppState<BS>>,
    Query(body): Query<ListPostHttpRequestBody>,
) -> Result<ApiSuccess<ListPostHttpResponseBody>, ApiError> {
    let domain_req = body.try_into_domain()?;
    state
        .blog_service
        .list_post(domain_req)
        .await
        .map_err(ApiError::from)
        .map(|ref res| ApiSuccess::new(StatusCode::OK, res.into()))
}
