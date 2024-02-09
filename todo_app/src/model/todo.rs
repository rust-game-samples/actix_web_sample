use actix_web::web::Json;
use mongodb::bson::{doc, Document};
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use uuid::Uuid;

#[derive(Serialize, EnumString, Display, Eq, PartialEq)]
pub enum TodoState {
    NotStarted,
    InProgress,
    Completed,
    Paused,
    Failed,
}

#[derive(Deserialize)]
pub struct SubmitTodoRequest {
    pub title: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Todo {
    pub uuid: String,
    pub user_id: String,
    pub title: String,
    pub state: String,
}
impl Todo {
    pub fn new(request: Json<SubmitTodoRequest>) -> Todo {
        Todo {
            uuid: Uuid::new_v4().to_string(),
            user_id: "".to_string(),
            title: request.title.to_string(),
            state: TodoState::NotStarted.to_string(),
        }
    }

    pub fn from_db(todo: Todo) -> Todo {
        Todo {
            uuid: todo.uuid.clone(),
            user_id: todo.user_id.clone(),
            title: todo.title.clone(),
            state: todo.state.clone(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct TodoUpdate {
    pub title: Option<String>,
    pub state: Option<String>,
}

impl TodoUpdate {
    pub fn to_doc(&self) -> Document {
        let mut update_doc = Document::new();
        if let Some(title) = &self.title {
            update_doc.insert("title", title);
        }
        if let Some(state) = &self.state {
            update_doc.insert("state", state);
        }
        update_doc
    }
}
