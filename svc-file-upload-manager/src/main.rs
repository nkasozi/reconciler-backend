mod core;
mod repositories;

//const DEFAULT_DAPR_CONNECTION_URL: &'static str = "http://localhost:5005";
//const DEFAULT_DAPR_BINDING_NAME: &'static str = "localStorage";
const DEFAULT_APP_LISTEN_IP: &'static str = "0.0.0.0";
const DEFAULT_APP_LISTEN_PORT: u16 = 8080;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //retrieve app settings from the env variables
    let app_port = std::env::var("PORT").unwrap_or(DEFAULT_APP_LISTEN_PORT.to_string());
    let app_ip = std::env::var("IP").unwrap_or(DEFAULT_APP_LISTEN_IP.to_string());
    let app_listen_url = format!("{app_ip}:{app_port}");

    println!("App is listening on: {:?}", app_listen_url);

    Ok(())
}
