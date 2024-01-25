use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct User {
    pub uuid: String,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub email: String,
}

impl User {
    pub fn new(first_name: String, last_name: String, username: String, email: String) -> User {
        User {
            uuid: Uuid::new_v4().to_string(),
            first_name,
            last_name,
            username,
            email,
        }
    }

    pub fn get_uuid(&self) -> String {
        format!("{}", self.uuid)
    }

    pub fn from_id(
        uuid: String,
        first_name: String,
        last_name: String,
        username: String,
        email: String,
    ) -> User {
        User {
            uuid,
            first_name,
            last_name,
            username,
            email,
        }
    }

    pub fn from_put(uuid: String, first_name: String, last_name: String, username: String) -> User {
        User {
            uuid,
            first_name,
            last_name,
            username,
            email: "".to_string(),
        }
    }
}
