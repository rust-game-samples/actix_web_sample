use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateTokenRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct CreateTokenResponse {
    pub token: String,
    pub refresh_token: String,
}

#[derive(Serialize)]
pub struct CreateRefreshTokenResponse {
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct TokenClaims {
    pub refresh: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseBody<T> {
    pub message: String,
    pub data: T,
}

impl<T> ResponseBody<T> {
    pub fn new(message: &str, data: T) -> ResponseBody<T> {
        ResponseBody {
            message: message.to_string(),
            data,
        }
    }
}