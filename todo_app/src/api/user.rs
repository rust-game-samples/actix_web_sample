use crate::constants::*;
use crate::error::ServiceError;
use crate::model::{
    response::ResponseBody,
    token::CreateTokenResponse,
    user::{PutUserRequest, RegisterUser, SubmitUserRequest, User},
};
use crate::repository::user::UserRepository;
use crate::utils::token::{create_access_token, create_refresh_token, get_request_sub_uuid};
use actix_web::{
    delete, get, post, put, web::Data, web::Json, web::Path, HttpRequest, HttpResponse,
};

#[post("/register")]
async fn register_user(
    ddb_repo: Data<UserRepository>,
    request: Json<SubmitUserRequest>,
) -> Result<HttpResponse, ServiceError> {
    let new_user = RegisterUser::new(request.email.clone(), request.password.clone());
    let result = ddb_repo.post_user(new_user.clone()).await;
    let uuid = new_user.get_uuid();

    match result {
        Ok(_) => {
            let token = create_access_token(uuid.clone())?;
            let refresh = create_refresh_token(uuid.clone())?;

            Ok(HttpResponse::Ok().json(ResponseBody::new(
                MESSAGE_SIGNUP_SUCCESS,
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

#[post("/login")]
async fn login_user(
    ddb_repo: Data<UserRepository>,
    request: Json<SubmitUserRequest>,
) -> Result<HttpResponse, ServiceError> {
    let result = ddb_repo.login_user(&request.email, &request.password).await;

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

#[get("/")]
async fn get_user(
    ddb_repo: Data<UserRepository>,
    request: HttpRequest,
) -> Result<HttpResponse, ServiceError> {
    let sub_uuid = get_request_sub_uuid(request)?;
    let result = ddb_repo.get_user(sub_uuid.clone()).await;
    match result {
        Ok(user) => Ok(HttpResponse::Ok().json(ResponseBody::new(MESSAGE_OK, user))),
        Err(err) => Err(err),
    }
}

#[put("/")]
async fn update_user(
    ddb_repo: Data<UserRepository>,
    request: HttpRequest,
    put_user: Json<PutUserRequest>,
) -> Result<HttpResponse, ServiceError> {
    let sub_uuid = get_request_sub_uuid(request)?;
    let new_user = User::from_put(sub_uuid.clone(), put_user);
    let result = ddb_repo.put_user(sub_uuid.clone(), new_user).await;

    match result {
        Ok(user) => Ok(HttpResponse::Ok().json(ResponseBody::new(MESSAGE_OK, user))),
        Err(err) => Err(err),
    }
}

#[delete("/")]
pub async fn delete_user(
    ddb_repo: Data<UserRepository>,
    request: HttpRequest,
) -> Result<HttpResponse, ServiceError> {
    let sub_uuid = get_request_sub_uuid(request)?;

    let result = ddb_repo.delete_user(sub_uuid.clone()).await;
    match result {
        Ok(uuid) => Ok(HttpResponse::Ok().json(ResponseBody::new(MESSAGE_OK, uuid))),
        Err(err) => Err(err),
    }
}
