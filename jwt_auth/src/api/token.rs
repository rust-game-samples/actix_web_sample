use actix_web::{get, post, web, HttpRequest, HttpResponse};
use crate::constants::*;
use crate::model::token::{CreateTokenRequest, CreateTokenResponse, CreateRefreshTokenResponse};
use crate::utils::token::{get_token, claims_verify_token, create_access_token, create_refresh_token};
use crate::error::ServiceError;

#[post("/")]
async fn create_token(data: web::Json<CreateTokenRequest>) -> Result<HttpResponse, ServiceError> {
    if !(data.username == "daisuke" && data.password == "1234") {
        return Err(ServiceError::BadRequest {
            error_message: "user does not exist or password is wrong".to_string(),
        });
    }
    let token = create_access_token()?;
    let refresh = create_refresh_token()?;

    Ok(HttpResponse::Ok().json(CreateTokenResponse {
        token,
        refresh_token: refresh,
    }))
}

#[post("/refresh")]
async fn refresh_token(req: HttpRequest) -> Result<HttpResponse, ServiceError> {
    let request_token = get_token(req)?;
    let claims = claims_verify_token(&request_token)?;
    if !claims.custom.refresh {
        return Err(ServiceError::BadRequest {
            error_message: MESSAGE_REFRESH_TOKEN_ERROR.to_string(),
        });
    }
    let token = create_access_token()?;

    Ok(HttpResponse::Ok().json(CreateRefreshTokenResponse { token }))
}

#[get("/hello")]
async fn hello(req: HttpRequest) -> Result<HttpResponse, ServiceError> {
    let token = get_token(req)?;
    let claims = claims_verify_token(&token)?;
    if claims.custom.refresh {
        return Err(ServiceError::BadRequest {
            error_message: MESSAGE_REFRESH_TOKEN_ERROR.to_string(),
        });
    }
    Ok(HttpResponse::Ok().body("Authorized Access Success! Hello World!"))
}
