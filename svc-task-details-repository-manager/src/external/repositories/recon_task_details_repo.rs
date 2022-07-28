use async_trait::async_trait;

use crate::internal::{
    interfaces::recon_tasks_repository::ReconTaskDetailsRepositoryInterface,
    models::entities::{
        app_errors::{AppError, AppErrorKind},
        recon_tasks_models::{ReconTaskDetails, ReconciliationConfigs},
    },
};

pub struct ReconTaskDetailsRepositoryManager {
    pub connection_url: String,
    pub store_name: String,
}

#[async_trait]
impl ReconTaskDetailsRepositoryInterface for ReconTaskDetailsRepositoryManager {
    async fn get_task_details(&self, task_details: &String) -> Result<ReconTaskDetails, AppError> {
        // Create the client
        let connection_url = self.connection_url.clone();
        let client_connect_result =
            dapr::Client::<dapr::client::TonicClient>::connect(connection_url).await;

        let mut client;

        match client_connect_result {
            Ok(s) => client = s,
            Err(e) => return Err(AppError::new(AppErrorKind::ConnectionError, e.to_string())),
        }

        let get_response = client
            .get_state(self.store_name.clone(), String::from(task_details), None)
            .await;

        match get_response {
            Ok(s) => {
                let retrieval_result: Result<ReconTaskDetails, _> = serde_json::from_slice(&s.data);
                match retrieval_result {
                    Ok(unmarshalled_task_details) => return Ok(unmarshalled_task_details),
                    Err(e) => return Err(AppError::new(AppErrorKind::NotFound, e.to_string())),
                };
            }
            Err(e) => return Err(AppError::new(AppErrorKind::NotFound, e.to_string())),
        }
    }

    async fn create_task_details(
        &self,
        task_details: &ReconTaskDetails,
    ) -> Result<String, AppError> {
        // Create the client
        let connection_url = self.connection_url.clone();
        let client_connect_result =
            dapr::Client::<dapr::client::TonicClient>::connect(connection_url).await;

        let mut client;

        match client_connect_result {
            Ok(s) => client = s,
            Err(e) => return Err(AppError::new(AppErrorKind::ConnectionError, e.to_string())),
        }

        let key = task_details.id.clone();
        let val = serde_json::to_vec(&task_details).unwrap();

        // save key-value pair in the state store
        let save_result = client
            .save_state(self.store_name.clone(), vec![(key.clone(), val)])
            .await;

        match save_result {
            Ok(_s) => return Ok(key.clone()),
            Err(e) => return Err(AppError::new(AppErrorKind::InternalError, e.to_string())),
        }
    }

    async fn update_task_details(
        &self,
        _task_details: &ReconTaskDetails,
    ) -> Result<ReconTaskDetails, AppError> {
        return Ok(ReconTaskDetails {
            comparison_file_id: String::from("1234"),
            has_begun: false,
            id: String::from("1234"),
            is_done: false,
            source_file_id: String::from("1234"),
            comparison_pairs: vec![],
            recon_config: ReconciliationConfigs {
                should_check_for_duplicate_records_in_comparison_file: true,
                should_reconciliation_be_case_sensitive: true,
                should_ignore_white_space: true,
                should_do_reverse_reconciliation: true,
            },
        });
    }

    async fn delete_task_details(&self, task_details_id: &String) -> Result<bool, AppError> {
        // Create the client
        let connection_url = self.connection_url.clone();
        let client_connect_result =
            dapr::Client::<dapr::client::TonicClient>::connect(connection_url).await;

        let mut client;

        match client_connect_result {
            Ok(s) => client = s,
            Err(e) => return Err(AppError::new(AppErrorKind::ConnectionError, e.to_string())),
        }

        // delete a value from the state store
        let delete_result = client
            .delete_state(self.store_name.clone(), String::from(task_details_id), None)
            .await;

        match delete_result {
            Ok(_s) => return Ok(true),
            Err(e) => return Err(AppError::new(AppErrorKind::InternalError, e.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {

    #[actix_rt::test]
    async fn given_valid_create_recon_task_request_calls_correct_dependencies_returns_success() {}
}
