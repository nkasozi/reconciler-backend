use async_trait::async_trait;
use mockall::automock;

use crate::internal::models::entities::{
    app_errors::AppError, recon_tasks_models::ReconTaskDetails,
};

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
