use async_trait::async_trait;
use mockall::automock;

use crate::internal::models::entities::{
    app_errors::AppError, recon_tasks_models::ReconFileMetaData,
};

#[automock]
#[async_trait]
pub trait ReconFileDetailsRepositoryInterface: Send + Sync {
    fn get_connection_string(&self) -> String;
    fn get_store_name(&self) -> String;
    async fn get_recon_file_details(&self, file_id: &String)
        -> Result<ReconFileMetaData, AppError>;
    async fn create_recon_file_details(
        &self,
        file_details: &ReconFileMetaData,
    ) -> Result<String, AppError>;
    async fn update_recon_file_details(
        &self,
        file_details: &ReconFileMetaData,
    ) -> Result<ReconFileMetaData, AppError>;
    async fn delete_recon_file_details(&self, file_id: &String) -> Result<bool, AppError>;
}
