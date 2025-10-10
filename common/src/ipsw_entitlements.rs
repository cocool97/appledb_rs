use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::Platform;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct IPSWEntitlements {
    /// Platform this IPSW originates from
    pub platform: Platform,
    /// Device model
    pub model_code: String,
    /// Software version of the platform
    pub version: String,
    /// Entitlements storage: `key=EXECUTABLE_FULLPATH`; `value=ENTITLEMENTS`
    pub executable_entitlements: BTreeMap<String, serde_json::Value>,
}

impl IPSWEntitlements {
    pub fn new(platform: Platform, model_code: &str, version: &str) -> Self {
        Self {
            platform,
            version: version.to_string(),
            model_code: model_code.to_string(),
            executable_entitlements: BTreeMap::new(),
        }
    }

    pub fn add_executable_entitlements<S: ToString>(
        &mut self,
        executable_fullpath: &S,
        entitlements: serde_json::Value,
    ) {
        self.executable_entitlements
            .insert(executable_fullpath.to_string(), entitlements);
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct IPSWExecutableEntitlements {
    pub key: String,
    pub value: String,
}
