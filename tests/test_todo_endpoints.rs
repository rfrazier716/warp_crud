mod common;
use warp_crud::data;

#[tokio::test]
async fn test_getting_todos() {
    //spawn the app so the server is running
    //need to block on this or the request can happen before the server starts
    let app_address = tokio::join!(common::spawn_app()).0.unwrap();
    let client = reqwest::Client::new();

    // Create the health endpoint and set a get request to the health endpoint
    let endpoint = format!(
        "http://{}:{}/api/todos",
        app_address.ip(),
        app_address.port()
    );

    //Build a response
    let resp = client.get(endpoint).send().await.unwrap();

    // Verify we got a success
    assert!(resp.status().is_success());

    let body = resp.json::<Vec<data::Todo>>().await.unwrap();

    // assure that the body has a length of 1 and that the only element says the default message
    assert_eq!(body.len(), 1);
    assert_eq!(body[0].name, "Delete This Todo");


    // assert that we got a "success" code back
}

async fn test_creating_todo() {
    //spawn the app so the server is running
    //need to block on this or the request can happen before the server starts
    let app_address = tokio::join!(common::spawn_app()).0.unwrap();
    let client = reqwest::Client::new();

    // Create the todos endpoint
    let endpoint = format!(
        "http://{}:{}/api/todos",
        app_address.ip(),
        app_address.port()
    );

    let new_todo = data::TodoRequest{name: "Run To The Hills!".to_owned()};

    //Build a response
    let resp = client.post(endpoint).json::<data::TodoRequest>(&new_todo).send().await.unwrap();

    // Verify we got a success
    assert!(resp.status().is_success());

}