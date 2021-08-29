use chrono::prelude::*;
use mongodb::bson::serde_helpers;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Person {
    pub id: String,
    pub fname: String,
    pub lname: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct PersonRequest {
    pub fname: String,
    pub lname: String,
}
#[derive(Serialize, Deserialize)]
pub struct TodoRequest {
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct Todo {
    pub id: uuid::Uuid,
    pub name: String,
    pub timestamp: DateTime<Utc>,
}

impl From<TodoRequest> for Todo {
    fn from(request: TodoRequest) -> Self {
        Self::new(&request.name)
    }
}

impl From<&str> for Todo {
    fn from(todo: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            name: todo.to_owned(),
            timestamp: Utc::now(),
        }
    }
}

impl Todo {
    fn new(todo: &str) -> Self {
        todo.into()
    }
}

#[derive(Deserialize, Serialize)]
pub struct Session {
    #[serde(with = "serde_helpers::uuid_as_binary")]
    id: uuid::Uuid,
}

impl Session {
    pub fn new() -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
        }
    }

    pub fn id(&self) -> &uuid::Uuid {
        &self.id
    }
}

impl Default for Session {
    fn default() -> Self {
        Self { id: Default::default() }
    }
}

impl From<uuid::Uuid> for Session {
    fn from(id: uuid::Uuid) -> Self {
        Self { id }
    }
}

#[derive(Serialize, Deserialize)]
pub struct TodoList {
    pub session: Session,
    pub todos: Vec<Todo>,
}
