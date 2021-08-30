mod common;
use warp_crud::data;

#[tokio::test]
async fn test_getting_todos() {
    //spawn the app so the server is running
    //need to block on this or the request can happen before the server starts

    let app = common::App::launch(Some("Test")).await.unwrap();
    let client = reqwest::Client::new();

    let resp = client.get(app.route("/api/todos")).send().await.unwrap();

    // Verify we got a success
    assert!(resp.status().is_success());

    let body = resp.json::<Vec<data::Todo>>().await.unwrap();

    // assure that the body has a length of 1 and that the only element says the default message
    assert_eq!(body.len(), 1);
    assert_eq!(body[0].name, "Delete This Todo");
}

// TODO: Figure out how to initialize a new cookie session
#[tokio::test]
async fn test_creating_todo() {
    //spawn the app so the server is running
    //need to block on this or the request can happen before the server starts
    let app = common::App::launch(Some("Test")).await.unwrap();
    let endpoint = app.route("/api/todos");
    let client = reqwest::Client::builder().cookie_store(true).build().expect("Could not Create Client");



    // Run a get reqest to the app so we get a session cookie back
    client.get(&endpoint).send().await.expect("Error Running Get Request to App");

    let new_todo = data::TodoRequest{name: "Run To The Hills!".to_owned()};

    // Send a post request to the
    let resp = client.post(&endpoint).json::<data::TodoRequest>(&new_todo).send().await.unwrap();

    // Verify we got a success
    assert!(resp.status().is_success());

    // Verify that if we now run a get request we should have 2 items, 
    let resp = client.get(&endpoint).send().await.expect("Error Running Get Request to App");
    let body = resp.json::<Vec<data::Todo>>().await.unwrap();

    // assure that the body has a length of 1 and that the only element says the default message
    assert_eq!(body.len(), 2);
    assert_eq!(body[1].name, "Run To The Hills!");
}

#[tokio::test]
async fn test_updating_todo() {
    //spawn the app so the server is running
    //need to block on this or the request can happen before the server starts
    let app = common::App::launch(Some("Test")).await.unwrap();
    let endpoint = app.route("/api/todos");
    let client = reqwest::Client::builder().cookie_store(true).build().expect("Could not Create Client");



    // Run a get reqest to the app so we get a session cookie back
    let resp = client.get(&endpoint).send().await.expect("Error Running Get Request to App");
    let todo_id = resp.json::<Vec<data::Todo>>().await.unwrap()[0].id;

    // make a new todo request and send an update request with the todo_id endpoint
    let new_todo = data::TodoRequest{name: "Run To The Hills!".to_owned()};
    let resp = client.put(format!("{}/{}",endpoint, todo_id)).json::<data::TodoRequest>(&new_todo).send().await.unwrap();

    // Verify we got a success
    assert!(resp.status().is_success());

    // Verify that if we now run a get request we get the updated item
    let resp = client.get(&endpoint).send().await.expect("Error Running Get Request to App");
    let body = resp.json::<Vec<data::Todo>>().await.unwrap();

    // assure that the body has a length of 1 and that the only element says the default message
    assert_eq!(body.len(), 1);
    assert_eq!(body[0].name, "Run To The Hills!");
}

#[tokio::test]
async fn test_deleting_todo() {
    //spawn the app so the server is running
    //need to block on this or the request can happen before the server starts
    let app = common::App::launch(Some("Test")).await.unwrap();
    let endpoint = app.route("/api/todos");
    let client = reqwest::Client::builder().cookie_store(true).build().expect("Could not Create Client");

    // Run a get reqest to the app so we get a session cookie back
    let resp = client.get(&endpoint).send().await.expect("Error Running Get Request to App");
    let todo_id = resp.json::<Vec<data::Todo>>().await.unwrap()[0].id;

    // Send a Delete request containing the id of the todo we want to delete
    let resp = client.delete(format!("{}/{}",endpoint, todo_id)).send().await.unwrap();

    // Verify we got a success
    assert!(resp.status().is_success());

    // Verify that if we now run a get request we get an empty vector
    let resp = client.get(&endpoint).send().await.expect("Error Running Get Request to App");
    let body = resp.json::<Vec<data::Todo>>().await.unwrap();

    // assure that the body has a length of 1 and that the only element says the default message
    assert_eq!(body.len(), 0);
}