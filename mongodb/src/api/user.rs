use crate::model::user::User;
use crate::repository::ddb::DDBRepository;
use actix_web::{
    error::ResponseError,
    get,
    http::{header::ContentType, StatusCode},
    post,
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
    first_name: String,
    last_name: String,
    username: String,
    email: String,
}

#[derive(Debug, Display)]
pub enum UserError {
    UserNotFound,
    UserUpdateFailure,
    UserCreationFailure,
    BadUserRequest,
}
impl ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            UserError::UserNotFound => StatusCode::NOT_FOUND,
            UserError::UserUpdateFailure => StatusCode::FAILED_DEPENDENCY,
            UserError::UserCreationFailure => StatusCode::FAILED_DEPENDENCY,
            UserError::BadUserRequest => StatusCode::BAD_REQUEST,
        }
    }
}

#[post("/user")]
async fn add_user(
    ddb_repo: Data<DDBRepository>,
    request: Json<SubmitUserRequest>,
) -> Result<Json<UserIdentifier>, UserError> {
    let user = User::new(
        request.first_name.clone(),
        request.last_name.clone(),
        request.username.clone(),
        request.email.clone(),
    );
    let user_uuid = user.get_uuid();
    let result = ddb_repo.post_user(user).await;
    match result {
        Ok(_) => Ok(Json(UserIdentifier { uuid: user_uuid })),
        Err(_) => Err(UserError::UserCreationFailure),
    }
}

#[get("/user/{username}")]
async fn get_user(ddb_repo: Data<DDBRepository>, username: Path<String>) -> HttpResponse {
    let username = username.into_inner();
    let collection = ddb_repo.get_user(username.clone()).await;
    match collection {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => {
            HttpResponse::NotFound().body(format!("No user found with username {username}"))
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
