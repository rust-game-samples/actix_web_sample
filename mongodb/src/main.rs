mod api;
mod model;
mod repository;

#[cfg(test)]
mod test;

use crate::repository::mdb::{MDBRepository, COLL_NAME};
use actix_web::{web, App, HttpServer};
use api::user::{add_user, delete_user, get_user, update_user};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mdb_repo: MDBRepository = MDBRepository::init(COLL_NAME.to_string()).await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(mdb_repo.clone()))
            .service(add_user)
            .service(get_user)
            .service(update_user)
            .service(delete_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
