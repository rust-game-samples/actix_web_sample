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
    pub fn new(request: Json<SubmitTodoRequest>, user_id: String) -> Todo {
        Todo {
            uuid: Uuid::new_v4().to_string(),
            user_id,
            title: request.title.to_string(),
            state: TodoState::NotStarted.to_string(),
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

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct TodoPaginationRequest {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct TodoPagination {
    pub page: i64,
    pub page_size: i64,
}
impl TodoPagination {
    pub fn new(request: Json<TodoPaginationRequest>) -> TodoPagination {
        let mut new_page = 1;
        let mut new_page_size = 30;
        if let Some(page) = &request.page {
            new_page = *page;
        }
        if let Some(page_size) = &request.page_size {
            new_page_size = *page_size;
        }

        TodoPagination {
            page: new_page,
            page_size: new_page_size,
        }
    }
}
