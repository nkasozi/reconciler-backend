use crate::internal::{
    interfaces::{
        file_chunk_upload_service::FileChunkUploadServiceInterface,
        pubsub_repo::PubSubRepositoryInterface, recon_tasks_repo::ReconTasksRepositoryInterface,
    },
    models::{
        entities::{
            app_error::{AppError, AppErrorKind},
            file_upload_chunk::{
                FileUploadChunk, FileUploadChunkRow, FileUploadChunkSource, ReconStatus,
            },
            recon_task::ReconFileMetaData,
        },
        view_models::{
            upload_file_chunk_request::UploadFileChunkRequest,
            upload_file_chunk_response::UploadFileChunkResponse,
        },
    },
};
use async_trait::async_trait;
use uuid::Uuid;
use validator::Validate;

const FILE_CHUNK_PREFIX: &'static str = "FILE-CHUNK";

pub struct FileChunkUploadService {
    pub file_upload_repo: Box<dyn PubSubRepositoryInterface>,
    pub recon_tasks_repo: Box<dyn ReconTasksRepositoryInterface>,
}

#[async_trait]
impl FileChunkUploadServiceInterface for FileChunkUploadService {
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

        //get recon file metadata
        let recon_file_metadata = self
            .recon_tasks_repo
            .get_recon_task_details(&upload_file_chunk_request.upload_request_id)
            .await?;

        //transform into the repo model
        let file_upload_chunk =
            self.transform_into_file_upload_chunk(upload_file_chunk_request, &recon_file_metadata);

        let file_save_result;

        //save it to the repository
        match file_upload_chunk.chunk_source {
            FileUploadChunkSource::ComparisonFileChunk => {
                file_save_result = self
                    .file_upload_repo
                    .save_file_upload_chunk_to_comparison_file_queue(&file_upload_chunk)
                    .await;
            }

            FileUploadChunkSource::PrimaryFileChunk => {
                file_save_result = self
                    .file_upload_repo
                    .save_file_upload_chunk_to_primary_file_queue(&file_upload_chunk)
                    .await;
            }
        }

        match file_save_result {
            Ok(_) => Ok(UploadFileChunkResponse {
                file_chunk_id: file_upload_chunk.id,
            }),
            Err(e) => Err(e),
        }
    }
}

impl FileChunkUploadService {
    fn transform_into_file_upload_chunk(
        &self,
        upload_file_chunk_request: &UploadFileChunkRequest,
        original_recon_file_task: &ReconFileMetaData,
    ) -> FileUploadChunk {
        FileUploadChunk {
            id: self.generate_uuid(FILE_CHUNK_PREFIX),
            upload_request_id: upload_file_chunk_request.upload_request_id.clone(),
            chunk_sequence_number: upload_file_chunk_request.chunk_sequence_number.clone(),
            chunk_source: upload_file_chunk_request.chunk_source.clone(),
            chunk_rows: self
                .transform_into_chunk_rows(&upload_file_chunk_request, original_recon_file_task),
            date_created: chrono::Utc::now().timestamp(),
            date_modified: chrono::Utc::now().timestamp(),
        }
    }

    fn transform_into_chunk_rows(
        &self,
        upload_file_chunk_request: &UploadFileChunkRequest,
        recon_file_meta_data: &ReconFileMetaData,
    ) -> Vec<FileUploadChunkRow> {
        let parsed_chunk_rows: Vec<FileUploadChunkRow> = vec![];

        let row_index = 1;

        for upload_file_row in upload_file_chunk_request.chunk_rows.clone() {
            let parsed_chunk_row = FileUploadChunkRow {
                raw_data: upload_file_row,
                parsed_columns_from_row: vec![],
                recon_result: ReconStatus::Pending,
                recon_result_reasons: vec![],
            };

            let upload_file_columns_in_row: Vec<String> = vec![];

            for delimiter in recon_file_meta_data.column_delimiter {
                let row_parts: Vec<String> = upload_file_row
                    .split(&delimiter)
                    .map(str::to_owned)
                    .collect();

                upload_file_columns_in_row.extend(row_parts);
            }

            for comparison_pair in recon_file_meta_data.recon_task_details.comparison_pairs {
                match upload_file_chunk_request.chunk_source {
                    FileUploadChunkSource::ComparisonFileChunk => {
                        if comparison_pair.comparison_column_index
                            > upload_file_columns_in_row.len()
                        {
                            //skip this row because the columns we have parsed are not enough
                            let reason = format!(
                                "cant find a value in column {} of comparison file for this row {}",
                                comparison_pair.comparison_column_index, row_index
                            );
                            parsed_chunk_row.recon_result = ReconStatus::Failed;
                            parsed_chunk_row.recon_result_reasons.push(reason);
                            continue;
                        }

                        //otherwise add new row to those that have been parsed
                        let row_column_value =
                            upload_file_columns_in_row[comparison_pair.comparison_column_index];

                        parsed_chunk_row
                            .parsed_columns_from_row
                            .push(row_column_value.clone());

                        continue;
                    }

                    FileUploadChunkSource::PrimaryFileChunk => {
                        if comparison_pair.source_column_index > upload_file_columns_in_row.len() {
                            //skip this row because the columns we have parsed are not enough
                            let reason = format!(
                                "cant find a value in column {} of source file for this row {}",
                                comparison_pair.source_column_index, row_index
                            );
                            parsed_chunk_row.recon_result = ReconStatus::Failed;
                            parsed_chunk_row.recon_result_reasons.push(reason);
                            continue;
                        }

                        //otherwise add new row column value to those that have been parsed
                        let row_column_value =
                            upload_file_columns_in_row[comparison_pair.source_column_index];

                        parsed_chunk_row
                            .parsed_columns_from_row
                            .push(row_column_value.clone());

                        continue;
                    }
                }
            }

            row_index = row_index + 1;
        }

        return parsed_chunk_rows;
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
        interfaces::{
            file_chunk_upload_service::FileChunkUploadServiceInterface,
            pubsub_repo::MockPubSubRepositoryInterface,
            recon_tasks_repo::MockReconTasksRepositoryInterface,
        },
        models::{
            entities::app_error::{AppError, AppErrorKind},
            view_models::upload_file_chunk_request::UploadFileChunkRequest,
        },
    };

    use crate::internal::models::entities::file_upload_chunk::FileUploadChunkSource;

    use super::FileChunkUploadService;

    #[actix_rt::test]
    async fn given_valid_request_calls_correct_dependencie_and_returns_success() {
        let mut mock_file_upload_repo = Box::new(MockPubSubRepositoryInterface::new());
        let mut mock_recon_tasks_repo = Box::new(MockReconTasksRepositoryInterface::new());

        mock_file_upload_repo
            .expect_save_file_upload_chunk_to_comparison_file_queue()
            .returning(|_y| Ok(String::from("FILE_CHUNK_1234")));

        let sut = FileChunkUploadService {
            file_upload_repo: mock_file_upload_repo,
            recon_tasks_repo: mock_recon_tasks_repo,
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
        let mut mock_file_upload_repo = Box::new(MockPubSubRepositoryInterface::new());
        let mut mock_recon_tasks_repo = Box::new(MockReconTasksRepositoryInterface::new());

        mock_file_upload_repo
            .expect_save_file_upload_chunk_to_comparison_file_queue()
            .returning(|_y| Ok(String::from("FILE_CHUNK_1234")));

        let sut = FileChunkUploadService {
            file_upload_repo: mock_file_upload_repo,
            recon_tasks_repo: mock_recon_tasks_repo,
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
        let mut mock_file_upload_repo = Box::new(MockPubSubRepositoryInterface::new());
        let mut mock_recon_tasks_repo = Box::new(MockReconTasksRepositoryInterface::new());

        mock_file_upload_repo
            .expect_save_file_upload_chunk_to_comparison_file_queue()
            .returning(|_y| {
                Err(AppError::new(
                    AppErrorKind::ConnectionError,
                    "unable to connect".to_string(),
                ))
            });

        let sut = FileChunkUploadService {
            file_upload_repo: mock_file_upload_repo,
            recon_tasks_repo: mock_recon_tasks_repo,
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
