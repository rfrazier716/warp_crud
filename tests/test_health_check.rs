mod common;

#[tokio::test]
async fn test_health_check_endpoint() {
    //spawn the app so the server is running
    //need to block on this or the request can happen before the server starts
    let app = common::App::launch(Some("Test")).await.unwrap();
    let client = reqwest::Client::new();

    // Create the health endpoint and set a get request to the health endpoint
    let resp = client.get(app.route("/health")).send().await.unwrap();

    // assert that we got a "success" error code back
    assert!(resp.status().is_success());
}
