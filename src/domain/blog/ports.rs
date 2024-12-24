use std::future::Future;

use super::models::posts::{
    CreatePostError, CreatePostRequest, ListPostError, ListPostRequest, ListPostResponse, Post,
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
}
