mod core;
mod repositories;

use crate::core::{
    models::app_error::AppErrorKind, view_models::upload_file_chunk_request::UploadFileChunkRequest,
};

use actix_web::{
    post,
    web::{self, Data},
    App, HttpResponse, HttpServer,
};

use crate::{
    core::{
        interfaces::file_upload_service::FileUploadServiceInterface,
        services::file_upload_service::FileUploadService,
    },
    repositories::file_upload_repo::FileUploadRepositoryManager,
};

const DEFAULT_DAPR_CONNECTION_URL: &'static str = "http://localhost:5005";
const DEFAULT_PUBSUB_NAME: &'static str = "localStorage";
const DEFAULT_PUBSUB_TOPIC: &'static str = "localStorage";
const DEFAULT_APP_LISTEN_IP: &'static str = "0.0.0.0";
const DEFAULT_APP_LISTEN_PORT: u16 = 8080;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //retrieve app settings from the env variables
    let app_port = std::env::var("PORT").unwrap_or(DEFAULT_APP_LISTEN_PORT.to_string());
    let app_ip = std::env::var("IP").unwrap_or(DEFAULT_APP_LISTEN_IP.to_string());
    let app_listen_url = format!("{app_ip}:{app_port}");

    println!("App is listening on: {:?}", app_listen_url);

    HttpServer::new(move || {
        //retrieve settings from the env variables
        let dapr_pubsub_name =
            std::env::var("PUBSUB_NAME").unwrap_or(DEFAULT_PUBSUB_NAME.to_string());
        let dapr_pubsub_topic =
            std::env::var("PUBSUB_TOPIC").unwrap_or(DEFAULT_PUBSUB_TOPIC.to_string());
        let dapr_connection_url =
            std::env::var("IP").unwrap_or(DEFAULT_DAPR_CONNECTION_URL.to_string());

        // Create some global state prior to running the handler threads
        let service: Box<dyn FileUploadServiceInterface> = Box::new(FileUploadService {
            file_upload_repo: Box::new(FileUploadRepositoryManager {
                connection_url: String::from(dapr_connection_url.clone()),
                pubsub_name: dapr_pubsub_name,
                pubsub_topic: dapr_pubsub_topic,
            }),
        });

        App::new()
            .app_data(Data::new(service)) // add shared state
            .service(upload_file_chunk)
    })
    .bind(app_listen_url)?
    .run()
    .await
}

#[post("/upload-file-chunk")]
async fn upload_file_chunk(
    task_details: web::Json<UploadFileChunkRequest>,
    service: Data<Box<dyn FileUploadServiceInterface>>,
) -> HttpResponse {
    let recon_task_details = service.upload_file_chunk(&task_details.0).await;

    return match recon_task_details {
        Ok(details) => HttpResponse::Ok().json(details),
        Err(err) => match err.kind {
            AppErrorKind::BadClientRequest => HttpResponse::BadRequest().json(format!("{}", err)),
            _ => HttpResponse::InternalServerError().json(format!("{}", err)),
        },
    };
}
