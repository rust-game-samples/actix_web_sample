mod api;
mod constants;
mod error;
mod model;
mod repository;
mod utils;

use crate::repository::mdb::{MDBRepository, COLL_NAME};
use actix_web::{web, App, HttpServer};
// use api::token::{create_token, hello, refresh_token};
use api::user::{login_user, register_user};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mdb_repo: MDBRepository = MDBRepository::init(COLL_NAME.to_string()).await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(mdb_repo.clone()))
            // .service(
            //     web::scope("/token")
            //         .service(create_token)
            //         .service(refresh_token),
            // )
            // .service(web::scope("/user"))
            // .service(hello)
            .service(register_user)
            .service(login_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
