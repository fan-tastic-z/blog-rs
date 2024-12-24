use std::future::Future;

use super::models::posts::{
    CreatePostError, CreatePostRequest, DeletePostError, DeletePostRequest, ListPostError,
    ListPostRequest, ListPostResponse, Post, UpdatePostRequest,
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

    fn delete_post(
        &self,
        req: &DeletePostRequest,
    ) -> impl Future<Output = Result<(), DeletePostError>> + Send;
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

    fn delete_post(
        &self,
        req: &DeletePostRequest,
    ) -> impl Future<Output = Result<(), DeletePostError>> + Send;
}
