use crate::error::{ServiceError, UserError};
use crate::model::user::{RegisterUser, User};
use bcrypt::verify;
use mongodb::bson::doc;
use mongodb::bson::extjson::de::Error;
use mongodb::options::IndexOptions;
use mongodb::{
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Client, Collection, IndexModel,
};

pub const DB_NAME: &str = "userJWT";
pub const COLL_NAME: &str = "users";

pub struct MDBError;

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

    pub async fn post_user(&self, user: RegisterUser) -> Result<InsertOneResult, Error> {
        let user = self
            .col
            .insert_one(user, None)
            .await
            .expect("Error creating user");
        Ok(user)
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
                        error_message: "Invalid credentials".to_string(),
                    })
                }
            }
            Ok(None) => Err(ServiceError::Unauthorized {
                error_message: "Invalid credentials".to_string(),
            }),
            Err(_) => Err(ServiceError::InternalServerError {
                error_message: "".to_string(),
            }),
        }
    }

    pub async fn get_user(&self, uuid: String) -> mongodb::error::Result<Option<User>> {
        let collection: Collection<User> =
            self.client.database(DB_NAME).collection(&self.table_name);
        collection.find_one(doc! { "uuid": &uuid }, None).await
    }

    pub async fn put_user(&self, uuid: String, user: User) -> mongodb::error::Result<UpdateResult> {
        let collection: Collection<User> =
            self.client.database(DB_NAME).collection(&self.table_name);
        let filter = doc! {"uuid": uuid};
        let new_doc = doc! {
            "$set":
                {
                    "first_name": user.first_name,
                    "last_name": user.last_name,
                    "username": user.username,
                    // "email": user.email
                },
        };
        collection.update_one(filter, new_doc, None).await
    }

    pub async fn delete_user(&self, uuid: String) -> mongodb::error::Result<DeleteResult> {
        let collection: Collection<User> =
            self.client.database(DB_NAME).collection(&self.table_name);
        let filter = doc! {"uuid": uuid};
        collection.delete_one(filter, None).await
    }
}
