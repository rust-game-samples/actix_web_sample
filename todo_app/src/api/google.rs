use crate::constants::*;
use crate::error::ServiceError;
use crate::model::response::ResponseBody;
use crate::model::token::CreateTokenResponse;
use crate::repository::user::UserRepository;
use crate::utils::google_auth::{get_code, get_token_response};
use crate::utils::token::{create_access_token, create_refresh_token};
use actix_web::web::Data;
use actix_web::{get, HttpRequest, HttpResponse, Responder};
use openidconnect::reqwest::async_http_client;
use openidconnect::{
    core::{CoreAuthenticationFlow, CoreClient, CoreGenderClaim},
    CsrfToken, EmptyAdditionalClaims, Nonce, OAuth2TokenResponse, Scope, UserInfoClaims,
};
use url::Url;

#[get("/google/login")]
pub async fn google_login(client: Data<CoreClient>) -> impl Responder {
    let (auth_url, _csrf_state, _nonce) = client
        .authorize_url(
            CoreAuthenticationFlow::AuthorizationCode,
            || CsrfToken::new_random(),
            || Nonce::new_random(),
        )
        .add_scope(Scope::new("openid".to_string()))
        .add_scope(Scope::new("email".to_string()))
        .url();

    HttpResponse::Found()
        .append_header(("Location", auth_url.to_string()))
        .finish()
}

#[get("/google/callback")]
pub async fn google_callback(
    client: Data<CoreClient>,
    ddb_repo: Data<UserRepository>,
    req: HttpRequest,
) -> Result<HttpResponse, ServiceError> {
    let query: String = req.query_string().to_string();
    let url = Url::parse(&(format!("{}?", BASE_URL).to_string() + &query)).unwrap();
    let code = get_code(url)?;
    let token_response = get_token_response(code).await?;

    let user_info_request = client
        .user_info(token_response.access_token().clone(), None)
        .map_err(|_| ServiceError::Unauthorized {
            error_message: MESSAGE_USER_INFORMATION_REQUEST_ERROR.to_string(),
        })
        .unwrap();

    let user_info: UserInfoClaims<EmptyAdditionalClaims, CoreGenderClaim> = user_info_request
        .request_async(async_http_client)
        .await
        .map_err(|_| ServiceError::InternalServerError {
            error_message: MESSAGE_CAN_NOT_FETCH_DATA.to_string(),
        })
        .unwrap();

    let email = user_info.email().map(|email| email.to_string());
    let result = ddb_repo
        .find_or_create_user(email.unwrap_or_default())
        .await;

    match result {
        Ok(user) => {
            let token = create_access_token(user.get_uuid())?;
            let refresh = create_refresh_token(user.get_uuid())?;
            Ok(HttpResponse::Ok().json(ResponseBody::new(
                MESSAGE_LOGIN_SUCCESS,
                CreateTokenResponse {
                    token,
                    refresh_token: refresh,
                },
            )))
        }
        Err(err) => Err(ServiceError::InternalServerError {
            error_message: err.to_string(),
        }),
    }
}
