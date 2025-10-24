use anyhow::Result;
use appledb_common::{IPSWEntitlements, IPSWFrameworks};
use std::path::PathBuf;
use tokio::fs::{File, create_dir_all};

use crate::data_writers::DataWriter;

pub struct LocalController {
    output_dir: PathBuf,
}

impl LocalController {
    pub async fn new(output_dir: PathBuf) -> Result<Self> {
        if !output_dir.exists() {
            create_dir_all(&output_dir).await?;
            log::info!("have created output directory {}", output_dir.display());
        }

        Ok(Self { output_dir })
    }
}

#[async_trait::async_trait]
impl DataWriter for LocalController {
    async fn post_executable_entitlements(
        &self,
        entitlements: IPSWEntitlements,
    ) -> anyhow::Result<()> {
        let file = File::create(&self.output_dir.join(format!(
            "{}_{}_{}.entitlements",
            entitlements.model_code, entitlements.platform, entitlements.version
        )))
        .await?;

        serde_json::to_writer(&mut file.into_std().await, &entitlements)?;

        Ok(())
    }

    async fn post_executable_frameworks(&self, frameworks: IPSWFrameworks) -> anyhow::Result<()> {
        let file = File::create(&self.output_dir.join(format!(
            "{}_{}_{}.frameworks",
            frameworks.model_code, frameworks.platform, frameworks.version
        )))
        .await?;

        serde_json::to_writer(&mut file.into_std().await, &frameworks)?;

        Ok(())
    }
}
