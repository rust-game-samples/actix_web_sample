use crate::constants::*;
use crate::error::ServiceError;
use crate::model::token::CreateRefreshTokenResponse;
use crate::utils::token::{claims_verify_token, create_access_token, get_token};
use actix_web::{post, HttpRequest, HttpResponse};

#[post("/refresh")]
async fn refresh_token(req: HttpRequest) -> Result<HttpResponse, ServiceError> {
    let request_token = get_token(req)?;
    let claims = claims_verify_token(&request_token)?;
    if !claims.custom.refresh {
        return Err(ServiceError::BadRequest {
            error_message: MESSAGE_REFRESH_TOKEN_ERROR.to_string(),
        });
    }

    let sub_uuid = claims.subject.unwrap();
    let token = create_access_token(sub_uuid.clone())?;

    Ok(HttpResponse::Ok().json(CreateRefreshTokenResponse { token }))
}
