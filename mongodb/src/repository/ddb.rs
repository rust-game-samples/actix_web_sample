use mongodb::{Client, IndexModel};
use mongodb::bson::doc;
use mongodb::options::IndexOptions;
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
}