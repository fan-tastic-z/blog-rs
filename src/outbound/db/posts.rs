use anyhow::Context;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

use crate::domain::blog::{
    error::Error,
    models::posts::{
        BatchDeletePostRequest, CreatePostRequest, DeletePostRequest, ListPostRequest,
        ListPostResponse, Post, UpdatePostRequest,
    },
    ports::BlogRepository,
};

use super::postgres::Pg;

impl Pg {
    pub async fn save_post(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        title: &str,
        content: &str,
    ) -> anyhow::Result<Post> {
        let id = Uuid::new_v4();
        let post = sqlx::query_as::<_, Post>(
            r#"
            INSERT INTO posts (id, title, content) VALUES ($1, $2, $3)
            RETURNING *
            "#,
        )
        .bind(id.to_string())
        .bind(title.to_string())
        .bind(content.to_string())
        .fetch_one(tx.as_mut())
        .await?;
        Ok(post)
    }

    pub async fn get_post(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        id: &Uuid,
    ) -> anyhow::Result<Post> {
        let post = sqlx::query_as::<_, Post>(
            r#"
            SELECT id, title, content, created_at, updated_at FROM posts WHERE id = $1
            "#,
        )
        .bind(id.to_string())
        .fetch_one(tx.as_mut())
        .await?;
        Ok(post)
    }

    pub async fn post_count(&self, tx: &mut Transaction<'_, Postgres>) -> anyhow::Result<u64> {
        let count: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(id) FROM posts
            "#,
        )
        .fetch_one(tx.as_mut())
        .await?;
        Ok(count.0 as u64)
    }

    pub async fn list_post(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        offset: u32,
        limit: u32,
    ) -> anyhow::Result<Vec<Post>> {
        let res = sqlx::query_as::<_, Post>(
            r#"
            SELECT
                id, title, content, created_at, updated_at
            FROM
                posts
            ORDER BY created_at DESC OFFSET $1 LIMIT $2
            "#,
        )
        .bind(offset as i64)
        .bind(limit as i64)
        .fetch_all(tx.as_mut())
        .await?;
        Ok(res)
    }

    pub async fn update_post(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        id: &str,
        title: &str,
        content: &str,
    ) -> anyhow::Result<Post> {
        let post = sqlx::query_as::<_, Post>(
            r#"
            UPDATE posts SET title = $2, content = $3, updated_at = NOW() WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id.to_string())
        .bind(title.to_string())
        .bind(content.to_string())
        .fetch_one(tx.as_mut())
        .await?;
        Ok(post)
    }

    pub async fn delete_by_id(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        id: &str,
    ) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            DELETE FROM posts WHERE id = $1
            "#,
        )
        .bind(id.to_string())
        .execute(tx.as_mut())
        .await?;
        Ok(())
    }

    pub async fn delete_by_ids(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        ids: Vec<String>,
    ) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            DELETE FROM posts WHERE id = ANY($1)
            "#,
        )
        .bind(ids)
        .execute(tx.as_mut())
        .await?;
        Ok(())
    }
}

impl BlogRepository for Pg {
    async fn create_post(&self, req: &CreatePostRequest) -> Result<Post, Error> {
        let mut tx = self
            .pool
            .begin()
            .await
            .context("failed t start transaction")?;
        let post = self
            .save_post(&mut tx, &req.title, &req.content)
            .await
            .context("failed to save post")?;
        tx.commit().await.context("failed to commit")?;
        Ok(post)
    }

    async fn list_post(&self, req: ListPostRequest) -> Result<ListPostResponse, Error> {
        let mut tx = self
            .pool
            .begin()
            .await
            .context("failed t start transaction")?;
        let posts = self.list_post(&mut tx, req.offset, req.limit).await?;
        let total = self.post_count(&mut tx).await?;
        tx.commit().await.context("failed to commit")?;
        Ok(ListPostResponse { total, posts })
    }

    async fn update_post(&self, req: &UpdatePostRequest) -> Result<Post, Error> {
        let mut tx = self
            .pool
            .begin()
            .await
            .context("failed t start transaction")?;
        let post = self
            .update_post(&mut tx, &req.id, &req.title, &req.content)
            .await
            .context("failed to update post")?;
        tx.commit().await.context("failed to commit")?;
        Ok(post)
    }

    async fn delete_post(&self, req: &DeletePostRequest) -> Result<(), Error> {
        let mut tx = self
            .pool
            .begin()
            .await
            .context("failed t start transaction")?;
        self.delete_by_id(&mut tx, &req.id).await?;
        tx.commit().await.context("failed to commit")?;
        Ok(())
    }

    async fn batch_delete_post(&self, req: &BatchDeletePostRequest) -> Result<(), Error> {
        let mut tx = self
            .pool
            .begin()
            .await
            .context("failed t start transaction")?;

        self.delete_by_ids(&mut tx, req.ids.clone()).await?;
        tx.commit().await.context("failed to commit")?;
        Ok(())
    }
}
