use anyhow::Context;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

use crate::domain::blog::{
    models::posts::{CreatePostError, CreatePostRequest, Post},
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
            SELECT id, title, content, create_at FROM posts WHERE id = $1
            "#,
        )
        .bind(id.to_string())
        .fetch_one(tx.as_mut())
        .await?;
        Ok(post)
    }
}

impl BlogRepository for Pg {
    async fn create_post(&self, req: &CreatePostRequest) -> Result<Post, CreatePostError> {
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
}
