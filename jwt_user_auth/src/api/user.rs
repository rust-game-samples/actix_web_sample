use crate::error::{ServiceError, UserError};
use crate::model::token::CreateTokenResponse;
use crate::model::user::{RegisterUser, User};
use crate::repository::mdb::MDBRepository;
use crate::utils::token::{create_access_token, create_refresh_token};
use actix_web::{
    delete,
    error::ResponseError,
    get,
    http::{header::ContentType, StatusCode},
    post, put,
    web::Data,
    web::Json,
    web::Path,
    HttpResponse,
};
use derive_more::Display;
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
            Ok(HttpResponse::Ok().json(CreateTokenResponse {
                token,
                refresh_token: refresh,
            }))
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

            Ok(HttpResponse::Ok().json(CreateTokenResponse {
                token,
                refresh_token: refresh,
            }))
        }
        Err(err) => Err(ServiceError::InternalServerError {
            error_message: err.to_string(),
        }),
    }
}

#[get("/{uuid}")]
async fn get_user(ddb_repo: Data<MDBRepository>, uuid: Path<String>) -> HttpResponse {
    let user_id = uuid.into_inner();
    let collection = ddb_repo.get_user(user_id.clone()).await;
    match collection {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().body(format!("No user found with userid")),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
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
