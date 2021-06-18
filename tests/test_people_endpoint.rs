use std::collections::HashMap;
use warp_crud::data::Person;

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

#[tokio::test]
async fn test_people_read() {
    //spawn the app so the server is running
    //need to block on this or the request can happen before the server starts
    let app_address = tokio::join!(common::spawn_app()).0.unwrap();
    let client = reqwest::Client::new();

    // Create the health endpoint and set a get request to the health endpoint
    let base_endpoint = format!(
        "http://{}:{}/api/people",
        app_address.ip(),
        app_address.port()
    );

    //Build a response
    let resp = client.get(&base_endpoint).send().await.unwrap();

    // assert that we got a "success" code back
    assert!(resp.status().is_success());

    //assert that the body of the response is a vector and each element can be converted into a person
    let people = resp
        .json::<Vec<Person>>()
        .await
        .map_err(|_source| String::from("Could not convert reply into list of People Structs"))
        .unwrap();
    assert!(!people.is_empty()); // make sure the reply isn't empty

    //now test that we can readback a single person
    let person_id = &people[0].id;

    // make a new endpoint
    let person_endpoint = format!("{}/{}", &base_endpoint, person_id);
    let resp = client.get(&person_endpoint).send().await.unwrap();
    assert!(resp.status().is_success());

    let person = resp
        .json::<Person>()
        .await
        .map_err(|x| format!("Could not convert reply into Person {}", x))
        .unwrap();
    assert_eq!(person, people[0]); // make sure we got the same person back that the ID matches

    // Make a request with an invalid endpoint (too short to be an object ID)
    // the error filter should make this a 404
    let malformed_id_endpoint = format!("{}/{}", &base_endpoint, "deadbeef");
    let resp = client.get(&malformed_id_endpoint).send().await.unwrap();
    assert_eq!(resp.status().as_u16(), 404); // verify we get a 404 error

    // now make a couple dummy requests - this is the right id layout but nonexistant
    let nonexistent_id_endpoint = format!("{}/{}", &base_endpoint, "deadbeefdeadbeefdeadbeef");

    let resp = client.get(&nonexistent_id_endpoint).send().await.unwrap();
    assert_eq!(resp.status().as_u16(), 404); // verify we get a 404 error
}
