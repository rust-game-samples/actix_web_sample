use actix_web::{web};
use mongodb::{Client, Collection, IndexModel};
use mongodb::bson::doc;
use mongodb::options::IndexOptions;
use mongodb::results::InsertOneResult;
use crate::model::user::User;

pub const DB_NAME: &str = "myApp";
pub const COLL_NAME: &str = "users";

async fn create_username_index(client: &Client) {
    let options = IndexOptions::builder().unique(true).build();
    let model = IndexModel::builder()
        .keys(doc! { "username": 1 })
        .options(options)
        .build();
    client
        .database(DB_NAME)
        .collection::<User>(COLL_NAME)
        .create_index(model, None)
        .await
        .expect("creating an index should succeed");
}

#[derive(Clone)]
pub struct DDBRepository {
    pub client: Client,
    pub table_name: String
}

impl DDBRepository {
    pub async fn init(table_name: String) -> DDBRepository {
        let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());
        let client = Client::with_uri_str(uri).await.expect("failed to connect");
        create_username_index(&client).await;

        DDBRepository {
            table_name,
            client
        }
    }

    pub async fn post_user(&self, user: web::Json<User>) -> mongodb::error::Result<InsertOneResult> {
        let collection = self.client.database(DB_NAME).collection(&self.table_name);
        // let result = collection.insert_one(user.into_inner(), None).await;
        // todo: add uuid

        collection.insert_one(user.into_inner(), None).await
        // match request.send().await {
        //     Ok(_) => Ok(()),
        //     Err(_) => Err(DDBError)
        // }
    }

    pub async fn get_user(&self, username: String) -> mongodb::error::Result<Option<User>> {
        let collection: Collection<User> = self.client.database(DB_NAME).collection(&self.table_name);
        collection.find_one(doc! { "username": &username }, None).await
    }
}