mod model;
mod api;
mod repository;

#[cfg(test)]
mod test;

use actix_web::{web, App, HttpServer};
use api::user::{get_user, add_user};
use crate::repository::ddb::{DDBRepository, COLL_NAME };

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let ddb_repo: DDBRepository = DDBRepository::init(COLL_NAME.to_string()).await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(ddb_repo.client.clone()))
            .service(add_user)
            .service(get_user)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}