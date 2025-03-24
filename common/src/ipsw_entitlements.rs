use std::collections::{BTreeMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::Platform;

#[derive(Debug, Serialize, Deserialize)]
pub struct IPSWEntitlements {
    /// Platform this IPSW originates from
    pub platform: Platform,
    /// Device model
    pub model_code: String,
    /// Software version of the platform
    pub version: String,
    /// Entitlements storage: key=EXECUTABLE_NAME; value=ENTITLEMENTS
    pub executable_entitlements: BTreeMap<String, HashSet<IPSWExecutableEntitlements>>,
}

impl IPSWEntitlements {
    pub fn new(platform: Platform, model_code: String, version: String) -> Self {
        Self {
            platform,
            version,
            model_code,
            executable_entitlements: BTreeMap::new(),
        }
    }

    pub fn add_executable_entitlements(
        &mut self,
        executable_name: String,
        entitlements: HashSet<IPSWExecutableEntitlements>,
    ) {
        self.executable_entitlements
            .insert(executable_name, entitlements);
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct IPSWExecutableEntitlements {
    pub key: String,
    pub value: String,
}
