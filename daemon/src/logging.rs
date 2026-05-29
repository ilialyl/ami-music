use std::{path::PathBuf, time::SystemTime};

use anyhow::Result;

pub fn setup_logger() -> Result<()> {
    let log_path = PathBuf::from("ami-daemon.log");

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .level_for("lofty", log::LevelFilter::Error)
        .level_for("zbus", log::LevelFilter::Error)
        .level_for("tracing", log::LevelFilter::Error)
        .level_for("symphonia", log::LevelFilter::Error)
        .level_for("symphonia_core", log::LevelFilter::Error)
        .level_for("symphonia_format_ogg", log::LevelFilter::Error)
        .chain(fern::log_file(log_path)?)
        .apply()?;
    Ok(())
}
