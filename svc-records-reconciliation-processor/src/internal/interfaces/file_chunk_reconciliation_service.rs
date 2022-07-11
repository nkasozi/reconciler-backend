use crate::internal::{
    entities::app_error::AppError,
    view_models::{
        reconcile_file_chunk_request::ReconcileFileChunkRequest,
        reconcile_file_chunk_response::ReconcileFileChunkResponse,
    },
};
use async_trait::async_trait;
use mockall::automock;

#[automock]
#[async_trait]
pub trait FileChunkReconciliationServiceInterface: Send + Sync {
    async fn reconcile_file_chunk(
        &self,
        primary_file_chunk: &ReconcileFileChunkRequest,
    ) -> Result<ReconcileFileChunkResponse, AppError>;
}
