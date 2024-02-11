use crate::constants::*;
use crate::error::ServiceError;
use crate::model::todo::{SubmitTodoRequest, Todo, TodoPagination, TodoUpdate};
use actix_web::web::Json;
use futures_util::stream::TryStreamExt;
use mongodb::{
    bson::doc, options::FindOneAndUpdateOptions, options::FindOptions, options::IndexOptions,
    Client, Collection, IndexModel,
};

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

    pub async fn post_todo(
        &self,
        request: Json<SubmitTodoRequest>,
        user_id: String,
    ) -> Result<Todo, ServiceError> {
        let new_todo = Todo::new(request, user_id);
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

    pub async fn get_todos(
        &self,
        todo_pagination: TodoPagination,
        user_id: String,
    ) -> Result<Vec<Todo>, ServiceError> {
        let find_options = FindOptions::builder()
            .limit(todo_pagination.page_size)
            .skip(((todo_pagination.page - 1) * todo_pagination.page_size) as u64)
            .build();
        let filter = doc! {
            "user_id": &user_id
        };

        match self.col.find(filter, Some(find_options)).await {
            Ok(mut cursor) => {
                let mut todos = Vec::new();
                while let Ok(Some(result)) = cursor.try_next().await {
                    todos.push(result);
                }
                Ok(todos)
            }
            Err(_) => Err(ServiceError::NotFound {
                error_message: "Failed to retrieve todos".to_string(),
            }),
        }
    }

    pub async fn get_todo(&self, uuid: String, user_id: String) -> Result<Todo, ServiceError> {
        let filter = doc! {
            "uuid": &uuid,
            "user_id": &user_id
        };
        match self.col.find_one(filter, None).await {
            Ok(Some(todo)) => Ok(todo),
            Ok(None) => Err(ServiceError::NotFound {
                error_message: MESSAGE_CAN_NOT_FETCH_DATA.to_string(),
            }),
            Err(_) => Err(ServiceError::InternalServerError {
                error_message: "Failed to retrieve todo".to_string(),
            }),
        }
    }

    pub async fn put_todo(
        &self,
        uuid: String,
        user_id: String,
        updated_todo: Todo,
    ) -> Result<Todo, ServiceError> {
        let filter = doc! {
            "uuid": &uuid,
            "user_id": &user_id
        };
        let update = doc! {
            "$set": {
                "title": updated_todo.title,
                "state": updated_todo.state,
            }
        };
        let options = FindOneAndUpdateOptions::builder()
            .return_document(mongodb::options::ReturnDocument::After)
            .build();

        match self.col.find_one_and_update(filter, update, options).await {
            Ok(Some(todo)) => Ok(todo),
            Ok(None) => Err(ServiceError::NotFound {
                error_message: MESSAGE_CAN_NOT_FETCH_DATA.to_string(),
            }),
            Err(_) => Err(ServiceError::InternalServerError {
                error_message: "Failed to update todo".to_string(),
            }),
        }
    }

    pub async fn delete_todo(&self, uuid: String, user_id: String) -> Result<(), ServiceError> {
        let filter = doc! {
            "uuid": &uuid,
            "user_id": &user_id
        };
        let delete_result = self.col.delete_one(filter, None).await;

        match delete_result {
            Ok(result) => {
                if result.deleted_count == 0 {
                    Err(ServiceError::NotFound {
                        error_message: "Todo not found".to_string(),
                    })
                } else {
                    Ok(())
                }
            }
            Err(_) => Err(ServiceError::InternalServerError {
                error_message: MESSAGE_CAN_NOT_DELETE_DATA.to_string(),
            }),
        }
    }

    pub async fn patch_todo(
        &self,
        uuid: String,
        user_id: String,
        update_data: TodoUpdate,
    ) -> Result<Todo, ServiceError> {
        let filter = doc! {
            "uuid": &uuid,
            "user_id": &user_id
        };
        let update = doc! {
            "$set": update_data.to_doc()
        };

        let options = FindOneAndUpdateOptions::builder()
            .return_document(mongodb::options::ReturnDocument::After)
            .build();

        match self.col.find_one_and_update(filter, update, options).await {
            Ok(Some(todo)) => Ok(todo),
            Ok(None) => Err(ServiceError::NotFound {
                error_message: "Todo not found".to_string(),
            }),
            Err(_) => Err(ServiceError::InternalServerError {
                error_message: "Failed to update todo".to_string(),
            }),
        }
    }
}
