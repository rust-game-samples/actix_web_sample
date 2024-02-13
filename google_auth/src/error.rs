use crate::model::response::ResponseBody;
use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum ServiceError {
    #[display(fmt = "{error_message}")]
    Unauthorized { error_message: String },

    #[display(fmt = "{error_message}")]
    InternalServerError { error_message: String },

    #[display(fmt = "{error_message}")]
    BadRequest { error_message: String },
    // #[display(fmt = "{error_message}")]
    // NotFound { error_message: String },
    //
    // #[display(fmt = "{error_message}")]
    // UpdateFailure { error_message: String },
    //
    // #[display(fmt = "{error_message}")]
    // CreationFailure { error_message: String },
}

impl ResponseError for ServiceError {
    fn status_code(&self) -> StatusCode {
        match *self {
            ServiceError::Unauthorized { .. } => StatusCode::UNAUTHORIZED,
            ServiceError::InternalServerError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::BadRequest { .. } => StatusCode::BAD_REQUEST,
            // ServiceError::UpdateFailure { .. } => StatusCode::FAILED_DEPENDENCY,
            // ServiceError::CreationFailure { .. } => StatusCode::FAILED_DEPENDENCY,
            // ServiceError::NotFound { .. } => StatusCode::NOT_FOUND,
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(ResponseBody::new(&self.to_string(), String::from("")))
    }
}
