use std::{collections::BTreeMap, path::PathBuf, sync::Arc};

use appledb_common::api_models::TaskProgress;
use tokio::{sync::RwLock, task::JoinHandle};
use uuid::Uuid;

use crate::db_controller::DBController;

pub type RunningTasks = Arc<RwLock<BTreeMap<Uuid, (Arc<RwLock<TaskProgress>>, JoinHandle<()>)>>>;

pub struct AppState {
    pub db_controller: Arc<DBController>,
    pub web_sources_path: PathBuf,
    pub max_concurrent_tasks: usize,
    pub running_tasks: RunningTasks,
}
