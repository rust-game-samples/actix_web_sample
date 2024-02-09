use crate::constants::*;
use crate::error::ServiceError;
use crate::model::{
    response::ResponseBody,
    todo::SubmitTodoRequest,
    token::CreateTokenResponse,
    user::{PutUserRequest, RegisterUser, SubmitUserRequest, User},
};
use crate::repository::todo::TodoRepository;
use crate::repository::user::UserRepository;
use crate::utils::token::{
    claims_verify_token, create_access_token, create_refresh_token, get_sub_uuid, get_token,
};
use actix_web::{
    delete, get, post, put, web::Data, web::Json, web::Path, HttpRequest, HttpResponse,
};

#[post("/")]
async fn post_todo(
    todo_repo: Data<TodoRepository>,
    request: Json<SubmitTodoRequest>,
) -> Result<HttpResponse, ServiceError> {
    let result = todo_repo.post_todo(request).await;

    match result {
        Ok(todo) => Ok(HttpResponse::Ok().json(ResponseBody::new(MESSAGE_OK, todo))),
        Err(err) => Err(ServiceError::InternalServerError {
            error_message: err.to_string(),
        }),
    }
}
