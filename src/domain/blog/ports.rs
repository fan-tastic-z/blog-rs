use std::future::Future;

use super::{
    error::Error,
    models::posts::{
        BatchDeletePostRequest, CreatePostRequest, DeletePostRequest, ListPostRequest,
        ListPostResponse, Post, UpdatePostRequest,
    },
};

pub trait BlogService: Clone + Send + Sync + 'static {
    fn create_post(
        &self,
        req: &CreatePostRequest,
    ) -> impl Future<Output = Result<Post, Error>> + Send;

    fn list_post(
        &self,
        req: ListPostRequest,
    ) -> impl Future<Output = Result<ListPostResponse, Error>> + Send;

    fn update_post(
        &self,
        req: &UpdatePostRequest,
    ) -> impl Future<Output = Result<Post, Error>> + Send;

    fn delete_post(
        &self,
        req: &DeletePostRequest,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    fn batch_delete_post(
        &self,
        req: &BatchDeletePostRequest,
    ) -> impl Future<Output = Result<(), Error>> + Send;
}

pub trait BlogRepository: Clone + Send + Sync + 'static {
    fn create_post(
        &self,
        req: &CreatePostRequest,
    ) -> impl Future<Output = Result<Post, Error>> + Send;

    fn list_post(
        &self,
        req: ListPostRequest,
    ) -> impl Future<Output = Result<ListPostResponse, Error>> + Send;

    fn update_post(
        &self,
        req: &UpdatePostRequest,
    ) -> impl Future<Output = Result<Post, Error>> + Send;

    fn delete_post(
        &self,
        req: &DeletePostRequest,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    fn batch_delete_post(
        &self,
        req: &BatchDeletePostRequest,
    ) -> impl Future<Output = Result<(), Error>> + Send;
}
