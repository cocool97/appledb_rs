use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Deserialize, Serialize, ToSchema)]
pub enum TaskType {
    #[serde(rename = "entitlements")]
    PostEntitlements,
    #[serde(rename = "frameworks")]
    PostFrameworks,
}

#[derive(Clone, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum TaskSource {
    Local,
    Api,
}

#[derive(Clone, Deserialize, Serialize, ToSchema)]
pub struct TaskProgress {
    task_type: TaskType,
    task_source: TaskSource,
    start_time: u64,
    done: u64,
    total: u64,
}

impl TaskProgress {
    pub fn new(task_type: TaskType, task_source: TaskSource, total: u64) -> Self {
        Self {
            task_type,
            task_source,
            start_time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            done: 0,
            total,
        }
    }

    pub fn increment_done(&mut self) {
        self.done += 1;
    }

    pub fn done(&self) -> u64 {
        self.done
    }

    pub fn total(&self) -> u64 {
        self.total
    }
}
