use actix_web::web::Json;
use bcrypt::{hash, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct SubmitUserRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct PutUserRequest {
    pub first_name: String,
    pub last_name: String,
    pub username: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct RegisterUser {
    pub uuid: String,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub email: String,
    pub password: String,
}
impl RegisterUser {
    pub fn new(email: String, password: String) -> RegisterUser {
        RegisterUser {
            uuid: Uuid::new_v4().to_string(),
            first_name: "".to_string(),
            last_name: "".to_string(),
            username: "".to_string(),
            email,
            password: hash(password.clone(), DEFAULT_COST).unwrap(),
        }
    }

    pub fn get_uuid(&self) -> String {
        format!("{}", self.uuid)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct User {
    pub uuid: String,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub email: String,
}

impl User {
    pub fn get_uuid(&self) -> String {
        format!("{}", self.uuid)
    }

    pub fn from_register_data(register_data: RegisterUser) -> User {
        User {
            uuid: register_data.uuid.clone(),
            first_name: register_data.first_name.clone(),
            last_name: register_data.last_name.clone(),
            username: register_data.username.clone(),
            email: register_data.email.clone(),
        }
    }

    pub fn from_put(uuid: String, put_user: Json<PutUserRequest>) -> User {
        User {
            uuid,
            first_name: put_user.first_name.clone(),
            last_name: put_user.last_name.clone(),
            username: put_user.username.clone(),
            email: "".to_string(),
        }
    }
}
