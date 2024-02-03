mod api;
mod model;
mod utils;

use actix_web::{web, App, HttpServer};
use api::token::{create_token, refresh_token, hello};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/token")
                    .service(create_token)
                    .service(refresh_token),
            )
            .service(web::scope("/user"))
            .service(hello)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}