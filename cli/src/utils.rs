use anyhow::Result;
use env_logger::{Builder, Target};
use log::LevelFilter;

pub fn set_logger(debug: bool) -> Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        let log_level = if debug {
            LevelFilter::Debug
        } else {
            LevelFilter::Info
        };

        Builder::new()
            .filter(None, log_level)
            .target(Target::Stdout)
            .init();
    }

    Ok(())
}
