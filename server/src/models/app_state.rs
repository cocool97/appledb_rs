use std::path::PathBuf;

use crate::db_controller::DBController;

pub struct AppState {
    pub db_controller: DBController,
    pub web_sources_path: PathBuf,
}
