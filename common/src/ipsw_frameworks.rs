use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::Platform;

#[derive(Deserialize, Serialize, ToSchema)]
pub struct IPSWFrameworks {
    /// Platform this IPSW originates from
    pub platform: Platform,
    /// Device model
    pub model_code: String,
    /// Software version of the platform
    pub version: String,
    /// Frameworks storage: `key=EXECUTABLE_FULLPATH`; `value=FRAMEWORKS`
    pub executable_frameworks: BTreeMap<String, Vec<String>>,
}

impl IPSWFrameworks {
    pub fn new(platform: Platform, model_code: &str, version: &str) -> Self {
        Self {
            platform,
            model_code: model_code.to_string(),
            version: version.to_string(),
            executable_frameworks: BTreeMap::new(),
        }
    }

    pub fn add_executable_frameworks<S: ToString>(
        &mut self,
        executable_fullpath: &S,
        frameworks: Vec<String>,
    ) {
        self.executable_frameworks
            .insert(executable_fullpath.to_string(), frameworks);
    }
}
