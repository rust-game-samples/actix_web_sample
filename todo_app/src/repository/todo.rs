use crate::constants::*;
use crate::error::ServiceError;
use crate::model::todo::{SubmitTodoRequest, Todo};
use actix_web::web::Json;
use mongodb::{bson::doc, options::IndexOptions, Client, Collection, IndexModel};

async fn create_todo_index(client: &Client) {
    let options = IndexOptions::builder().unique(true).build();
    let model = IndexModel::builder()
        .keys(doc! { "uuid": 1 })
        .options(options)
        .build();
    client
        .database(DB_NAME)
        .collection::<Todo>(COLL_NAME_TODO)
        .create_index(model, None)
        .await
        .expect("creating an index should succeed");
}

#[derive(Clone)]
pub struct TodoRepository {
    col: Collection<Todo>,
}
impl TodoRepository {
    pub async fn new(client: &Client, db_name: &str) -> TodoRepository {
        let col = client.database(db_name).collection(COLL_NAME_TODO);
        create_todo_index(&client).await;
        TodoRepository { col }
    }

    pub async fn post_todo(&self, request: Json<SubmitTodoRequest>) -> Result<Todo, ServiceError> {
        let new_todo = Todo::new(request);
        let result = self.col.insert_one(new_todo, None).await;

        match result {
            Ok(insert_result) => {
                let object_id = insert_result.inserted_id.as_object_id().ok_or_else(|| {
                    ServiceError::CreationFailure {
                        error_message: MESSAGE_CAN_NOT_INSERT_DATA.to_string(),
                    }
                })?;

                let filter = doc! {"_id": object_id};
                match self.col.find_one(filter, None).await {
                    Ok(Some(todo)) => Ok(todo),
                    Ok(None) => Err(ServiceError::NotFound {
                        error_message: MESSAGE_CAN_NOT_INSERT_DATA.to_string(),
                    }),
                    Err(_) => Err(ServiceError::CreationFailure {
                        error_message: MESSAGE_CAN_NOT_INSERT_DATA.to_string(),
                    }),
                }
            }
            Err(_) => Err(ServiceError::CreationFailure {
                error_message: MESSAGE_CAN_NOT_INSERT_DATA.to_string(),
            }),
        }
    }
}
