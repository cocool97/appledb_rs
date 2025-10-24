use anyhow::Result;
use appledb_common::{IPSWEntitlements, IPSWFrameworks};

#[async_trait::async_trait]
pub trait DataWriter {
    async fn post_executable_entitlements(&self, entitlements: IPSWEntitlements) -> Result<()>;

    async fn post_executable_frameworks(&self, frameworks: IPSWFrameworks) -> Result<()>;
}
