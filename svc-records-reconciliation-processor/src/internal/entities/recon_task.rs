use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ReconTaskDetails {
    pub id: String,
    pub source_file_id: String,
    pub comparison_file_id: String,
    pub is_done: bool,
    pub has_begun: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReconFileDetails {
    pub id: String,
    pub file_name: String,
    pub file_size: u64,
    pub row_count: u64,
    pub column_count: u64,
    pub recon_file_type: ReconFileType,
    pub file_hash: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ReconFileType {
    SourceReconFile,
    ComparisonReconFile,
}
