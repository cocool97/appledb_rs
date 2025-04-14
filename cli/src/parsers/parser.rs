use std::path::Path;

use apple_codesign::MachOBinary;

use crate::server_controller::ServerController;

pub trait IPSWParser {
    async fn parse_file<P: AsRef<Path>>(
        &mut self,
        full_absolute_path: P,
        macho: &MachOBinary<'_>,
    ) -> anyhow::Result<()>;

    async fn post_results(self, server_controller: &ServerController) -> anyhow::Result<String>;
}
