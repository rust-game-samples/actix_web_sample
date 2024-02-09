mod api;
mod constants;
mod error;
mod model;
mod repository;
mod utils;

use crate::api::todo::post_todo;
use crate::constants::*;
use crate::repository::{todo::TodoRepository, user::UserRepository};
use actix_web::{web, App, HttpServer};
use api::token::refresh_token;
use api::user::{delete_user, get_user, login_user, register_user, update_user};
use mongodb::Client;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());
    let client = Client::with_uri_str(uri).await.expect("failed to connect");
    let user_repo = UserRepository::new(&client, DB_NAME).await;
    let todo_repo = TodoRepository::new(&client, DB_NAME).await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(user_repo.clone()))
            .app_data(web::Data::new(todo_repo.clone()))
            .service(register_user)
            .service(login_user)
            .service(web::scope("/token").service(refresh_token))
            .service(
                web::scope("/user")
                    .service(get_user)
                    .service(update_user)
                    .service(delete_user),
            )
            .service(web::scope("/todos").service(post_todo))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
