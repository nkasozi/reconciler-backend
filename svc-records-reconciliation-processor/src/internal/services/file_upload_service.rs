use crate::internal::{
    entities::{
        app_error::{AppError, AppErrorKind},
        file_upload_chunk::FileUploadChunk,
    },
    interfaces::{
        file_upload_repo::FileUploadRepositoryInterface,
        file_upload_service::FileUploadServiceInterface,
    },
    view_models::{
        upload_file_chunk_request::UploadFileChunkRequest,
        upload_file_chunk_response::UploadFileChunkResponse,
    },
};
use async_trait::async_trait;
use uuid::Uuid;
use validator::Validate;

const FILE_CHUNK_PREFIX: &'static str = "FILE-CHUNK";

pub struct FileUploadService {
    pub file_upload_repo: Box<dyn FileUploadRepositoryInterface>,
}

#[async_trait]
impl FileUploadServiceInterface for FileUploadService {
    /**
    uploads a file chunk to the repository

    # Errors

    This function will return an error if the request fails validation or fails to be uploaded.
    */
    async fn upload_file_chunk(
        &self,
        upload_file_chunk_request: &UploadFileChunkRequest,
    ) -> Result<UploadFileChunkResponse, AppError> {
        //validate request
        match upload_file_chunk_request.validate() {
            Ok(_) => (),
            Err(e) => {
                return Err(AppError::new(
                    AppErrorKind::BadClientRequest,
                    e.to_string().replace("\n", " , "),
                ));
            }
        }

        //transform into the repo model
        let file_upload_chunk = self.transform_into_file_upload_chunk(upload_file_chunk_request);

        //save it to the repository
        let file_save_result = self
            .file_upload_repo
            .save_file_upload_chunk(&file_upload_chunk)
            .await;

        match file_save_result {
            Ok(file_chunk_id) => Ok(UploadFileChunkResponse { file_chunk_id }),
            Err(e) => Err(e),
        }
    }
}

impl FileUploadService {
    fn transform_into_file_upload_chunk(
        &self,
        upload_file_chunk_request: &UploadFileChunkRequest,
    ) -> FileUploadChunk {
        FileUploadChunk {
            id: self.generate_uuid(FILE_CHUNK_PREFIX),
            upload_request_id: upload_file_chunk_request.upload_request_id.clone(),
            chunk_sequence_number: upload_file_chunk_request.chunk_sequence_number.clone(),
            chunk_source: upload_file_chunk_request.chunk_source.clone(),
            chunk_rows: upload_file_chunk_request.chunk_rows.clone(),
            date_created: chrono::Utc::now().timestamp(),
            date_modified: chrono::Utc::now().timestamp(),
        }
    }

    fn generate_uuid(&self, prefix: &str) -> String {
        let id = Uuid::new_v4().to_string();
        let full_id = String::from(format!("{}-{}", prefix, id));
        return full_id;
    }
}

#[cfg(test)]
mod tests {
    use crate::internal::{
        entities::app_error::{AppError, AppErrorKind},
        interfaces::{
            file_upload_repo::MockFileUploadRepositoryInterface,
            file_upload_service::FileUploadServiceInterface,
        },
        view_models::upload_file_chunk_request::UploadFileChunkRequest,
    };

    use crate::internal::entities::file_upload_chunk::FileUploadChunkSource;

    use super::FileUploadService;

    #[actix_rt::test]
    async fn given_valid_request_calls_correct_dependencie_and_returns_success() {
        let mut mock_file_upload_repo = Box::new(MockFileUploadRepositoryInterface::new());

        mock_file_upload_repo
            .expect_save_file_upload_chunk()
            .returning(|_y| Ok(String::from("FILE_CHUNK_1234")));

        let sut = FileUploadService {
            file_upload_repo: mock_file_upload_repo,
        };

        let test_request = UploadFileChunkRequest {
            upload_request_id: String::from("1234"),
            chunk_sequence_number: 2,
            chunk_source: FileUploadChunkSource::ComparisonFileChunk,
            chunk_rows: vec![String::from("testing, 1234")],
        };

        let actual = sut.upload_file_chunk(&test_request).await;

        assert!(actual.is_ok());
    }

    #[actix_rt::test]
    async fn given_invalid_request_returns_error() {
        let mut mock_file_upload_repo = Box::new(MockFileUploadRepositoryInterface::new());

        mock_file_upload_repo
            .expect_save_file_upload_chunk()
            .returning(|_y| Ok(String::from("FILE_CHUNK_1234")));

        let sut = FileUploadService {
            file_upload_repo: mock_file_upload_repo,
        };

        let test_request = UploadFileChunkRequest {
            upload_request_id: String::from("1234"),
            chunk_sequence_number: 0,
            chunk_source: FileUploadChunkSource::ComparisonFileChunk,
            chunk_rows: vec![String::from("testing, 1234")],
        };

        let actual = sut.upload_file_chunk(&test_request).await;

        assert!(actual.is_err());
    }

    #[actix_rt::test]
    async fn given_valid_request_but_repo_returns_error_returns_error() {
        let mut mock_file_upload_repo = Box::new(MockFileUploadRepositoryInterface::new());

        mock_file_upload_repo
            .expect_save_file_upload_chunk()
            .returning(|_y| {
                Err(AppError::new(
                    AppErrorKind::ConnectionError,
                    "unable to connect".to_string(),
                ))
            });

        let sut = FileUploadService {
            file_upload_repo: mock_file_upload_repo,
        };

        let test_request = UploadFileChunkRequest {
            upload_request_id: String::from("1234"),
            chunk_sequence_number: 2,
            chunk_source: FileUploadChunkSource::ComparisonFileChunk,
            chunk_rows: vec![String::from("testing, 1234")],
        };

        let actual = sut.upload_file_chunk(&test_request).await;

        assert!(actual.is_err());
    }
}
