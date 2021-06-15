use std::collections::HashMap;

mod common;

#[tokio::test]
async fn test_people_create() {
    //spawn the app so the server is running
    //need to block on this or the request can happen before the server starts
    let app_address = tokio::join!(common::spawn_app()).0.unwrap();
    let client = reqwest::Client::new();

    // Create the health endpoint and set a get request to the health endpoint
    let endpoint = format!(
        "http://{}:{}/api/people",
        app_address.ip(),
        app_address.port()
    );

    //Create the new person entry
    let mut map = HashMap::new();
    map.insert("fname", "Samuel");
    map.insert("lname", "Vimes");

    //Build a response
    let resp = client.post(endpoint).json(&map).send().await.unwrap();

    // assert that we got a "success" code back
    assert!(resp.status().is_success());
}
