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
        username: &str,
    ) -> anyhow::Result<Post> {
        let id = Uuid::new_v4();
        let post = sqlx::query_as::<_, Post>(
            r#"
            INSERT INTO posts (id, title, content, username) VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
        )
        .bind(id.to_string())
        .bind(title.to_string())
        .bind(content.to_string())
        .bind(username.to_string())
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
        username: &str,
    ) -> anyhow::Result<Vec<Post>> {
        let res = sqlx::query_as::<_, Post>(
            r#"
            SELECT
                id, title, content, created_at, updated_at
            FROM
                posts
            where
                username = $1
            ORDER BY created_at DESC OFFSET $2 LIMIT $3
            "#,
        )
        .bind(username.to_string())
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
        username: &str,
    ) -> anyhow::Result<Post> {
        let post = sqlx::query_as::<_, Post>(
            r#"
            UPDATE posts SET title = $1, content = $2, updated_at = NOW() WHERE id = $3 AND username = $4
            RETURNING *
            "#,
        )
        .bind(title.to_string())
        .bind(content.to_string())
        .bind(id.to_string())
        .bind(username.to_string())
        .fetch_one(tx.as_mut())
        .await?;
        Ok(post)
    }

    pub async fn delete_post_by_id_and_username(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        id: &str,
        username: &str,
    ) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            DELETE FROM posts WHERE id = $1 AND username = $2
            "#,
        )
        .bind(id.to_string())
        .bind(username.to_string())
        .execute(tx.as_mut())
        .await?;
        Ok(())
    }

    pub async fn delete_posts_by_ids(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        ids: Vec<String>,
        username: &str,
    ) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            DELETE FROM posts WHERE id = ANY($1) AND username = $2
            "#,
        )
        .bind(ids)
        .bind(username.to_string())
        .execute(tx.as_mut())
        .await?;
        Ok(())
    }
}
