use std::{
    fmt,
    io::{Error, ErrorKind},
};

use super::entities::ReconTaskDetails;
use serde::{Deserialize, Serialize};
use validator::Validate;

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
    #[validate(range(min = 1, message = "please supply a source_file_column_count"))]
    pub source_file_column_count: u64,
    #[validate(length(min = 1, message = "please supply a comparison_file_name"))]
    pub comparison_file_name: String,
    #[validate(length(min = 1, message = "please supply a comparison_file_hash"))]
    pub comparison_file_hash: String,
    #[validate(range(min = 1, message = "please supply a comparison_file_column_count"))]
    pub comparison_file_column_count: u64,
    #[validate(range(min = 1, message = "please supply a comparison_file_row_count"))]
    pub comparison_file_row_count: u64,
    #[validate]
    pub recon_configurations: ReconciliationConfigs,
    pub comparison_pairs: Vec<ComparisonPair>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ComparisonPair {
    pub source_column_index: u64,
    pub comparison_column_index: u64,
    pub is_record_id: bool,
}

#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct ReconciliationConfigs {
    pub should_check_for_duplicate_records_in_comparison_file: bool,
    pub should_reconciliation_be_case_sensitive: bool,
    pub should_ignore_white_space: bool,
    pub should_do_reverse_reconciliation: bool,
}

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct ReconTaskResponseDetails {
    pub task_id: String,
    pub is_done: bool,
    pub has_begun: bool,
}

impl From<ReconTaskDetails> for ReconTaskResponseDetails {
    fn from(details: ReconTaskDetails) -> Self {
        return Self {
            task_id: details.id,
            is_done: details.is_done,
            has_begun: details.has_begun,
        };
    }
}

impl From<AppError> for Error {
    fn from(details: AppError) -> Self {
        return Error::new(ErrorKind::Other, details.message);
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AppErrorKind {
    NotFound,
    InternalError,
    ConnectionError,
    ResponseUnmarshalError,
    BadClientRequest,
}

impl AppErrorKind {
    pub fn as_str(&self) -> &'static str {
        use AppErrorKind::*;
        // Strictly alphabetical, please.  (Sadly rustfmt cannot do this yet.)
        match *self {
            NotFound => "TaskNotFound",
            InternalError => "InternalError",
            ConnectionError => "ConnectionError",
            ResponseUnmarshalError => "ResponseUnmarshalError",
            BadClientRequest => "BadClientRequest",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AppError {
    pub kind: AppErrorKind,
    pub message: String,
}

impl AppError {
    pub fn new(kind: AppErrorKind, message: String) -> AppError {
        AppError { kind, message }
    }
}

// Different error messages according to AppError.code
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} - [{}]", self.kind.as_str(), self.message)
    }
}
