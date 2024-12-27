use sqlx::{Postgres, Transaction};
use uuid::Uuid;

use crate::domain::blog::models::users::User;

use super::postgres::Pg;

impl Pg {
    pub async fn save_user(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        username: &str,
        email: &Option<String>,
        phone: &Option<String>,
        password: &str,
    ) -> anyhow::Result<User> {
        let id = Uuid::new_v4();
        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (id, username, email, phone, password) VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#,
        )
        .bind(id.to_string())
        .bind(username.to_string())
        .bind(email)
        .bind(phone)
        .bind(password.to_string())
        .fetch_one(tx.as_mut())
        .await?;
        Ok(user)
    }

    pub async fn get_user_by_username(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        username: &str,
    ) -> anyhow::Result<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT * FROM users WHERE username = $1 LIMIT 1

            "#,
        )
        .bind(username)
        .fetch_optional(tx.as_mut())
        .await?;
        Ok(user)
    }

    pub async fn find_user_by_id(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        id: &str,
    ) -> anyhow::Result<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT * FROM users WHERE id = $1 LIMIT 1
            "#,
        )
        .bind(id)
        .fetch_optional(tx.as_mut())
        .await?;
        Ok(user)
    }

    pub async fn delete_user_by_id(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        id: &str,
    ) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            DELETE FROM users WHERE id = $1
            "#,
        )
        .bind(id.to_string())
        .execute(tx.as_mut())
        .await?;
        Ok(())
    }

    pub async fn remove_policy(&self, ptype: &str, params: Vec<String>) -> anyhow::Result<()> {
        self.enforcer.remove_policy(ptype, params).await?;
        Ok(())
    }
}
