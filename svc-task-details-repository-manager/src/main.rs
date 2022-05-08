use crate::services::{
    interfaces::ReconTaskAggregationServiceInterface,
    view_models::{AppErrorKind, CreateReconTaskRequest, GetTaskDetailsRequest},
};
use actix_web::{
    get, post,
    web::{self, Data, Path},
    App, HttpResponse, HttpServer,
};
use repositories::{
    recon_file_details_repo::ReconFileDetailsRepositoryManager,
    recon_task_details_repo::ReconTaskDetailsRepositoryManager,
};
use services::recon_task_aggregation::ReconTaskAggregationService;

mod repositories;
mod services;

const DEFAULT_DAPR_CONNECTION_URL: &'static str = "http://localhost:5005";
const DEFAULT_DAPR_STORE_NAME: &'static str = "statestore";
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
        //retrieve data store settings from the env variables
        let dapr_store_name = std::env::var("IP").unwrap_or(DEFAULT_DAPR_STORE_NAME.to_string());
        let dapr_connection_url =
            std::env::var("IP").unwrap_or(DEFAULT_DAPR_CONNECTION_URL.to_string());

        // Create some global state prior to running the handler threadsss
        let service: Box<dyn ReconTaskAggregationServiceInterface> =
            Box::new(ReconTaskAggregationService {
                recon_task_details_repo: Box::new(ReconTaskDetailsRepositoryManager {
                    connection_url: String::from(dapr_connection_url.clone()),
                    store_name: String::from(dapr_store_name.clone()),
                }),
                recon_file_details_repo: Box::new(ReconFileDetailsRepositoryManager {
                    connection_url: String::from(dapr_connection_url.clone()),
                    store_name: String::from(dapr_store_name.clone()),
                }),
            });

        App::new()
            .app_data(Data::new(service)) // add shared state
            .service(get_task_details)
            .service(create_task_details)
    })
    .bind(app_listen_url)?
    .run()
    .await
}

#[get("/task-details/{task_id}")]
async fn get_task_details(
    get_task_details_request: Path<GetTaskDetailsRequest>,
    service: Data<Box<dyn ReconTaskAggregationServiceInterface>>,
) -> HttpResponse {
    let task_id = &get_task_details_request.task_id;
    let recon_task_details = service.get_recon_task(task_id).await;
    return match recon_task_details {
        Ok(details) => HttpResponse::Ok().json(details),
        Err(err) => match err.kind {
            AppErrorKind::NotFound => HttpResponse::NotFound().json(format!("{}", err)),
            _ => HttpResponse::InternalServerError().json(format!("{}", err)),
        },
    };
}

#[post("/task-details")]
async fn create_task_details(
    task_details: web::Json<CreateReconTaskRequest>,
    service: Data<Box<dyn ReconTaskAggregationServiceInterface>>,
) -> HttpResponse {
    let recon_task_details = service.create_recon_task(&task_details.0).await;

    return match recon_task_details {
        Ok(details) => HttpResponse::Ok().json(details),
        Err(err) => match err.kind {
            AppErrorKind::BadClientRequest => HttpResponse::BadRequest().json(format!("{}", err)),
            _ => HttpResponse::InternalServerError().json(format!("{}", err)),
        },
    };
}

#[cfg(test)]
mod tests {
    use actix_web::{
        test::{self, TestRequest},
        App,
    };

    use crate::services::{
        interfaces::MockReconTaskAggregationServiceInterface, view_models::ReconTaskResponseDetails,
    };

    use super::*;

    #[actix_web::test]
    async fn test_get_task_details_given_valid_id_returns_success() {
        let mut app = test::init_service((move || {
            // Create some global state prior to running the handler thread
            let mut mock_recon_task_aggregation_service =
                Box::new(MockReconTaskAggregationServiceInterface::new());

            mock_recon_task_aggregation_service
                .expect_get_recon_task()
                .returning(|_y| {
                    Ok(ReconTaskResponseDetails {
                        task_id: String::from("task-1234"),
                        is_done: false,
                        has_begun: false,
                    })
                });

            let service: Box<dyn ReconTaskAggregationServiceInterface> =
                mock_recon_task_aggregation_service;

            App::new()
                .app_data(Data::new(service)) // add shared state
                .service(get_task_details)
        })())
        .await;

        let resp = TestRequest::get()
            .uri(&format!("/task-details/123456"))
            .send_request(&mut app)
            .await;

        assert!(resp.status().is_success());
    }
}
