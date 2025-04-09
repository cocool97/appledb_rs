use std::{collections::HashMap, path::PathBuf, sync::Arc};

use tokio::{sync::RwLock, task::JoinHandle};
use uuid::Uuid;

use crate::db_controller::DBController;

pub struct AppState {
    pub db_controller: Arc<DBController>,
    pub web_sources_path: PathBuf,
    pub max_concurrent_tasks: usize,
    pub running_entitlements_tasks: Arc<RwLock<HashMap<Uuid, JoinHandle<()>>>>,
}
