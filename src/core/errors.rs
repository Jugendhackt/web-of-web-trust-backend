// imports
use actix_web::{
    error::BlockingError, error::ResponseError, http::StatusCode, Error as WebError, HttpResponse,
    HttpResponseBuilder,
};
use serde::Serialize;
use sqlx::error::Error as SQLError;
use std::fmt::{self, Debug};

/*
APIError:
    displays user when something is not e.g. available as html for GET Requests
    returns template error.html

    Errors:
    ValidationError: Only returned for non-auth queries as everything else is API (POST) based
    InternalError: Something went really, really wrong but was gracefully caught
*/
#[derive(Debug, Clone)]
pub enum APIError {
    /// Used to mitigate thread panics on execution errors that weren't expected
    InternalError(String),
    /// Used when some validation is failing e.g. a too short or malformed string
    ValidationError([&'static str; 1], String),
    /// Error Container for Errors that were created by a function inside a actix::web::block closure
    BlockingError(String),
    /// Used to handle access violations when e.g. a token is expired or permissions are missing
    AuthError(String),
    /// Fallback when not finding a requested record. This may arise from some DB queries
    NotFoundError,
    /// Used when sqlx connection pool is exhausted and unable to provide further connection for requests
    /// When this happens to often either increase the pool size or check if any connection is not dropped properly (memory leak?)
    PoolError,
    /// Placeholder for an APIError when handling e.g. Results that require an Err(APIError) but don't actually care about the specific Error
    EmptyError,
    /// Integrity Error raised by Database
    IntegrityError,
}

#[derive(Serialize)]
pub struct APIErrorDetails {
    pub loc: Option<[&'static str; 1]>,
    pub msg: String,
    #[serde(rename(serialize = "type"))]
    pub error_type: String,
}

#[derive(Serialize)]
pub struct APIErrorWrapper {
    pub detail: APIErrorDetails,
}

impl fmt::Display for APIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.to_string().as_str())
    }
}

impl From<SQLError> for APIError {
    fn from(e: SQLError) -> Self {
        match e {
            SQLError::PoolClosed => Self::PoolError,
            SQLError::PoolTimedOut => Self::PoolError,
            SQLError::ColumnNotFound(msg) => Self::InternalError(msg),
            SQLError::RowNotFound => Self::NotFoundError,
            SQLError::Database(db) => match db.code() {
                Some(code) => {
                    // see error code table https://www.postgresql.org/docs/14/errcodes-appendix.html | Class 23
                    if code.starts_with("23") {
                        Self::IntegrityError
                    } else {
                        Self::InternalError(db.message().to_owned())
                    }
                }
                None => Self::InternalError(db.message().to_owned()),
            },
            e => Self::InternalError(e.to_string()),
        }
    }
}

impl ResponseError for APIError {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code()).json(self.to_wrapped())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            APIError::InternalError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            APIError::ValidationError { .. } => StatusCode::UNPROCESSABLE_ENTITY,
            APIError::PoolError => StatusCode::TOO_MANY_REQUESTS,
            APIError::AuthError { .. } => StatusCode::UNAUTHORIZED,
            APIError::NotFoundError { .. } => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl APIError {
    pub fn to_wrapped(&self) -> APIErrorWrapper {
        APIErrorWrapper {
            detail: APIErrorDetails {
                loc: match self {
                    Self::ValidationError(loc, _) => Some(*loc),
                    _ => None,
                },
                msg: self.to_string(),
                error_type: self.title().to_owned(),
            },
        }
    }

    pub fn title(&self) -> &'static str {
        match self {
            APIError::PoolError => "Internal Error",
            APIError::InternalError(_) => "Internal Error",
            APIError::ValidationError(_, _) => "Validation Error",
            APIError::BlockingError(_) => "Internal Error",
            APIError::AuthError(_) => "Security Error",
            APIError::NotFoundError => "Not Found",
            APIError::EmptyError | APIError::IntegrityError => "Internal Error",
        }
    }
}

// convert WebErrors to APIErrors
impl From<WebError> for APIError {
    fn from(error: WebError) -> Self {
        APIError::InternalError(error.to_string())
    }
}

// Convert Thread Blocking Errors to APIErrors
impl From<BlockingError> for APIError {
    fn from(error: BlockingError) -> Self {
        APIError::BlockingError(format!("Error {} occurred", error.status_code()))
    }
}
// String casting for APIErrors
impl From<APIError> for String {
    fn from(error: APIError) -> String {
        match error {
            APIError::NotFoundError { .. } => "Didn't found the requested resource".to_owned(),
            APIError::InternalError(message) => message.to_owned(),
            APIError::ValidationError(_loc, message) => message.to_owned(),
            APIError::BlockingError(message) => message.to_owned(),
            APIError::AuthError(message) => message.to_owned(),
            APIError::PoolError => "Exhausted Pool connections".to_owned(),
            APIError::EmptyError => "Empty Error".to_owned(),
            APIError::IntegrityError => "Internal Error".to_owned(),
        }
    }
}

impl From<&APIError> for String {
    fn from(error: &APIError) -> String {
        match error {
            APIError::EmptyError => "Empty Error".to_owned(),
            APIError::NotFoundError => "Didn't found the requested resource".to_owned(),
            APIError::InternalError(message) => message.clone().to_owned(),
            APIError::ValidationError(_loc, message) => message.clone().to_owned(),
            APIError::BlockingError(message) => message.clone().to_owned(),
            APIError::AuthError(message) => message.clone().to_owned(),
            APIError::PoolError => "Exhausted Pool connections".to_owned(),
            APIError::IntegrityError => "Internal Error".to_owned(),
        }
    }
}
