use async_trait::async_trait;
use uuid::Uuid;

use crate::core::{
    interfaces::{
        file_upload_repo::FileUploadRepositoryInterface,
        file_upload_service::FileUploadServiceInterface,
    },
    models::{app_error::AppError, file_upload_chunk::FileUploadChunk},
    view_models::upload_file_chunk_request::UploadFileChunkRequest,
};

const FILE_CHUNK_PREFIX: &'static str = "FILE-CHUNK";

pub struct FileUploadService {
    pub file_upload_repo: Box<dyn FileUploadRepositoryInterface>,
}

#[async_trait]
impl FileUploadServiceInterface for FileUploadService {
    async fn upload_file_chunk(
        &self,
        upload_file_chunk_request: &UploadFileChunkRequest,
    ) -> Result<String, AppError> {
        let file_upload_chunk = self.get_file_upload_chunk(upload_file_chunk_request);

        return self
            .file_upload_repo
            .save_file_upload_chunk(&file_upload_chunk)
            .await;
    }
}

impl FileUploadService {
    fn get_file_upload_chunk(
        &self,
        upload_file_chunk_request: &UploadFileChunkRequest,
    ) -> FileUploadChunk {
        FileUploadChunk {
            id: self.generate_uuid(FILE_CHUNK_PREFIX),
            upload_request_id: upload_file_chunk_request.upload_request_id.clone(),
            chunk_sequence_number: upload_file_chunk_request.chunk_sequence_number.clone(),
            chunk_type: upload_file_chunk_request.chunk_type.clone(),
            date_created: String::from(""),
            date_modified: String::from(""),
        }
    }

    fn generate_uuid(&self, prefix: &str) -> String {
        let id = Uuid::new_v4().to_string();
        let full_id = String::from(format!("{}-{}", prefix, id));
        return full_id;
    }
}
