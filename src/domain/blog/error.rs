use thiserror::Error;

use crate::inbound::http::response::ApiError;

#[derive(Debug, Error)]
pub enum Error {
    #[error("request validate error")]
    ValidationError(#[from] validator::ValidationErrors),
    #[error("{0}")]
    Custom(String),
    #[error(transparent)]
    UtilsError(#[from] crate::utils::error::Error),
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
            Error::UtilsError(err) => {
                tracing::error!("{:?}", err);
                ApiError::InternalServerError(err.to_string())
            }
            Error::Custom(err) => ApiError::BadRequestError(err),
        }
    }
}
