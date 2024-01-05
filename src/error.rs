use std::fmt;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub type CustomResult<T> = std::result::Result<T, CustomError>;

pub enum CustomError {
    MongoDbError(mongodb::error::Error),
    RedisError {
        message: String,
    },
    NotFound {
        message: String,
    },
    SerdeError(serde_json::Error),
    TemplateError(askama::Error),
    InvalidAuthorizationHeader(http_auth_basic::AuthBasicError),
    UserNotFound {
        message: String,
    },
    UserUnauthorized {
        message: String,
    },
    HashError(argon2::password_hash::Error),
    TooManyRequests {
        actual_count: u64,
        permission_count: u64,
    },
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CustomError::RedisError { message }
                | CustomError::NotFound { message }
                | CustomError::UserNotFound { message }
                | CustomError::UserUnauthorized { message } => format!("{}", message),
                CustomError::MongoDbError(err) => err.to_string(),
                CustomError::SerdeError(err) => err.to_string(),
                CustomError::TemplateError(err) => err.to_string(),
                CustomError::InvalidAuthorizationHeader(err) => err.to_string(),
                CustomError::HashError(err) => err.to_string(),
                CustomError::TooManyRequests {
                    actual_count,
                    permission_count,
                } => format!(
                    "Too many requests: actual_count = {}, permission_count: {}",
                    actual_count, permission_count
                ),
            }
        )
    }
}

impl IntoResponse for CustomError {
    fn into_response(self) -> Response {
        log::error!("{}", self.to_string());

        let response = match self {
            CustomError::InvalidAuthorizationHeader(_)
            | CustomError::UserNotFound { .. }
            | CustomError::HashError(_)
            | CustomError::UserUnauthorized { .. } => {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .header(
                        "WWW-Authenticate",
                        "Basic realm=\"Please enter your credentials\"",
                    )
                    .body(axum::body::Body::from("Unauthorized"))
                    .unwrap()
            }
            CustomError::NotFound { message } => (StatusCode::NOT_FOUND, message),
            CustomError::TooManyRequests { .. } => (
                StatusCode::TOO_MANY_REQUESTS,
                String::from("Too many requests"),
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Something went wrong"),
            ),
        };

        response.into_response()
    }
}

impl From<mongodb::error::Error> for CustomError {
    fn from(err: mongodb::error::Error) -> Self {
        Self::MongoDbError(err)
    }
}

impl From<redis::RedisError> for CustomError {
    fn from(err: redis::RedisError) -> Self {
        Self::RedisError {
            message: err.to_string(),
        }
    }
}

impl From<mongodb::bson::oid::Error> for CustomError {
    fn from(err: mongodb::bson::oid::Error) -> Self {
        Self::NotFound {
            message: err.to_string(),
        }
    }
}

impl From<serde_json::Error> for CustomError {
    fn from(err: serde_json::Error) -> Self {
        Self::SerdeError(err)
    }
}

impl From<askama::Error> for CustomError {
    fn from(err: askama::Error) -> Self {
        Self::TemplateError(err)
    }
}

impl From<http_auth_basic::AuthBasicError> for CustomError {
    fn from(err: http_auth_basic::AuthBasicError) -> Self {
        Self::InvalidAuthorizationHeader(err)
    }
}

impl From<argon2::password_hash::Error> for CustomError {
    fn from(err: argon2::password_hash::Error) -> Self {
        Self::HashError(err)
    }
}
