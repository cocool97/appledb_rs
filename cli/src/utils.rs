use std::path::PathBuf;

use anyhow::{Result, bail};
use apple_codesign::MachOBinary;

use crate::data_writers::{DataWriter, LocalController, ServerController};

pub fn set_logger(debug: bool) {
    if std::env::var("RUST_LOG").is_err() {
        let log_level = if debug { "debug" } else { "info" };

        unsafe { std::env::set_var("RUST_LOG", log_level) };
    }

    env_logger::init();
}

pub fn parse_macho(data: &[u8]) -> Result<Option<MachOBinary<'_>>> {
    let macho = match MachOBinary::parse(data) {
        Ok(macho) => macho,
        Err(e) => return Err(e.into()),
    };

    if !macho.is_executable() {
        return Ok(None);
    }

    Ok(Some(macho))
}

pub async fn data_writer_from_ops(
    server_url: Option<String>,
    insecure: bool,
    output: Option<PathBuf>,
) -> Result<Box<dyn DataWriter>> {
    match (server_url, output) {
        (None, None) => bail!("neither --server-url nor --output flags specified..."),
        (None | Some(_), Some(output)) => Ok(Box::new(LocalController::new(output).await?)),
        (Some(server_url), None) => Ok(Box::new(ServerController::new(server_url, insecure)?)),
    }
}
