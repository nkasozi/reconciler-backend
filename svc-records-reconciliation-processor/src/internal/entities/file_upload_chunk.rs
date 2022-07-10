use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FileUploadChunk {
    pub id: String,

    pub upload_request_id: String,

    pub chunk_sequence_number: i64,

    pub chunk_source: FileUploadChunkSource,

    pub chunk_rows: Vec<String>,

    pub date_created: i64,

    pub date_modified: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum FileUploadChunkSource {
    ComparisonFileChunk,
    PrimaryFileChunk,
}
