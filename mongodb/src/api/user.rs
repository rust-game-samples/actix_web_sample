use actix_web::{get, HttpResponse, post, web::Path, web::Json, web::Data};
use crate::model::user::User;
use crate::repository::ddb::{DDBRepository};

#[post("/user")]
async fn add_user(ddb_repo: Data<DDBRepository>, request: Json<User>) -> HttpResponse {
    let result = ddb_repo.post_user(request).await;
    match result {
        Ok(_) => HttpResponse::Ok().body("user added"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
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