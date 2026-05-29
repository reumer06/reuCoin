use crate::core::{Config, Core, FeeConfig, FeeType, Recipient};
use anyhow::Result;
use std::panic;
use std::path::PathBuf;
use tracing::*;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

// initialize tracing to save logs into logs/folder
pub fn setup_tracing() -> Result<()> {
    let file_appender = RollingFileAppender::new(Rotation::DAILY, "logs", "wallet.log");
    tracing_subscriber::registry()
        .with(fmt::layer().with_writer(file_appender))
        .with(EnvFilter::from_default_env().add_directive((tracing::Level::TRACE.into())))
        .init();
    Ok(())
}

pub fn setup_panic_hook() {
    panic::set_hook(Box::new(|panic_info| {
        let backtrace = std::backtrace::Backtrace::force_capture();
        error!("Application panicked!");
        error!("Panic info: {}", panic_info);
        error!("Backtrace: {}", backtrace);
    }));
}
