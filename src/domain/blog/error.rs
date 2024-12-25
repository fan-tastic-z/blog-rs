use thiserror::Error;

use crate::inbound::http::response::ApiError;

#[derive(Debug, Error)]
pub enum Error {
    #[error("request validate error")]
    ValidationError(#[from] validator::ValidationErrors),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

impl From<Error> for ApiError {
    fn from(e: Error) -> Self {
        match e {
            Error::ValidationError(err) => ApiError::UnprocessableEntity(err.to_string()),
            Error::Unknown(err) => {
                tracing::error!("{:?}\n{}", err, err.backtrace());
                ApiError::InternalServerError(err.to_string())
            }
        }
    }
}
