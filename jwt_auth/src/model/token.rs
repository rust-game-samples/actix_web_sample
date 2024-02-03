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