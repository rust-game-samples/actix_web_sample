use actix_web::{web, App, HttpServer};

mod api;
mod constants;
mod error;
mod model;
mod utils;

use api::google::{google_callback, google_login};
use utils::google_auth::get_client;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = get_client().await;

    HttpServer::new(move || {
        App::new().app_data(web::Data::new(client.clone())).service(
            web::scope("/auth")
                .service(google_login)
                .service(google_callback),
        )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
