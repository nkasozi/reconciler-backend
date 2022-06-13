use serde::{Deserialize, Serialize};

use crate::core::models::file_upload_chunk::FileUploadChunkType;

#[derive(Serialize, Deserialize, Debug)]
pub struct UploadFileChunkRequest {
    pub id: String,
    pub upload_request_id: String,
    pub chunk_sequence_number: i64,
    pub chunk_type: FileUploadChunkType,
}
