use super::errors::APIError;
use actix_web::HttpResponse;

// types
pub type APIResponse = Result<HttpResponse, APIError>;
