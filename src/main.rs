use byte_genius_hosting::configuration::get_configuration;
use byte_genius_hosting::startup::run;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to load configuration");
    let app_settings = configuration.application;

    let address = format!("{}:{}", app_settings.address, app_settings.port);
    let listener =
        TcpListener::bind(&address).expect(&format!("Failed to bind address {}", address));

    println!("Listener {}", address);

    run(listener)?.await
}
