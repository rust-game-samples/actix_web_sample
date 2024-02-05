use crate::constants::*;
use crate::error::{ServiceError, UserError};
use crate::model::response::ResponseBody;
use crate::model::token::CreateTokenResponse;
use crate::model::user::{RegisterUser, User};
use crate::repository::mdb::MDBRepository;
use crate::utils::token::{
    claims_verify_token, create_access_token, create_refresh_token, get_token,
};
use actix_web::{
    delete, get, post, put, web::Data, web::Json, web::Path, HttpRequest, HttpResponse,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UserIdentifier {
    uuid: String,
}

#[derive(Deserialize)]
pub struct SubmitUserRequest {
    email: String,
    password: String,
}

#[derive(Deserialize)]
pub struct PutUserRequest {
    first_name: String,
    last_name: String,
    username: String,
}

#[post("/register")]
async fn register_user(
    ddb_repo: Data<MDBRepository>,
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
    ddb_repo: Data<MDBRepository>,
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

#[get("/{uuid}")]
async fn get_user(
    ddb_repo: Data<MDBRepository>,
    uuid: Path<String>,
    request: HttpRequest,
) -> Result<HttpResponse, ServiceError> {
    let user_id = uuid.into_inner();

    let token = get_token(request)?;
    let claims = claims_verify_token(&token)?;

    if claims.custom.refresh {
        return Err(ServiceError::BadRequest {
            error_message: MESSAGE_REFRESH_TOKEN_ERROR.to_string(),
        });
    }

    let sub_uuid = claims.subject.unwrap();
    if sub_uuid.clone() != user_id {
        return Err(ServiceError::BadRequest {
            error_message: MESSAGE_BAD_REQUEST.to_string(),
        });
    }

    let result = ddb_repo.get_user(sub_uuid.clone()).await;
    match result {
        Ok(user) => Ok(HttpResponse::Ok().json(ResponseBody::new(MESSAGE_OK, user))),
        Err(err) => Err(err),
    }
}

#[put("/{uuid}")]
async fn update_user(
    ddb_repo: Data<MDBRepository>,
    uuid: Path<String>,
    request: Json<PutUserRequest>,
) -> Result<Json<UserIdentifier>, UserError> {
    let user_id = uuid.into_inner();
    let user = User::from_put(
        user_id.clone(),
        request.first_name.clone(),
        request.last_name.clone(),
        request.username.clone(),
    );
    match ddb_repo.put_user(user_id.clone(), user).await {
        Ok(_) => Ok(Json(UserIdentifier {
            uuid: user_id.clone(),
        })),
        Err(_) => Err(UserError::UserUpdateFailure),
    }
}

#[delete("/{id}")]
pub async fn delete_user(
    ddb_repo: Data<MDBRepository>,
    uuid: Path<String>,
) -> Result<String, UserError> {
    let user_id = uuid.into_inner();
    match ddb_repo.delete_user(user_id.clone()).await {
        Ok(_) => Ok("deleted".to_string()),
        Err(_) => Err(UserError::UserUpdateFailure),
    }
}
