use async_trait::async_trait;
use mockall::automock;

use crate::core::models::{app_error::AppError, file_upload_chunk::FileUploadChunk};

#[automock]
#[async_trait]
pub trait FileUploadRepositoryInterface: Send + Sync {
    async fn save_file_upload_chunk(
        &self,
        file_upload_chunk: &FileUploadChunk,
    ) -> Result<String, AppError>;
}
