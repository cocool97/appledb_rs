use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Deserialize, Serialize, ToSchema)]
pub enum TaskType {
    PostEntitlements,
    PostFrameworks,
}

#[derive(Clone, Deserialize, Serialize, ToSchema)]
pub enum TaskSource {
    Local,
    Api,
}

#[derive(Clone, Deserialize, Serialize, ToSchema)]
pub struct TaskProgress {
    task_type: TaskType,
    task_source: TaskSource,
    start_time: u64,
    pub done: usize,
    pub total: usize,
}

impl TaskProgress {
    pub fn new(task_type: TaskType, task_source: TaskSource, total: usize) -> Self {
        Self {
            task_type,
            task_source,
            start_time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or(Duration::from_secs(0))
                .as_secs(),
            done: 0,
            total,
        }
    }
}
