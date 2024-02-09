use crate::constants::*;
use crate::error::ServiceError;
use crate::model::todo::{Todo, TodoUpdate};
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
    delete, get, patch, post, put, web::Data, web::Json, web::Path, HttpRequest, HttpResponse,
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

#[get("/")]
async fn get_todos(
    todo_repo: Data<TodoRepository>,
    request: Json<SubmitTodoRequest>,
) -> Result<HttpResponse, ServiceError> {
    let page = 1;
    let page_size = 30;
    let result = todo_repo.get_todos(page, page_size).await;

    match result {
        Ok(todos) => Ok(HttpResponse::Ok().json(ResponseBody::new(MESSAGE_OK, todos))),
        Err(err) => Err(ServiceError::InternalServerError {
            error_message: err.to_string(),
        }),
    }
}

#[get("/{uuid}")]
async fn get_todo(
    todo_repo: Data<TodoRepository>,
    uuid: Path<String>,
) -> Result<HttpResponse, ServiceError> {
    let result = todo_repo.get_todo(uuid.into_inner()).await;

    match result {
        Ok(todo) => Ok(HttpResponse::Ok().json(ResponseBody::new(MESSAGE_OK, todo))),
        Err(err) => Err(ServiceError::InternalServerError {
            error_message: err.to_string(),
        }),
    }
}

#[put("/{uuid}")]
async fn put_todo(
    todo_repo: Data<TodoRepository>,
    uuid: Path<String>,
    request: Json<Todo>,
) -> Result<HttpResponse, ServiceError> {
    let todo_id = uuid.into_inner();
    let todo = request.into_inner();
    let result = todo_repo.put_todo(todo_id, todo).await;

    match result {
        Ok(todo) => Ok(HttpResponse::Ok().json(ResponseBody::new(MESSAGE_OK, todo))),
        Err(err) => Err(ServiceError::InternalServerError {
            error_message: err.to_string(),
        }),
    }
}

#[delete("/{id}")]
async fn delete_todo(
    todo_repo: Data<TodoRepository>,
    uuid: Path<String>,
) -> Result<HttpResponse, ServiceError> {
    let todo_id = uuid.into_inner();
    let result = todo_repo.delete_todo(todo_id).await;

    match result {
        Ok(todo) => Ok(HttpResponse::Ok().json(ResponseBody::new(MESSAGE_OK, todo))),
        Err(err) => Err(ServiceError::InternalServerError {
            error_message: err.to_string(),
        }),
    }
}

#[patch("/{id}/state")]
async fn patch_todo_state(
    todo_repo: Data<TodoRepository>,
    uuid: Path<String>,
    request: Json<TodoUpdate>,
) -> Result<HttpResponse, ServiceError> {
    let todo_id = uuid.into_inner();
    let todo_update = request.into_inner();
    let result = todo_repo.patch_todo(todo_id, todo_update).await;

    match result {
        Ok(todo) => Ok(HttpResponse::Ok().json(ResponseBody::new(MESSAGE_OK, todo))),
        Err(err) => Err(ServiceError::InternalServerError {
            error_message: err.to_string(),
        }),
    }
}
