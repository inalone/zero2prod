use std::net::TcpListener;

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();

    let server = zero2prod::run(listener);
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_check_works() {
    let addr = spawn_app();
    let client = reqwest::Client::new();

    let endpoint = format!("{}/health-check", addr);

    let response = client
        .get(endpoint)
        .send()
        .await
        .expect("Failed to execute request");

    println!("{}", response.status());
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
