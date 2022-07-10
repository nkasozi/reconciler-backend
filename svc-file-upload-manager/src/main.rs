mod external;
mod internal;

use crate::internal::{
    entities::app_error::AppErrorKind,
    view_models::upload_file_chunk_request::UploadFileChunkRequest,
};
use crate::{
    external::repositories::dapr_file_upload_repo::DaprFileUploadRepositoryManager,
    internal::{
        interfaces::file_upload_service::FileUploadServiceInterface,
        services::file_upload_service::FileUploadService,
    },
};
use actix_web::{
    post,
    web::{self, Data},
    App, HttpResponse, HttpServer,
};

// constants
const DEFAULT_DAPR_CONNECTION_URL: &'static str = "http://localhost:5005";
const DEFAULT_DAPR_PUBSUB_NAME: &'static str = "FileChunksQueue";
const DEFAULT_DAPR_PUBSUB_TOPIC: &'static str = "FileChunks";
const DEFAULT_APP_LISTEN_IP: &'static str = "0.0.0.0";
const DEFAULT_APP_LISTEN_PORT: u16 = 8080;

struct AppSettings {
    pub app_port: String,

    pub app_ip: String,

    pub dapr_pubsub_name: String,

    pub dapr_pubsub_topic: String,

    pub dapr_grpc_server_address: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //retrieve app settings from the env variables
    let app_settings = read_app_settings();
    let app_listen_url = format!("{}:{}", app_settings.app_ip, app_settings.app_port);

    //just for logging purposes
    println!("App is listening on: {:?}", app_listen_url);

    HttpServer::new(move || {
        // Create some global state prior to running the handler threads
        let service: Box<dyn FileUploadServiceInterface> = Box::new(FileUploadService {
            file_upload_repo: Box::new(DaprFileUploadRepositoryManager {
                dapr_grpc_server_address: app_settings.dapr_grpc_server_address.clone(),
                dapr_pubsub_name: app_settings.dapr_pubsub_name.clone(),
                dapr_pubsub_topic: app_settings.dapr_pubsub_topic.clone(),
            }),
        });

        // add shared state and routing
        App::new()
            .app_data(Data::new(service))
            .service(upload_file_chunk)
    })
    .bind(app_listen_url)?
    .run()
    .await
}

fn read_app_settings() -> AppSettings {
    AppSettings {
        app_port: std::env::var("APP_PORT").unwrap_or(DEFAULT_APP_LISTEN_PORT.to_string()),
        app_ip: std::env::var("APP_IP").unwrap_or(DEFAULT_APP_LISTEN_IP.to_string()),
        dapr_pubsub_name: std::env::var("PUBSUB_NAME")
            .unwrap_or(DEFAULT_DAPR_PUBSUB_NAME.to_string()),
        dapr_pubsub_topic: std::env::var("PUBSUB_TOPIC")
            .unwrap_or(DEFAULT_DAPR_PUBSUB_TOPIC.to_string()),
        dapr_grpc_server_address: std::env::var("DAPR_IP")
            .unwrap_or(DEFAULT_DAPR_CONNECTION_URL.to_string()),
    }
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
