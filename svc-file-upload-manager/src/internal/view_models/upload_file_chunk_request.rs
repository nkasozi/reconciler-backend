use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::internal::models::file_upload_chunk::FileUploadChunkSource;

#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct UploadFileChunkRequest {
    #[validate(length(min = 1, message = "please supply an upload_request_id"))]
    pub upload_request_id: String,

    #[validate(range(min = 1))]
    pub chunk_sequence_number: i64,

    pub chunk_source: FileUploadChunkSource,

    #[validate(length(min = 1, message = "please supply the chunk raw data"))]
    pub chunk_raw_data: String,
}
