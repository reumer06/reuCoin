use crate::core::{Config, Core, FeeConfig, FeeType, Recipient};
use anyhow::Result;
use std::panic;
use std::path::PathBuf;
use tracing::*;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};
