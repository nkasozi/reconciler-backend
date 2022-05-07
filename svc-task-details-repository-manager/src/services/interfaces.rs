use async_trait::async_trait;
use mockall::automock;

use super::{
    entities::{ReconFileDetails, ReconTaskDetails},
    view_models::{AppError, CreateReconTaskRequest, ReconTaskResponseDetails},
};

#[automock]
#[async_trait]
pub trait ReconFileDetailsRepositoryInterface: Send + Sync {
    fn get_connection_string(&self) -> String;
    fn get_store_name(&self) -> String;
    async fn get_recon_file_details(&self, file_id: &String) -> Result<ReconFileDetails, AppError>;
    async fn create_recon_file_details(
        &self,
        file_details: &ReconFileDetails,
    ) -> Result<String, AppError>;
    async fn update_recon_file_details(
        &self,
        file_details: &ReconFileDetails,
    ) -> Result<ReconFileDetails, AppError>;
    async fn delete_recon_file_details(&self, file_id: &String) -> Result<bool, AppError>;
}

#[automock]
#[async_trait]
pub trait ReconTaskDetailsRepositoryInterface: Send + Sync {
    async fn get_task_details(&self, task_id: &String) -> Result<ReconTaskDetails, AppError>;
    async fn create_task_details(
        &self,
        task_details: &ReconTaskDetails,
    ) -> Result<String, AppError>;
    async fn update_task_details(
        &self,
        task_details: &ReconTaskDetails,
    ) -> Result<ReconTaskDetails, AppError>;
    async fn delete_task_details(&self, task_id: &String) -> Result<bool, AppError>;
}

#[automock]
#[async_trait]
pub trait ReconTaskAggregationServiceInterface {
    async fn create_recon_task(
        &self,
        request: &CreateReconTaskRequest,
    ) -> Result<ReconTaskResponseDetails, AppError>;

    async fn get_recon_task(&self, task_id: &String) -> Result<ReconTaskResponseDetails, AppError>;
}
