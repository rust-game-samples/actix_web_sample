use crate::constants::*;
use crate::error::*;
use openidconnect::reqwest::async_http_client;
use openidconnect::{
    core::{CoreClient, CoreProviderMetadata, CoreTokenResponse},
    AuthorizationCode, ClientId, ClientSecret, IssuerUrl, RedirectUrl,
};
use std::env;
use url::Url;

pub async fn get_client() -> CoreClient {
    let client_id =
        ClientId::new(env::var(GOOGLE_CLIENT_ID_ENV).expect(MESSAGE_GOOGLE_CLIENT_ID_MISSING));
    let client_secret = ClientSecret::new(
        env::var(GOOGLE_CLIENT_SECRET_ENV).expect(MESSAGE_GOOGLE_CLIENT_SECRET_MISSING),
    );
    let issuer_url = IssuerUrl::new("https://accounts.google.com".to_string()).unwrap();
    let redirect_url =
        RedirectUrl::new(format!("{}/auth/google/callback", BASE_URL).to_string()).unwrap();
    let provider_metadata = CoreProviderMetadata::discover_async(issuer_url, async_http_client)
        .await
        .unwrap();

    CoreClient::from_provider_metadata(provider_metadata, client_id, Some(client_secret))
        .set_redirect_uri(redirect_url)
}

pub fn get_code(url: Url) -> Result<AuthorizationCode, ServiceError> {
    let code_pair = url.query_pairs().find(|(key, _)| key == "code");

    if let Some(code_pair) = code_pair {
        Ok(AuthorizationCode::new(code_pair.1.to_string()))
    } else {
        Err(ServiceError::BadRequest {
            error_message: MESSAGE_INTERNAL_SERVER_ERROR.to_string(),
        })
    }
}

pub async fn get_token_response(
    code: AuthorizationCode,
) -> Result<CoreTokenResponse, ServiceError> {
    let client = get_client().await;
    let result = client
        .exchange_code(code)
        .request_async(async_http_client)
        .await;

    match result {
        Ok(core_token_response) => Ok(core_token_response),
        Err(_) => Err(ServiceError::Unauthorized {
            error_message: MESSAGE_INVALID_TOKEN.to_string(),
        }),
    }
}
