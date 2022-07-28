use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::internal::models::entities::recon_tasks_models::{
    ComparisonPair, ReconciliationConfigs,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct GetTaskDetailsRequest {
    pub task_id: String,
}

#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct CreateReconTaskRequest {
    #[validate(length(min = 1, message = "please supply a user_id"))]
    pub user_id: String,

    #[validate(length(min = 1, message = "please supply a source_file_name"))]
    pub source_file_name: String,

    #[validate(length(min = 1, message = "please supply a source_file_hash"))]
    pub source_file_hash: String,

    #[validate(range(min = 1, message = "please supply a source_file_row_count"))]
    pub source_file_row_count: u64,

    #[validate(length(min = 1, message = "please supply the source_file_headers"))]
    pub source_file_headers: Vec<String>,

    #[validate(length(min = 1, message = "please supply the source_file_delimiters"))]
    pub source_file_delimiters: Vec<String>,

    #[validate(length(min = 1, message = "please supply a comparison_file_name"))]
    pub comparison_file_name: String,

    #[validate(length(min = 1, message = "please supply a comparison_file_hash"))]
    pub comparison_file_hash: String,

    #[validate(length(min = 1, message = "please supply the comparison_file_headers"))]
    pub comparison_file_headers: Vec<String>,

    #[validate(range(min = 1, message = "please supply a comparison_file_row_count"))]
    pub comparison_file_row_count: u64,

    #[validate(length(min = 1, message = "please supply the comparison_file_delimiters"))]
    pub comparison_file_delimiters: Vec<String>,

    pub recon_configurations: ReconciliationConfigs,

    pub comparison_pairs: Vec<ComparisonPair>,
}
