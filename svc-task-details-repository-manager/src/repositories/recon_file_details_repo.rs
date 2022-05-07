use crate::services::{
    entities::ReconFileDetails,
    interfaces::ReconFileDetailsRepositoryInterface,
    view_models::{AppError, AppErrorKind},
};

use async_trait::async_trait;

pub struct ReconFileDetailsRepositoryManager {
    pub connection_url: String,
    pub store_name: String,
}

#[async_trait]
impl ReconFileDetailsRepositoryInterface for ReconFileDetailsRepositoryManager {
    fn get_connection_string(&self) -> String {
        self.connection_url.to_owned()
    }

    fn get_store_name(&self) -> String {
        self.store_name.to_owned()
    }

    async fn get_recon_file_details(&self, file_id: &String) -> Result<ReconFileDetails, AppError> {
        // Create the client
        let connection_url = self.connection_url.clone();
        let store_name = self.store_name.clone();

        let client_connect_result =
            dapr::Client::<dapr::client::TonicClient>::connect(connection_url).await;

        let mut client;

        match client_connect_result {
            Ok(s) => client = s,
            Err(e) => return Err(AppError::new(AppErrorKind::ConnectionError, e.to_string())),
        }

        let get_response = client
            .get_state(store_name, String::from(file_id), None)
            .await;

        match get_response {
            Ok(s) => {
                let retrieval_result: Result<ReconFileDetails, _> = serde_json::from_slice(&s.data);
                match retrieval_result {
                    Ok(unmarshalled_file_details) => return Ok(unmarshalled_file_details),
                    Err(e) => return Err(AppError::new(AppErrorKind::NotFound, e.to_string())),
                };
            }
            Err(e) => return Err(AppError::new(AppErrorKind::NotFound, e.to_string())),
        }
    }

    async fn create_recon_file_details(
        &self,
        file_details: &ReconFileDetails,
    ) -> Result<String, AppError> {
        // Create the client
        let connection_url = self.connection_url.clone();
        let store_name = self.store_name.clone();

        let client_connect_result =
            dapr::Client::<dapr::client::TonicClient>::connect(connection_url).await;

        let mut client;

        match client_connect_result {
            Ok(s) => client = s,
            Err(e) => return Err(AppError::new(AppErrorKind::ConnectionError, e.to_string())),
        }

        let key = file_details.id.clone();
        let val = serde_json::to_vec(&file_details).unwrap();

        // save key-value pair in the state store
        let save_result = client
            .save_state(store_name, vec![(key.clone(), val)])
            .await;

        match save_result {
            Ok(_s) => return Ok(key.clone()),
            Err(e) => return Err(AppError::new(AppErrorKind::InternalError, e.to_string())),
        }
    }

    async fn update_recon_file_details(
        &self,
        _file_details: &ReconFileDetails,
    ) -> Result<ReconFileDetails, AppError> {
        unimplemented!();
    }

    async fn delete_recon_file_details(&self, file_id: &String) -> Result<bool, AppError> {
        // Create the client
        let connection_url = self.connection_url.clone();
        let store_name = self.store_name.clone();
        let client_connect_result =
            dapr::Client::<dapr::client::TonicClient>::connect(connection_url).await;

        let mut client;

        match client_connect_result {
            Ok(s) => client = s,
            Err(e) => return Err(AppError::new(AppErrorKind::ConnectionError, e.to_string())),
        }

        // delete a value from the state store
        let delete_result = client
            .delete_state(store_name, String::from(file_id), None)
            .await;

        match delete_result {
            Ok(_s) => return Ok(true),
            Err(e) => return Err(AppError::new(AppErrorKind::InternalError, e.to_string())),
        }
    }
}
