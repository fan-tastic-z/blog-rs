use sqlx::{Postgres, Transaction};
use uuid::Uuid;

use crate::domain::blog::models::posts::Post;

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
