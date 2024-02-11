use crate::constants::*;
use crate::error::ServiceError;
use crate::model::{
    response::ResponseBody,
    todo::{SubmitTodoRequest, Todo, TodoPagination, TodoPaginationRequest, TodoUpdate},
    token::CreateTokenResponse,
    user::{PutUserRequest, RegisterUser, SubmitUserRequest, User},
};
use crate::repository::todo::TodoRepository;
use crate::repository::user::UserRepository;
use crate::utils::token::{
    claims_verify_token, create_access_token, create_refresh_token, get_request_sub_uuid,
    get_sub_uuid, get_token,
};
use actix_web::{
    delete, get, patch, post, put, web::Data, web::Json, web::Path, HttpRequest, HttpResponse,
};

#[post("/")]
async fn post_todo(
    todo_repo: Data<TodoRepository>,
    http_request: HttpRequest,
    request: Json<SubmitTodoRequest>,
) -> Result<HttpResponse, ServiceError> {
    let sub_user_id = get_request_sub_uuid(http_request)?;
    let result = todo_repo.post_todo(request, sub_user_id).await;

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
    http_request: HttpRequest,
    todo_pagination_request: Json<TodoPaginationRequest>,
) -> Result<HttpResponse, ServiceError> {
    let sub_user_id = get_request_sub_uuid(http_request)?;
    let todo_pagination = TodoPagination::new(todo_pagination_request);
    let result = todo_repo.get_todos(todo_pagination, sub_user_id).await;

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
    path_todo_id: Path<String>,
    http_request: HttpRequest,
) -> Result<HttpResponse, ServiceError> {
    let sub_user_id = get_request_sub_uuid(http_request)?;
    let result = todo_repo
        .get_todo(path_todo_id.into_inner(), sub_user_id)
        .await;

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
    path_todo_id: Path<String>,
    request: Json<Todo>,
    http_request: HttpRequest,
) -> Result<HttpResponse, ServiceError> {
    let sub_user_id = get_request_sub_uuid(http_request)?;
    let todo_id = path_todo_id.into_inner();
    let todo = request.into_inner();
    let result = todo_repo.put_todo(todo_id, sub_user_id, todo).await;

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
    path_todo_id: Path<String>,
    http_request: HttpRequest,
) -> Result<HttpResponse, ServiceError> {
    let sub_user_id = get_request_sub_uuid(http_request)?;
    let todo_id = path_todo_id.into_inner();
    let result = todo_repo.delete_todo(todo_id, sub_user_id).await;

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
    http_request: HttpRequest,
) -> Result<HttpResponse, ServiceError> {
    let sub_user_id = get_request_sub_uuid(http_request)?;
    let todo_id = uuid.into_inner();
    let todo_update = request.into_inner();
    let result = todo_repo
        .patch_todo(todo_id, sub_user_id, todo_update)
        .await;

    match result {
        Ok(todo) => Ok(HttpResponse::Ok().json(ResponseBody::new(MESSAGE_OK, todo))),
        Err(err) => Err(ServiceError::InternalServerError {
            error_message: err.to_string(),
        }),
    }
}
