use super::{
    models::posts::{
        BatchDeletePostError, BatchDeletePostRequest, CreatePostError, CreatePostRequest,
        DeletePostError, DeletePostRequest, ListPostError, ListPostRequest, ListPostResponse, Post,
        UpdatePostRequest,
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
    async fn create_post(&self, req: &CreatePostRequest) -> Result<Post, CreatePostError> {
        self.repo.create_post(req).await
    }

    async fn list_post(&self, req: ListPostRequest) -> Result<ListPostResponse, ListPostError> {
        self.repo.list_post(req).await
    }

    async fn update_post(&self, req: &UpdatePostRequest) -> Result<Post, CreatePostError> {
        self.repo.update_post(req).await
    }
    async fn delete_post(&self, req: &DeletePostRequest) -> Result<(), DeletePostError> {
        self.repo.delete_post(req).await
    }

    async fn batch_delete_post(
        &self,
        req: &BatchDeletePostRequest,
    ) -> Result<(), BatchDeletePostError> {
        self.repo.batch_delete_post(req).await
    }
}
