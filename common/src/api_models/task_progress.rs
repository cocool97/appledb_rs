use std::{
    fmt::Display,
    time::{SystemTime, UNIX_EPOCH},
};

use serde::{Deserialize, Serialize};
use tokio::sync::OwnedSemaphorePermit;
use utoipa::ToSchema;

#[derive(Clone, Deserialize, Serialize, ToSchema)]
pub enum TaskType {
    #[serde(rename = "entitlements")]
    PostEntitlements,
    #[serde(rename = "frameworks")]
    PostFrameworks,
}

pub enum TaskSource {
    Local(OwnedSemaphorePermit),
    Api,
}

impl Display for TaskSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskSource::Local(_) => write!(f, "local"),
            TaskSource::Api => write!(f, "api"),
        }
    }
}

#[derive(Clone, Deserialize, Serialize, ToSchema)]
pub struct TaskProgress {
    task_type: TaskType,
    task_source: String,
    start_time: u64,
    done: u64,
    total: u64,
}

impl TaskProgress {
    pub fn new(task_type: TaskType, task_source: String, total: u64) -> Self {
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
