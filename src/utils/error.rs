use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("compute password hash error")]
    PasswordHashError(#[from] argon2::password_hash::Error),
}
