use super::{
    models::posts::{CreatePostError, CreatePostRequest, Post},
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

    async fn list_post(
        &self,
        req: super::models::posts::ListPostRequest,
    ) -> Result<super::models::posts::ListPostResponse, super::models::posts::ListPostError> {
        self.repo.list_post(req).await
    }
}
