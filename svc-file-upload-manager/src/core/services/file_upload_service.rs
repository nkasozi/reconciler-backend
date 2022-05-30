use async_trait::async_trait;

use crate::core::{
    interfaces::{
        file_upload_repo::FileUploadRepositoryInterface,
        file_upload_service::FileUploadServiceInterface,
    },
    models::{app_error::AppError, file_upload_chunk::FileUploadChunk},
};

//const FILE_UPLOAD_REPO_PREFIX: &'static str = "FILE-CHUNK";

pub struct FileUploadService {
    pub file_upload_repo: Box<dyn FileUploadRepositoryInterface>,
}

#[async_trait]
impl FileUploadServiceInterface for FileUploadService {
    async fn upload_file_chunk(
        &self,
        _file_upload_chunk: &FileUploadChunk,
    ) -> Result<String, AppError> {
        unimplemented!()
    }
}
