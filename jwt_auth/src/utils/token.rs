use actix_web::{HttpRequest, http::header::HeaderValue};
use jwt_simple::prelude::*;
use crate::constants::*;
use crate::error::ServiceError;
use crate::model::token::TokenClaims;

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
    let token_key = HS256Key::from_bytes(b"secret");
    if let Ok(claims) = token_key.verify_token::<TokenClaims>(&token, None) {
        Ok(claims)
    } else {
        Err(ServiceError::Unauthorized {
            error_message: MESSAGE_TOKEN_MISSING.to_string(),
        })
    }
}

pub fn is_auth_header_valid(auth_header: &HeaderValue) -> bool {
    if let Ok(auth_str) = auth_header.to_str() {
        return auth_str.starts_with("bearer") || auth_str.starts_with("Bearer");
    }
    false
}