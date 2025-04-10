use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Deserialize, Serialize, ToSchema)]
pub enum TaskType {
    PostEntitlements,
}

#[derive(Clone, Deserialize, Serialize, ToSchema)]
pub struct TaskProgress {
    task_type: TaskType,
    start_time: u64,
    pub done: usize,
    pub total: usize,
}

impl TaskProgress {
    pub fn new(task_type: TaskType, total: usize) -> Self {
        Self {
            task_type,
            start_time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or(Duration::from_secs(0))
                .as_secs(),
            done: 0,
            total,
        }
    }
}
