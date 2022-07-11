use crate::internal::entities::{app_error::AppError, file_upload_chunk::FileUploadChunk};
use async_trait::async_trait;
use mockall::automock;

#[automock]
#[async_trait]
pub trait PubSubRepositoryInterface: Send + Sync {
    async fn get_next_comparison_file_upload_chunk(&self) -> Result<FileUploadChunk, AppError>;
}
