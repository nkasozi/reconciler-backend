use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FileUploadChunk {
    pub id: String,
    pub upload_request_id: String,
    pub chunk_sequence_number: i64,
    pub chunk_type: FileUploadChunkType,
    pub date_created: String,
    pub date_modified: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum FileUploadChunkType {
    SrcFileChunk,
    PrimaryFileChunk,
}
