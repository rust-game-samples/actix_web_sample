use crate::constants::*;
use crate::error::ServiceError;
use crate::model::token::TokenClaims;
use actix_web::{http::header::HeaderValue, HttpRequest};
use jwt_simple::prelude::*;
use uuid::Uuid;

fn create_token_key() -> HS256Key {
    HS256Key::from_bytes(b"your_secret_key")
}

fn create_custom_claims(
    is_refresh: bool,
    uuid: String,
    duration: Duration,
) -> JWTClaims<TokenClaims> {
    Claims::with_custom_claims(
        TokenClaims {
            refresh: is_refresh,
        },
        duration,
    )
    .with_subject(uuid)
    .with_jwt_id(Uuid::new_v4().to_string())
}

pub fn get_token(req: HttpRequest) -> Result<String, ServiceError> {
    if let Some(auth_header) = req.headers().get(AUTHORIZATION) {
        if let Ok(auth_str) = auth_header.to_str() {
            if is_auth_header_valid(auth_header) {
                let token = auth_str[6..auth_str.len()].trim();
                return Ok(token.to_string());
            }
        }
    }
    Err(ServiceError::InternalServerError {
        error_message: MESSAGE_PROCESS_TOKEN_ERROR.to_string(),
    })
}

pub fn claims_verify_token(token: &str) -> Result<JWTClaims<TokenClaims>, ServiceError> {
    if let Ok(claims) = create_token_key().verify_token::<TokenClaims>(token, None) {
        Ok(claims)
    } else {
        Err(ServiceError::Unauthorized {
            error_message: MESSAGE_TOKEN_MISSING.to_string(),
        })
    }
}

fn claims_authenticate(claims: JWTClaims<TokenClaims>) -> Result<String, ServiceError> {
    if let Ok(token) = create_token_key().authenticate(claims) {
        Ok(token)
    } else {
        Err(ServiceError::InternalServerError {
            error_message: MESSAGE_PROCESS_TOKEN_ERROR.to_string(),
        })
    }
}

pub fn is_auth_header_valid(auth_header: &HeaderValue) -> bool {
    if let Ok(auth_str) = auth_header.to_str() {
        return auth_str.starts_with("bearer") || auth_str.starts_with("Bearer");
    }
    false
}

pub fn create_access_token(uuid: String) -> Result<String, ServiceError> {
    let claims = create_custom_claims(false, uuid, Duration::from_mins(15));
    claims_authenticate(claims)
}

pub fn create_refresh_token(uuid: String) -> Result<String, ServiceError> {
    let claims = create_custom_claims(true, uuid, Duration::from_hours(24));
    claims_authenticate(claims)
}

pub fn get_sub_uuid(
    claims: &JWTClaims<TokenClaims>,
    user_id: &str,
) -> Result<String, ServiceError> {
    let sub_uuid = claims.subject.clone().unwrap();
    if sub_uuid.clone() != user_id {
        return Err(ServiceError::BadRequest {
            error_message: MESSAGE_BAD_REQUEST.to_string(),
        });
    }
    Ok(sub_uuid)
}
