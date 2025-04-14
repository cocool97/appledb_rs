use appledb_common::{IPSWFrameworks, Platform};

use super::IPSWParser;

const OWN_FRAMEWORK_NAME: &str = "self";

pub struct FrameworksParser {
    ipsw_frameworks: IPSWFrameworks,
}

impl FrameworksParser {
    pub fn new(platform: Platform, model_code: &str, version: &str) -> Self {
        Self {
            ipsw_frameworks: IPSWFrameworks::new(platform, model_code, version),
        }
    }
}

impl IPSWParser for FrameworksParser {
    async fn parse_file<P: AsRef<std::path::Path>>(
        &mut self,
        full_absolute_path: P,
        macho: &apple_codesign::MachOBinary<'_>,
    ) -> anyhow::Result<()> {
        self.ipsw_frameworks.add_executable_frameworks(
            full_absolute_path.as_ref().to_string_lossy().to_string(),
            macho
                .macho
                .libs
                .iter()
                .filter_map(|v| {
                    if *v == OWN_FRAMEWORK_NAME {
                        // Filter out "own" framework name
                        return None;
                    }

                    Some(v.to_string())
                })
                .collect(),
        );

        Ok(())
    }

    async fn post_results(
        self,
        server_controller: &crate::server_controller::ServerController,
    ) -> anyhow::Result<String> {
        log::info!("Sending frameworks to server...");
        server_controller
            .post_executable_frameworks(self.ipsw_frameworks)
            .await
    }
}
