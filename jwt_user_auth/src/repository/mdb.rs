use crate::constants::*;
use crate::error::ServiceError;
use crate::model::user::{RegisterUser, User};
use bcrypt::verify;
use mongodb::bson::doc;
use mongodb::options::IndexOptions;
use mongodb::{results::InsertOneResult, Client, Collection, IndexModel};

pub const DB_NAME: &str = "userJWT";
pub const COLL_NAME: &str = "users";

async fn create_username_index(client: &Client) {
    let options = IndexOptions::builder().unique(true).build();
    let model = IndexModel::builder()
        .keys(doc! { "email": 1 })
        .options(options)
        .build();
    client
        .database(DB_NAME)
        .collection::<RegisterUser>(COLL_NAME)
        .create_index(model, None)
        .await
        .expect("creating an index should succeed");
}

#[derive(Clone)]
pub struct MDBRepository {
    pub client: Client,
    pub table_name: String,
    col: Collection<RegisterUser>,
}

impl MDBRepository {
    pub async fn init(table_name: String) -> MDBRepository {
        let uri =
            std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());
        let client = Client::with_uri_str(uri).await.expect("failed to connect");
        create_username_index(&client).await;
        let col: Collection<RegisterUser> =
            client.database(DB_NAME).collection(&table_name.clone());

        MDBRepository {
            table_name,
            client,
            col,
        }
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
        let collection: Collection<RegisterUser> =
            self.client.database(DB_NAME).collection(&self.table_name);

        match collection.find_one(doc! {"email": email}, None).await {
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
        let collection: Collection<User> =
            self.client.database(DB_NAME).collection(&self.table_name);

        match collection.find_one(doc! { "uuid": &uuid }, None).await {
            Ok(Some(user)) => Ok(user),
            Ok(None) => Err(ServiceError::BadRequest {
                error_message: MESSAGE_BAD_REQUEST.to_string(),
            }),
            Err(_) => Err(ServiceError::InternalServerError {
                error_message: MESSAGE_INTERNAL_SERVER_ERROR.to_string(),
            }),
        }
    }

    pub async fn put_user(&self, uuid: String, user: User) -> Result<User, ServiceError> {
        let collection: Collection<User> =
            self.client.database(DB_NAME).collection(&self.table_name);
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

        match collection
            .find_one_and_update(filter, update, options)
            .await
        {
            Ok(Some(updated_user)) => Ok(updated_user),
            Ok(None) => Err(ServiceError::UpdateFailure {
                error_message: MESSAGE_CAN_NOT_UPDATE_DATA.to_string(),
            }),
            Err(_) => Err(ServiceError::InternalServerError {
                error_message: MESSAGE_INTERNAL_SERVER_ERROR.to_string(),
            }),
        }
    }

    pub async fn delete_user(&self, uuid: String) -> Result<String, ServiceError> {
        let collection: Collection<User> =
            self.client.database(DB_NAME).collection(&self.table_name);
        let filter = doc! {"uuid": &uuid};

        match collection.delete_one(filter, None).await {
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
}
