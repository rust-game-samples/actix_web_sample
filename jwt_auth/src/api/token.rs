use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use uuid::Uuid;
use jwt_simple::prelude::*;
use crate::constants::*;
use crate::model::token::{CreateTokenRequest, TokenClaims, CreateTokenResponse, CreateRefreshTokenResponse};
use crate::utils::token::{get_token, claims_verify_token};
use crate::error::ServiceError;

#[post("/")]
async fn create_token(data: web::Json<CreateTokenRequest>) -> impl Responder {
    if !(data.username == "daisuke" && data.password == "1234") {
        return HttpResponse::BadRequest().body("user does not exist or password is wrong");
    }
    let token_key = HS256Key::from_bytes(b"secret");

    let claims =
        Claims::with_custom_claims(TokenClaims { refresh: false }, Duration::from_mins(15))
            .with_subject(1)
            .with_jwt_id(Uuid::new_v4().to_string());
    let access = token_key.authenticate(claims).unwrap();

    let claims =
        Claims::with_custom_claims(TokenClaims { refresh: true }, Duration::from_hours(24))
            .with_subject(1)
            .with_jwt_id(Uuid::new_v4().to_string());
    let refresh = token_key.authenticate(claims).unwrap();

    HttpResponse::Ok().json(CreateTokenResponse {
        token: access,
        refresh_token: refresh,
    })
}

#[post("/refresh")]
async fn refresh_token(req: HttpRequest) -> Result<HttpResponse, ServiceError> {
    let token = get_token(req)?;
    let claims = claims_verify_token(&token)?;
    let token_key = HS256Key::from_bytes(b"secret");

    if !claims.custom.refresh {
        return Err(ServiceError::BadRequest {
            error_message: MESSAGE_REFRESH_TOKEN_ERROR.to_string(),
        });
    }

    let claims =
        Claims::with_custom_claims(TokenClaims { refresh: false }, Duration::from_mins(15))
            .with_subject(1)
            .with_jwt_id(Uuid::new_v4().to_string());
    let token = token_key.authenticate(claims).unwrap();
    Ok(HttpResponse::Ok().json(CreateRefreshTokenResponse { token: token }))
}

#[get("/hello")]
async fn hello(req: HttpRequest) -> Result<HttpResponse, ServiceError> {
    let token = get_token(req)?;
    let claims = claims_verify_token(&token)?;

    if claims.custom.refresh {
        return Err(ServiceError::InternalServerError {
            error_message: MESSAGE_REFRESH_TOKEN_ERROR.to_string(),
        });
    }
    Ok(HttpResponse::Ok().body("Authorized Access Success! Hello World!"))
}
