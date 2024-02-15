use crate::constants::*;
use crate::error::ServiceError;
use crate::model::user::{RegisterUser, User};
use bcrypt::verify;
use mongodb::{
    bson::doc, options::IndexOptions, results::InsertOneResult, Client, Collection, IndexModel,
};

async fn create_username_index(client: &Client) {
    let options = IndexOptions::builder().unique(true).build();
    let model = IndexModel::builder()
        .keys(doc! { "email": 1 })
        .options(options)
        .build();
    client
        .database(DB_NAME)
        .collection::<RegisterUser>(COLL_NAME_USERS)
        .create_index(model, None)
        .await
        .expect("creating an index should succeed");
}

#[derive(Clone)]
pub struct UserRepository {
    col: Collection<RegisterUser>,
}
impl UserRepository {
    pub async fn new(client: &Client, db_name: &str) -> UserRepository {
        let col = client.database(db_name).collection(COLL_NAME_USERS);
        create_username_index(&client).await;
        UserRepository { col }
    }

    pub async fn post_user(&self, user: RegisterUser) -> Result<InsertOneResult, ServiceError> {
        let result = self.col.insert_one(user, None).await;

        match result {
            Ok(user_result) => Ok(user_result),
            Err(_) => Err(ServiceError::CreationFailure {
                error_message: MESSAGE_SIGNUP_FAILED.to_string(),
            }),
        }
    }

    pub async fn login_user(&self, email: &str, password: &str) -> Result<User, ServiceError> {
        match self.col.find_one(doc! {"email": email}, None).await {
            Ok(Some(user_data)) => {
                if verify(password, &user_data.password).unwrap() {
                    Ok(User::from_register_data(user_data))
                } else {
                    Err(ServiceError::Unauthorized {
                        error_message: MESSAGE_LOGIN_FAILED.to_string(),
                    })
                }
            }
            Ok(None) => Err(ServiceError::Unauthorized {
                error_message: MESSAGE_LOGIN_FAILED.to_string(),
            }),
            Err(_) => Err(ServiceError::InternalServerError {
                error_message: "".to_string(),
            }),
        }
    }

    pub async fn get_user(&self, uuid: String) -> Result<User, ServiceError> {
        match self.col.find_one(doc! { "uuid": &uuid }, None).await {
            Ok(Some(user)) => Ok(User {
                uuid: user.uuid.clone(),
                first_name: user.first_name.clone(),
                last_name: user.last_name.clone(),
                username: user.username.clone(),
                email: user.email.clone(),
            }),
            Ok(None) => Err(ServiceError::BadRequest {
                error_message: MESSAGE_BAD_REQUEST.to_string(),
            }),
            Err(_) => Err(ServiceError::InternalServerError {
                error_message: MESSAGE_INTERNAL_SERVER_ERROR.to_string(),
            }),
        }
    }

    pub async fn put_user(&self, uuid: String, user: User) -> Result<User, ServiceError> {
        let filter = doc! {"uuid": uuid};
        let update = doc! {
            "$set":
                {
                    "first_name": user.first_name,
                    "last_name": user.last_name,
                    "username": user.username,
                    // "email": user.email
                },
        };

        let options = mongodb::options::FindOneAndUpdateOptions::builder()
            .return_document(mongodb::options::ReturnDocument::After)
            .build();

        match self.col.find_one_and_update(filter, update, options).await {
            Ok(Some(updated_user)) => Ok(User {
                uuid: updated_user.uuid.clone(),
                first_name: updated_user.first_name.clone(),
                last_name: updated_user.last_name.clone(),
                username: updated_user.username.clone(),
                email: updated_user.email.clone(),
            }),
            Ok(None) => Err(ServiceError::UpdateFailure {
                error_message: MESSAGE_CAN_NOT_UPDATE_DATA.to_string(),
            }),
            Err(_) => Err(ServiceError::InternalServerError {
                error_message: MESSAGE_INTERNAL_SERVER_ERROR.to_string(),
            }),
        }
    }

    pub async fn delete_user(&self, uuid: String) -> Result<String, ServiceError> {
        let filter = doc! {"uuid": &uuid};
        match self.col.delete_one(filter, None).await {
            Ok(delete_result) => {
                if delete_result.deleted_count == 0 {
                    Err(ServiceError::NotFound {
                        error_message: "User not found".to_string(),
                    })
                } else {
                    Ok(uuid)
                }
            }
            Err(_) => Err(ServiceError::InternalServerError {
                error_message: MESSAGE_INTERNAL_SERVER_ERROR.to_string(),
            }),
        }
    }

    pub async fn find_or_create_user(&self, email: String) -> Result<User, ServiceError> {
        match self.col.find_one(doc! { "email": &email }, None).await {
            Ok(Some(user_data)) => Ok(User::from_register_data(user_data)),
            Ok(None) => {
                let new_user = RegisterUser::new(email.clone(), "".to_string());
                let result = self.col.insert_one(new_user, None).await;
                match result {
                    Ok(insert_one_result) => {
                        let inserted_id = insert_one_result.inserted_id;
                        match self.col.find_one(doc! { "_id": inserted_id }, None).await {
                            Ok(Some(created_user_data)) => {
                                Ok(User::from_register_data(created_user_data))
                            }
                            _ => Err(ServiceError::CreationFailure {
                                error_message: MESSAGE_SIGNUP_FAILED.to_string(),
                            }),
                        }
                    }
                    Err(_) => Err(ServiceError::CreationFailure {
                        error_message: MESSAGE_SIGNUP_FAILED.to_string(),
                    }),
                }
            }
            Err(_) => Err(ServiceError::InternalServerError {
                error_message: MESSAGE_INTERNAL_SERVER_ERROR.to_string(),
            }),
        }
    }
}
