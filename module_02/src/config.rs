//! This module contains the configuration options for the application.
//! # Examples:
//! ```
//! use module_02::config::Logging;
//! let config = Logging::new();
//! ```
//!
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

pub enum LogOutput {
    Stdout,
    Stderr,
    File(String),
}

/// This struct contains configuration options for the application.
/// # Examples:
/// ```
/// use module_02::config::Logging;
/// let config = Logging::new();
/// ```
///
/// Creating a new instance of the Logging struct:
/// ```
/// use module_02::config::{Logging, LogLevel, LogOutput};
/// let config = Logging{ enabled: true, level: LogLevel::Info, destination: LogOutput::Stdout };
/// ```
pub struct Logging {
    pub enabled: bool,
    pub level: LogLevel,
    pub destination: LogOutput,
}

impl Default for Logging {
    fn default() -> Self {
        Self::new()
    }
}

impl Logging {
    pub fn new() -> Self {
        Self {
            enabled: false,
            level: LogLevel::Info,
            destination: LogOutput::Stdout,
        }
    }
}
