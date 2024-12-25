use super::{
    error::Error,
    models::{
        posts::{
            BatchDeletePostRequest, CreatePostRequest, DeletePostRequest, ListPostRequest,
            ListPostResponse, Post, UpdatePostRequest,
        },
        users::{CreateUserRequest, User},
    },
    ports::{BlogRepository, BlogService},
};

#[derive(Debug, Clone)]
pub struct Service<R>
where
    R: BlogRepository,
{
    repo: R,
}

impl<R> Service<R>
where
    R: BlogRepository,
{
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

impl<R> BlogService for Service<R>
where
    R: BlogRepository,
{
    async fn create_post(&self, req: &CreatePostRequest) -> Result<Post, Error> {
        self.repo.create_post(req).await
    }

    async fn list_post(&self, req: ListPostRequest) -> Result<ListPostResponse, Error> {
        self.repo.list_post(req).await
    }

    async fn update_post(&self, req: &UpdatePostRequest) -> Result<Post, Error> {
        self.repo.update_post(req).await
    }
    async fn delete_post(&self, req: &DeletePostRequest) -> Result<(), Error> {
        self.repo.delete_post(req).await
    }

    async fn batch_delete_post(&self, req: &BatchDeletePostRequest) -> Result<(), Error> {
        self.repo.batch_delete_post(req).await
    }

    async fn create_user(&self, req: &CreateUserRequest) -> Result<User, Error> {
        self.repo.create_user(req).await
    }
}
