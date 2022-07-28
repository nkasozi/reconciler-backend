use serde::{Deserialize, Serialize};

use crate::internal::models::entities::recon_tasks_models::ReconTaskDetails;

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
