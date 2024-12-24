use std::future::Future;

use super::models::posts::{
    CreatePostError, CreatePostRequest, ListPostError, ListPostRequest, ListPostResponse, Post,
    UpdatePostRequest,
};

pub trait BlogService: Clone + Send + Sync + 'static {
    fn create_post(
        &self,
        req: &CreatePostRequest,
    ) -> impl Future<Output = Result<Post, CreatePostError>> + Send;

    fn list_post(
        &self,
        req: ListPostRequest,
    ) -> impl Future<Output = Result<ListPostResponse, ListPostError>> + Send;

    fn update_post(
        &self,
        req: &UpdatePostRequest,
    ) -> impl Future<Output = Result<Post, CreatePostError>> + Send;
}

pub trait BlogRepository: Clone + Send + Sync + 'static {
    fn create_post(
        &self,
        req: &CreatePostRequest,
    ) -> impl Future<Output = Result<Post, CreatePostError>> + Send;

    fn list_post(
        &self,
        req: ListPostRequest,
    ) -> impl Future<Output = Result<ListPostResponse, ListPostError>> + Send;

    fn update_post(
        &self,
        req: &UpdatePostRequest,
    ) -> impl Future<Output = Result<Post, CreatePostError>> + Send;
}
