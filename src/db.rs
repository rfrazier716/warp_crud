use crate::{data, error::Error::*, Result};

use chrono::prelude::*;
use mongodb::bson;
use mongodb::bson::{doc, serde_helpers::serialize_uuid_as_binary, Bson, Document, Serializer};
use uuid::Uuid;

const DB_NAME: &str = "warp_crud"; // database name

// Create a few consts for inserting and requesting from database
const SESSION: &str = "session.id";
const TODOS: &str = "todos";

pub(crate) type Client = mongodb::Client;

pub async fn ping(client: &Client) -> Result<Document> {
    client
        .database("admin")
        .run_command(doc! {"ping":1}, None)
        .await
        .map_err(MongoQueryError)
}

pub fn uuid_to_bson(uuid: &Uuid) -> Result<Bson> {
    serialize_uuid_as_binary(uuid, Serializer::new()).map_err(SerializationError)
}

pub async fn create_todo_list(client: &Client) -> Result<data::TodoList> {
    // Create a new dummy todo list
    let todo_list = data::TodoList {
        session: data::Session::new(),
        todos: vec!["Delete This Todo".into()],
    };

    // Insert it into the Database
    client
        .database(DB_NAME)
        .collection::<data::TodoList>(TODOS)
        .insert_one(&todo_list, None)
        .await
        .map_err(MongoQueryError)?;
    Ok(todo_list)
}

pub async fn get_todos(client: &Client, session: &data::Session) -> Result<Vec<data::Todo>> {
    let filter = doc! {SESSION: uuid_to_bson(session.id())?};

    let result = client
        .database(DB_NAME)
        .collection::<data::TodoList>(TODOS)
        // .collection::<Document>(TODOS)
        .find_one(Some(filter), None)
        .await
        .map_err(MongoQueryError)?;

    if let Some(todo_list) = result {
        Ok(todo_list.todos)
    } else {
        Err(NonexistentResourceError)
    }
}

pub async fn create_todo(
    client: &Client,
    session: &data::Session,
    todo: &data::Todo,
) -> Result<()> {
    // Create a TODO and Only keep the 10 most recent ones
    let filter = doc! {SESSION: uuid_to_bson(session.id())?};
    let todo = bson::to_bson(todo).map_err(SerializationError)?;
    let update = doc! {
    "$push": {
        "todos": {
            "$each": vec![todo],
            "$slice": 10,
        }
    }};

    // Find the Document and push a todo
    client
        .database(DB_NAME)
        .collection::<Document>(TODOS)
        .find_one_and_update(filter, update, None)
        .await
        .map_err(MongoQueryError)?;

    Ok(())
}

pub async fn update_todo(
    client: &Client,
    session: &data::Session,
    todo_id: &uuid::Uuid,
    update: &data::TodoRequest,
) -> Result<()> {
    let filter = doc! {
        SESSION: uuid_to_bson(session.id())?,
        "todos.id": uuid_to_bson(todo_id)?
    };

    let update = doc! { "$set": { "todos.$.name": &update.name , "todos.$.timestamp": bson::to_bson(&Utc::now()).unwrap()}};

    client
        .database(DB_NAME)
        .collection::<Document>(TODOS)
        .find_one_and_update(filter, update, None)
        .await
        .map_err(MongoQueryError)?;

    Ok(())
}

pub async fn delete_todo(
    client: &Client,
    session: &data::Session,
    todo_id: &uuid::Uuid,
) -> Result<()> {
    let filter = doc! {SESSION: uuid_to_bson(session.id())?};
    let update =
        doc! {"$pull": {"todos": {"id": uuid_to_bson(todo_id)?}}};
    client
        .database(DB_NAME)
        .collection::<Document>(TODOS)
        .find_one_and_update(filter, update, None)
        .await
        .map_err(MongoQueryError)?;

    Ok(())
}

pub async fn delete_all_todos(client: &Client, session: &data::Session) -> Result<()> {
    let filter = doc! {SESSION: uuid_to_bson(session.id())?};
    let update = doc! { "$set": {"todos": []}};

    client
        .database(DB_NAME)
        .collection::<Document>(TODOS)
        .find_one_and_update(filter, update, None)
        .await
        .map_err(MongoQueryError)?;

    Ok(())
}
