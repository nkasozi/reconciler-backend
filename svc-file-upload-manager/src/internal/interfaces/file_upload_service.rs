use crate::internal::{
    entities::app_error::AppError, view_models::upload_file_chunk_request::UploadFileChunkRequest,
};
use async_trait::async_trait;
use mockall::automock;

#[automock]
#[async_trait]
pub trait FileUploadServiceInterface: Send + Sync {
    async fn upload_file_chunk(
        &self,
        file_upload_chunk: &UploadFileChunkRequest,
    ) -> Result<String, AppError>;
}
