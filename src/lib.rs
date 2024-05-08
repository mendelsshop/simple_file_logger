#![doc = include_str!("../README.md")]
use flexi_logger::{FlexiLoggerError, Logger};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "clap")]
use clap::Subcommand;
extern crate dirs;

use std::{fmt, path::PathBuf};

fn get_log_path(program_name: &str) -> Option<PathBuf> {
    let mut path = dirs::data_local_dir()?;
    path.push(program_name);
    path.push("log");
    Some(path)
}

/// Errors that this crate can produce.
#[derive(Debug)]
pub enum Error {
    /// When [flexi_logger] produces an error during logging initialization.
    Log(FlexiLoggerError),
    // When you give an empty program name to [`init_logger`].
    EmptyProgramName,
    /// If your system does not have a path for application data (see [init_logger] for more information about application data folders).
    NoLogPathFound,
}

impl std::error::Error for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Log(log) => write!(f, "FlexiLogger Error: {}", log),
            Error::EmptyProgramName => write!(f, "Empty program name"),
            Error::NoLogPathFound => write!(f, "Application data folder could not be found"),
        }
    }
}

/// This initializes the logger with the given log level or the default log level (info) if no log level is given.
/// Log level can be one of the following: trace, debug, info, warn, error
/// The log file is located:
///
/// | OS | Path | Example |
/// | --- | --- | --- |
/// |Windows| `%FOLDERID_LocalAppData%\program_name\log\program_nametime_stamp.log` | C:\Users\username\AppData\Local\program_name\log\program_name_2020-05-01T12-34-56.log|
/// |Linux| `$XDG_DATA_HOME/program_name/log/program_name_time_stamp.log` |/home/username/.local/share/program_name/log/program_name_2020-05-01T12-34-56.log|
/// |macOS| `$HOME/Library/Application Support/program_name/log/program_nametime_stamp.log` |Users/username/Library/Application Support/program_name/log/program_name_2020-05-01T12-34-56.log|
///
/// Once the logger is initialized, it can be used with the macros: trace!, debug!, info!, warn!, error! from the [log](https://crates.io/crates/log) crate.
/// # Example:
/// ```rust
/// use simple_file_logger::{init_logger, LogLevel};
/// use log::info;
///
/// init_logger("my_app", LogLevel::Info).unwrap();
/// info!("Hello, world!");
/// ```

pub fn init_logger(program_name: &str, level: LogLevel) -> Result<(), Error> {
    if program_name.is_empty() {
        // return Err("Program name must not be empty.".into());
        return Err(Error::EmptyProgramName);
    }
    let level = match level {
        LogLevel::Trace => "trace",
        LogLevel::Debug => "debug",
        LogLevel::Info => "info",
        LogLevel::Warn => "warn",
        LogLevel::Error => "error",
    };
    let path = get_log_path(program_name).ok_or(Error::NoLogPathFound)?;
    let log_path = path.to_str().unwrap();
    let log_path = flexi_logger::FileSpec::default().directory(log_path);
    Logger::try_with_str(level)
        .map_err(Error::Log)?
        .log_to_file(log_path)
        .start()
        .map_err(Error::Log)?;
    Ok(())
}
#[macro_export]
/// This macro initializes the logger with the given log level or the default log level (info) if no log level is given.
///
/// Log level can be one of the following: trace, debug, info, warn, error as from [LogLevel](enum.LogLevel.html) enum.
///
/// expands to a result of type `Result<(), Box<dyn Error>>` like [init_logger](fn.init_logger.html).
///
/// # Example:
/// ```rust
/// use simple_file_logger::init_logger;    
/// use log::info;
///
/// init_logger!("my_app").unwrap();
/// info!("Hello, world!");
macro_rules! init_logger {
    ($program_name:expr, $level:expr) => {
        $crate::init_logger($program_name, $level)
    };
    ($program_name:expr) => {
        $crate::init_logger!($program_name, $crate::LogLevel::Info)
    };
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "clap", derive(Subcommand))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
/// Used to set the minimum log level displayed in the log file.
pub enum LogLevel {
    Trace,
    Debug,
    #[default]
    /// Default log level
    Info,
    Warn,
    Error,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_log_path() {
        let path = get_log_path("my_app");
        let binding = path.unwrap();
        let path = binding.to_str().unwrap();
        assert!(path.contains("my_app"));
        assert!(path.contains("log"));
    }

    #[test]
    fn test_init_logger() {
        init_logger!("my_app").unwrap();
    }
}
