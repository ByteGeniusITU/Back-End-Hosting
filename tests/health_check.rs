use byte_genius_hosting::startup::run;
use std::net::TcpListener;

#[actix_rt::test]
async fn health_check_should_return_200() {
    let address = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health_check", address))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(200, response.status().as_u16())
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind address");
    let port = listener.local_addr().unwrap().port();

    let server = run(listener).expect("Failed to run server");

    tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
