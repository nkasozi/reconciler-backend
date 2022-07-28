use async_trait::async_trait;
use uuid::Uuid;
use validator::Validate;

use crate::internal::{
    interfaces::{
        recon_files_repository::ReconFileDetailsRepositoryInterface,
        recon_tasks_aggregator::ReconTaskAggregationServiceInterface,
        recon_tasks_repository::ReconTaskDetailsRepositoryInterface,
    },
    models::{
        entities::{
            app_errors::{AppError, AppErrorKind},
            recon_tasks_models::{ReconFileMetaData, ReconFileType, ReconTaskDetails},
        },
        view_models::{requests::CreateReconTaskRequest, responses::ReconTaskResponseDetails},
    },
};

const RECON_FILE_STORE_PREFIX: &'static str = "RECON-FILE";
const RECON_TASKS_STORE_PREFIX: &'static str = "RECON-TASK";

pub struct ReconTaskAggregationService {
    pub recon_task_details_repo: Box<dyn ReconTaskDetailsRepositoryInterface>,
    pub recon_file_details_repo: Box<dyn ReconFileDetailsRepositoryInterface>,
}

#[async_trait]
impl ReconTaskAggregationServiceInterface for ReconTaskAggregationService {
    async fn create_recon_task(
        &self,
        request: &CreateReconTaskRequest,
    ) -> Result<ReconTaskResponseDetails, AppError> {
        //validate request
        match request.validate() {
            Ok(_) => (),
            Err(e) => {
                return Err(AppError::new(
                    AppErrorKind::BadClientRequest,
                    e.to_string().replace("\n", " , "),
                ));
            }
        }
        //save primary file task_details
        let src_file_details = ReconTaskAggregationService::get_src_file_details(&request);

        let src_file_id = self
            .recon_file_details_repo
            .create_recon_file_details(&src_file_details)
            .await?;

        //save comparison file task_details
        let cmp_file_details = ReconTaskAggregationService::get_comparison_file_details(&request);

        let cmp_file_id = self
            .recon_file_details_repo
            .create_recon_file_details(&cmp_file_details)
            .await?;

        //save recon task details
        let recon_task_details = &ReconTaskAggregationService::get_recon_task_details(
            &src_file_id,
            &cmp_file_id,
            &request,
        );

        let task_id = self
            .recon_task_details_repo
            .create_task_details(&recon_task_details)
            .await?;

        //retrieve saved details
        return self.get_recon_task(&task_id).await;
    }

    async fn get_recon_task(&self, task_id: &String) -> Result<ReconTaskResponseDetails, AppError> {
        //validate request
        if task_id.is_empty() {
            return Err(AppError::new(
                AppErrorKind::BadClientRequest,
                String::from("please supply a taskID"),
            ));
        }

        //fetch details from repository
        let task_details = self
            .recon_task_details_repo
            .get_task_details(task_id)
            .await?;

        //convert details to view model
        let task_details_response: ReconTaskResponseDetails = task_details.into();

        //return success
        return Ok(task_details_response);
    }
}

impl ReconTaskAggregationService {
    fn get_src_file_details(request: &CreateReconTaskRequest) -> ReconFileMetaData {
        return ReconFileMetaData {
            id: ReconTaskAggregationService::generate_uuid(RECON_FILE_STORE_PREFIX),
            file_name: request.source_file_name.clone(),
            row_count: request.source_file_row_count,
            recon_file_type: ReconFileType::SourceReconFile,
            file_hash: request.source_file_hash.clone(),
            column_delimiters: request.source_file_delimiters.clone(),
            column_headers: request.source_file_headers.clone(),
        };
    }

    fn get_comparison_file_details(request: &CreateReconTaskRequest) -> ReconFileMetaData {
        return ReconFileMetaData {
            id: ReconTaskAggregationService::generate_uuid(RECON_FILE_STORE_PREFIX),
            file_name: request.comparison_file_name.clone(),
            row_count: request.comparison_file_row_count,
            recon_file_type: ReconFileType::ComparisonReconFile,
            file_hash: request.comparison_file_hash.clone(),
            column_delimiters: request.comparison_file_delimiters.clone(),
            column_headers: request.comparison_file_headers.clone(),
        };
    }

    fn get_recon_task_details(
        src_file_id: &String,
        cmp_file_id: &String,
        request: &CreateReconTaskRequest,
    ) -> ReconTaskDetails {
        return ReconTaskDetails {
            id: ReconTaskAggregationService::generate_uuid(RECON_TASKS_STORE_PREFIX),
            source_file_id: String::from(src_file_id),
            comparison_file_id: String::from(cmp_file_id),
            is_done: false,
            has_begun: true,
            comparison_pairs: request.comparison_pairs.clone(),
            recon_config: request.recon_configurations.clone(),
        };
    }

    fn generate_uuid(prefix: &str) -> String {
        let id = Uuid::new_v4().to_string();
        let full_id = String::from(format!("{}-{}", prefix, id));
        return full_id;
    }
}

#[cfg(test)]
mod tests {

    use crate::internal::{
        interfaces::{
            recon_files_repository::MockReconFileDetailsRepositoryInterface,
            recon_tasks_repository::MockReconTaskDetailsRepositoryInterface,
        },
        models::entities::recon_tasks_models::{ComparisonPair, ReconciliationConfigs},
    };

    use super::*;

    #[actix_web::test]
    async fn given_valid_create_recon_task_request_calls_correct_dependencies_returns_success() {
        //setup
        let mut mock_recon_task_details_repo =
            Box::new(MockReconTaskDetailsRepositoryInterface::new());

        mock_recon_task_details_repo
            .expect_create_task_details()
            .returning(|_y| Ok(String::from("task-1234")));

        mock_recon_task_details_repo
            .expect_get_task_details()
            .returning(|_y| {
                Ok(ReconTaskDetails {
                    id: String::from("task-1234"),
                    source_file_id: String::from("src-file-1234"),
                    comparison_file_id: String::from("cmp-file-1234"),
                    is_done: false,
                    has_begun: false,
                    comparison_pairs: vec![ComparisonPair {
                        source_column_index: 0,
                        comparison_column_index: 0,
                        is_row_identifier: true,
                    }],
                    recon_config: ReconciliationConfigs {
                        should_check_for_duplicate_records_in_comparison_file: true,
                        should_reconciliation_be_case_sensitive: true,
                        should_ignore_white_space: true,
                        should_do_reverse_reconciliation: true,
                    },
                })
            });

        let mut mock_recon_file_details_repo =
            Box::new(MockReconFileDetailsRepositoryInterface::new());

        mock_recon_file_details_repo
            .expect_create_recon_file_details()
            .returning(|_y| Ok(String::from("file-1234")));

        let service = ReconTaskAggregationService {
            recon_task_details_repo: mock_recon_task_details_repo,
            recon_file_details_repo: mock_recon_file_details_repo,
        };

        let test_request = CreateReconTaskRequest {
            user_id: String::from("test-user-id"),
            source_file_name: String::from("test-src-file"),
            source_file_hash: String::from("test-src-file-hash"),
            source_file_row_count: 1000,
            comparison_file_name: String::from("test-cmp-file"),
            comparison_file_hash: String::from("test-src-file-hash"),
            comparison_file_row_count: 10,
            recon_configurations: ReconciliationConfigs {
                should_check_for_duplicate_records_in_comparison_file: false,
                should_reconciliation_be_case_sensitive: true,
                should_ignore_white_space: true,
                should_do_reverse_reconciliation: false,
            },
            comparison_pairs: vec![ComparisonPair {
                source_column_index: 0,
                comparison_column_index: 0,
                is_row_identifier: true,
            }],
            source_file_headers: vec![String::from("src-file-header-1")],
            source_file_delimiters: vec![String::from(",")],
            comparison_file_headers: vec![String::from("cmp-file-header-1")],
            comparison_file_delimiters: vec![String::from(",")],
        };

        let expected = ReconTaskResponseDetails {
            task_id: String::from("task-1234"),
            is_done: false,
            has_begun: false,
        };

        //act
        let result = service.create_recon_task(&test_request).await;

        //assert
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(expected))
    }

    #[actix_web::test]
    async fn given_that_errors_occurs_when_handling_create_recon_task_request_returns_error() {
        //setup
        let mut mock_recon_task_details_repo =
            Box::new(MockReconTaskDetailsRepositoryInterface::new());

        mock_recon_task_details_repo
            .expect_create_task_details()
            .returning(|_y| Ok(String::from("task-1234")));

        mock_recon_task_details_repo
            .expect_get_task_details()
            .returning(|_y| {
                Ok(ReconTaskDetails {
                    id: String::from("task-1234"),
                    source_file_id: String::from("src-file-1234"),
                    comparison_file_id: String::from("cmp-file-1234"),
                    is_done: false,
                    has_begun: false,
                    comparison_pairs: vec![ComparisonPair {
                        source_column_index: 0,
                        comparison_column_index: 0,
                        is_row_identifier: true,
                    }],
                    recon_config: ReconciliationConfigs {
                        should_check_for_duplicate_records_in_comparison_file: true,
                        should_reconciliation_be_case_sensitive: true,
                        should_ignore_white_space: true,
                        should_do_reverse_reconciliation: true,
                    },
                })
            });

        let mut mock_recon_file_details_repo =
            Box::new(MockReconFileDetailsRepositoryInterface::new());

        mock_recon_file_details_repo
            .expect_create_recon_file_details()
            .returning(|_y| {
                Err(AppError::new(
                    AppErrorKind::ConnectionError,
                    "unable to connect".to_string(),
                ))
            });

        let service = ReconTaskAggregationService {
            recon_task_details_repo: mock_recon_task_details_repo,
            recon_file_details_repo: mock_recon_file_details_repo,
        };

        let test_request = CreateReconTaskRequest {
            user_id: String::from("test-user-id"),
            source_file_name: String::from("test-src-file"),
            source_file_hash: String::from("test-src-file-hash"),
            source_file_row_count: 1000,
            comparison_file_name: String::from("test-cmp-file"),
            comparison_file_hash: String::from("test-src-file-hash"),
            comparison_file_row_count: 10,
            recon_configurations: ReconciliationConfigs {
                should_check_for_duplicate_records_in_comparison_file: false,
                should_reconciliation_be_case_sensitive: true,
                should_ignore_white_space: true,
                should_do_reverse_reconciliation: false,
            },
            comparison_pairs: vec![],
            source_file_headers: vec![],
            source_file_delimiters: vec![],
            comparison_file_headers: vec![],
            comparison_file_delimiters: vec![],
        };

        //act
        let result = service.create_recon_task(&test_request).await;

        //assert
        assert!(result.is_err())
    }
}
