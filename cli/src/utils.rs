use anyhow::Result;
use apple_codesign::MachOBinary;

pub fn set_logger(debug: bool) -> Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        let log_level = if debug { "debug" } else { "info" };

        unsafe { std::env::set_var("RUST_LOG", log_level) };
    }

    env_logger::init();

    Ok(())
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
