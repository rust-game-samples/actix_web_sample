use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct User {
    id: u32,
    first_name: String,
    last_name: String,
    username: String,
    email: String,
}

fn load_users() -> HashMap<u32, User> {
    let mut file = File::open("data/user_list.json").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Something went wrong reading the file");
    let users: Vec<User> = serde_json::from_str(&contents).unwrap();
    users.into_iter().map(|user| (user.id, user)).collect()
}

use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn get_user(id: web::Path<u32>, data: web::Data<HashMap<u32, User>>) -> impl Responder {
    if let Some(user) = data.get(&id.into_inner()) {
        HttpResponse::Ok().json(user)
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let users = web::Data::new(load_users());
    HttpServer::new(move || {
        App::new()
            .app_data(users.clone())
            .route("/user/{id}", web::get().to(get_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
