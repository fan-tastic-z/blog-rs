use std::future::Future;

use super::{
    error::Error,
    models::{
        posts::{
            BatchDeletePostRequest, CreatePostRequest, DeletePostRequest, ListPostRequest,
            ListPostResponse, Post, UpdatePostRequest,
        },
        users::{
            CreateUserRequest, DeleteUserRequest, GetUserByIdRequest, GetUserRequest, LoginRequest,
            User,
        },
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

    fn create_user(
        &self,
        req: &CreateUserRequest,
    ) -> impl Future<Output = Result<User, Error>> + Send;

    fn get_user_by_id(
        &self,
        req: &GetUserByIdRequest,
    ) -> impl Future<Output = Result<User, Error>> + Send;

    fn delete_user(
        &self,
        req: &DeleteUserRequest,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    fn get_user(&self, req: &GetUserRequest) -> impl Future<Output = Result<User, Error>> + Send;

    fn login(&self, req: &LoginRequest) -> impl Future<Output = Result<String, Error>> + Send;

    fn check_permission(
        &self,
        sub: &str,
        obj: &str,
        act: &str,
    ) -> impl Future<Output = Result<bool, Error>> + Send;
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

    fn create_user(
        &self,
        req: &CreateUserRequest,
    ) -> impl Future<Output = Result<User, Error>> + Send;

    fn get_user_by_id(
        &self,
        req: &GetUserByIdRequest,
    ) -> impl Future<Output = Result<User, Error>> + Send;

    fn delete_user(
        &self,
        req: &DeleteUserRequest,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    fn get_user(&self, req: &GetUserRequest) -> impl Future<Output = Result<User, Error>> + Send;

    fn login(&self, req: &LoginRequest) -> impl Future<Output = Result<String, Error>> + Send;

    fn check_permission(
        &self,
        sub: &str,
        obj: &str,
        act: &str,
    ) -> impl Future<Output = Result<bool, Error>> + Send;
}
