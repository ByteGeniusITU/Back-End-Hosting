use byte_genius_hosting::startup::run;
use byte_genius_hosting::telemetry::*;
use once_cell::sync::Lazy;
use std::env;
use std::net::TcpListener;

static TRACING: Lazy<()> = Lazy::new(|| {
    let subscriber_name = "tests".into();
    let default_filter_level = "info".into();

    if env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    }
});

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

    Lazy::force(&TRACING);

    let server = run(listener).expect("Failed to run server");

    tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
