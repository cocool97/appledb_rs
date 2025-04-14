use std::{io::Cursor, path::Path};

use apple_codesign::MachOBinary;
use appledb_common::{IPSWEntitlements, Platform};

use crate::server_controller::ServerController;

use super::parser::IPSWParser;

pub struct EntitlementsParser {
    ipsw_entitlements: IPSWEntitlements,
}

impl EntitlementsParser {
    pub fn new(platform: Platform, model_code: &str, version: &str) -> Self {
        Self {
            ipsw_entitlements: IPSWEntitlements::new(platform, model_code, version),
        }
    }
}

impl IPSWParser for EntitlementsParser {
    async fn parse_file<P: AsRef<Path>>(
        &mut self,
        full_absolute_path: P,
        macho: &MachOBinary<'_>,
    ) -> anyhow::Result<()> {
        if let Some(code_signature) = macho.code_signature()? {
            if let Some(entitlements) = code_signature.entitlements()? {
                let plist_value = plist::Value::from_reader(Cursor::new(entitlements.as_str()))?;
                let json_value = serde_json::to_value(plist_value)?;

                self.ipsw_entitlements.add_executable_entitlements(
                    full_absolute_path.as_ref().to_string_lossy(),
                    json_value,
                );
            }
        }

        Ok(())
    }

    async fn post_results(self, server_controller: &ServerController) -> anyhow::Result<String> {
        log::info!("Sending entitlements to server...");
        server_controller
            .post_executable_entitlements(self.ipsw_entitlements)
            .await
    }
}
