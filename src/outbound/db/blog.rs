use std::vec;

use anyhow::Context;

use crate::{
    domain::blog::{
        error::Error,
        models::{
            posts::{
                BatchDeletePostRequest, CreatePostRequest, DeletePostRequest, ListPostRequest,
                ListPostResponse, Post, UpdatePostRequest,
            },
            users::{
                CreateUserRequest, DeleteUserRequest, GetUserByIdRequest, GetUserRequest,
                LoginRequest, User,
            },
        },
        ports::BlogRepository,
    },
    utils::{jwt, verify_password_hash},
};

use super::postgres::Pg;

impl BlogRepository for Pg {
    async fn create_post(&self, req: &CreatePostRequest) -> Result<Post, Error> {
        let mut tx = self
            .pool
            .begin()
            .await
            .context("failed t start transaction")?;
        let post = self
            .save_post(&mut tx, &req.title, &req.content, &req.username)
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
        let posts = self
            .list_post(&mut tx, req.offset, req.limit, &req.username)
            .await?;
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
            .update_post(&mut tx, &req.id, &req.title, &req.content, &req.username)
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
        self.delete_post_by_id_and_username(&mut tx, &req.id, &req.username)
            .await?;
        tx.commit().await.context("failed to commit")?;
        Ok(())
    }

    async fn batch_delete_post(&self, req: &BatchDeletePostRequest) -> Result<(), Error> {
        let mut tx = self
            .pool
            .begin()
            .await
            .context("failed t start transaction")?;

        self.delete_posts_by_ids(&mut tx, req.ids.clone(), &req.username)
            .await?;
        tx.commit().await.context("failed to commit")?;
        Ok(())
    }

    async fn create_user(&self, req: &CreateUserRequest) -> Result<User, Error> {
        let mut tx = self
            .pool
            .begin()
            .await
            .context("failed t start transaction")?;
        let res = self.get_user_by_username(&mut tx, &req.username).await?;
        if res.is_some() {
            return Err(Error::Custom("username already exists".to_string()));
        }
        let user = self
            .save_user(
                &mut tx,
                &req.username.clone(),
                &req.email,
                &req.phone,
                &req.password,
            )
            .await
            .context("failed to save user")?;
        self.enforcer
            .add_policy(
                "p",
                vec![
                    user.username.clone(),
                    format!("/api/users/{}", user.username),
                    "(GET)|(POST)|(PUT)|(DELETE)".to_string(),
                ],
            )
            .await?;
        tx.commit().await.context("failed to commit")?;
        Ok(user)
    }

    async fn get_user(&self, req: &GetUserRequest) -> Result<User, Error> {
        let mut tx = self
            .pool
            .begin()
            .await
            .context("failed t start transaction")?;
        let res = self.get_user_by_username(&mut tx, &req.username).await?;
        tx.commit().await.context("failed to commit")?;
        match res {
            Some(user) => Ok(user),
            None => Err(Error::Custom("user not found".to_string())),
        }
    }

    async fn get_user_by_id(&self, req: &GetUserByIdRequest) -> Result<User, Error> {
        let mut tx = self
            .pool
            .begin()
            .await
            .context("failed t start transaction")?;
        let res = self.find_user_by_id(&mut tx, &req.id).await?;
        tx.commit().await.context("failed to commit")?;
        match res {
            Some(user) => Ok(user),
            None => Err(Error::Custom("user not found".to_string())),
        }
    }

    async fn delete_user(&self, req: &DeleteUserRequest) -> Result<(), Error> {
        let mut tx = self
            .pool
            .begin()
            .await
            .context("failed t start transaction")?;
        let user = self.get_user_by_username(&mut tx, &req.username).await?;
        if let Some(user) = user {
            self.delete_user_by_id(&mut tx, &user.id).await?;
            self.remove_policy(
                "p",
                vec![
                    user.username,
                    format!("/api/users/{}", req.username),
                    "(GET)|(POST)|(PUT)|(DELETE)".to_string(),
                ],
            )
            .await?;
            tx.commit().await.context("failed to commit")?;
            Ok(())
        } else {
            Err(Error::Custom("user not found".to_string()))
        }
    }

    async fn login(&self, req: &LoginRequest) -> Result<String, Error> {
        let mut tx = self
            .pool
            .begin()
            .await
            .context("failed t start transaction")?;
        let user = self.get_user_by_username(&mut tx, &req.username).await?;
        if let Some(user) = user {
            verify_password_hash(user.password.clone(), req.password.clone())?;
            let token =
                jwt::JWT::new(&req.jwt_secret).generate_token(&req.expiration, user.id, None)?;
            Ok(token)
        } else {
            Err(Error::Custom("user not found".to_string()))
        }
    }

    async fn check_permission(&self, sub: &str, obj: &str, act: &str) -> Result<bool, Error> {
        let res = self.enforcer.check_permission(sub, obj, act).await?;
        Ok(res)
    }
}
