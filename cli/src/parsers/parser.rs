use std::path::Path;

use apple_codesign::MachOBinary;

use crate::data_writers::DataWriter;

pub trait IPSWParser {
    async fn parse_file<P: AsRef<Path>>(
        &mut self,
        full_absolute_path: P,
        macho: &MachOBinary<'_>,
    ) -> anyhow::Result<()>;

    async fn post_results(self, data_writer: &dyn DataWriter) -> anyhow::Result<()>;
}
