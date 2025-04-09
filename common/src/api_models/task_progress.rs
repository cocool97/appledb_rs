use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Deserialize, Serialize, ToSchema)]
pub struct TaskProgress {
    pub done: usize,
    pub total: usize,
}

impl TaskProgress {
    pub fn new(total: usize) -> Self {
        Self { done: 0, total }
    }
}
