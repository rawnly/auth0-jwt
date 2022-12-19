#[cfg(feature = "axum")]
use axum::{extract::rejection::TypedHeaderRejection, response::IntoResponse, Json};

use reqwest::StatusCode;
use serde::Serialize;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to fetch JWKS")]
    JWKSFetchError,

    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("Failed to decode token kid")]
    TokenKidDecodeError,

    #[error("{0}")]
    ValidationError(alcoholic_jwt::ValidationError),

    #[error("Failed to deserialize json: {0}")]
    DeserializationError(serde_json::Error),

    #[cfg(feature = "axum")]
    #[error("Invalid authorization header")]
    InvalidTokenHeader(TypedHeaderRejection),
}

impl Error {
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::DeserializationError(_)
            | Self::TokenKidDecodeError
            | Self::ValidationError(_)
            | Self::JWKSFetchError => StatusCode::FORBIDDEN,
            _ => StatusCode::UNAUTHORIZED,
        }
    }

    pub fn title(&self) -> &'static str {
        match self {
            Self::DeserializationError(_) => "Deserialization Error",
            Self::TokenKidDecodeError => "Invalid token kid",
            Self::ValidationError(_) => "Invalid Token",
            Self::JWKSFetchError => "JWKS fetch error",
            _ => "Internal Server Error",
        }
    }
}

#[cfg(feature = "axum")]
impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        (self.status_code(), Json(ErrorBody::from_error(self))).into_response()
    }
}

#[derive(Debug, Serialize)]
pub struct ErrorBody {
    pub title: String,
    pub message: String,
    pub status_code: u16,
}

impl ErrorBody {
    pub fn from_error(error: Error) -> Self {
        Self {
            title: error.title().to_string(),
            message: error.to_string(),
            status_code: error.status_code().as_u16(),
        }
    }
}
