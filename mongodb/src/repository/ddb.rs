use crate::model::user::User;
use mongodb::bson::doc;
use mongodb::options::IndexOptions;
use mongodb::results::{InsertOneResult, UpdateResult};
use mongodb::{Client, Collection, IndexModel};

pub const DB_NAME: &str = "myApp";
pub const COLL_NAME: &str = "users";

pub struct DDBError;

async fn create_username_index(client: &Client) {
    let options = IndexOptions::builder().unique(true).build();
    let model = IndexModel::builder()
        .keys(doc! { "email": 1 })
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
    pub table_name: String,
}

impl DDBRepository {
    pub async fn init(table_name: String) -> DDBRepository {
        let uri =
            std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());
        let client = Client::with_uri_str(uri).await.expect("failed to connect");
        create_username_index(&client).await;

        DDBRepository { table_name, client }
    }

    pub async fn post_user(&self, user: User) -> mongodb::error::Result<InsertOneResult> {
        let collection = self.client.database(DB_NAME).collection(&self.table_name);
        collection.insert_one(user, None).await
        // match request.send().await {
        //     Ok(_) => Ok(()),
        //     Err(_) => Err(DDBError)
        // }
    }

    pub async fn get_user(&self, uuid: String) -> mongodb::error::Result<Option<User>> {
        let collection: Collection<User> =
            self.client.database(DB_NAME).collection(&self.table_name);
        collection.find_one(doc! { "uuid": &uuid }, None).await
    }

    pub async fn put_user(&self, uuid: String, user: User) -> mongodb::error::Result<UpdateResult> {
        let collection: Collection<User> =
            self.client.database(DB_NAME).collection(&self.table_name);
        let filter = doc! {"uuid": uuid.clone()};
        let new_doc = doc! {
            "$set":
                {
                    // "uuid": uuid,
                    "first_name": user.first_name,
                    "last_name": user.last_name,
                    "username": user.username,
                    // "email": user.email
                },
        };
        collection.update_one(filter, new_doc, None).await
    }
}
