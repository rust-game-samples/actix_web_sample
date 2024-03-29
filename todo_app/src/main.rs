mod api;
mod constants;
mod error;
mod middleware;
mod model;
mod repository;
mod utils;

use actix_web::{web, App, HttpServer};
use api::{google::*, todo::*, token::*, user::*};
use constants::*;
use middleware::auth_middleware::AuthMiddleware;
use mongodb::Client;
use repository::{todo::TodoRepository, user::UserRepository};
use utils::google_auth::get_client;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());
    let client = Client::with_uri_str(uri).await.expect("failed to connect");
    let google_client = get_client().await;

    let user_repo = UserRepository::new(&client, DB_NAME).await;
    let todo_repo = TodoRepository::new(&client, DB_NAME).await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(user_repo.clone()))
            .app_data(web::Data::new(todo_repo.clone()))
            .app_data(web::Data::new(google_client.clone()))
            .service(register_user)
            .service(login_user)
            .service(web::scope("/token").service(refresh_token))
            .service(
                web::scope("/auth")
                    .service(google_login)
                    .service(google_callback),
            )
            .service(
                web::scope("/user")
                    .wrap(AuthMiddleware)
                    .service(get_user)
                    .service(update_user)
                    .service(delete_user),
            )
            .service(
                web::scope("/todos")
                    .wrap(AuthMiddleware)
                    .service(post_todo)
                    .service(get_todos)
                    .service(get_todo)
                    .service(put_todo)
                    .service(patch_todo_state)
                    .service(delete_todo),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
