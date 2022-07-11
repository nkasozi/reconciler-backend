use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ReconcileFileChunkResponse {
    pub file_chunk_id: String,
}
