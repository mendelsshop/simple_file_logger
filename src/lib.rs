use flexi_logger::Logger;
extern crate dirs;

use std::{error::Error, path::PathBuf};

fn get_log_path(program_name: &str) -> PathBuf {
    let mut path = dirs::data_local_dir().unwrap();
    path.push(program_name);
    path.push("log");
    path
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

pub fn init_logger(program_name: &str, level: LogLevel) -> Result<(), Box<dyn Error>> {
    if program_name.is_empty() {
        return Err("Program name must not be empty.".into());
    }
    let level = match level {
        LogLevel::Trace => "trace",
        LogLevel::Debug => "debug",
        LogLevel::Info => "info",
        LogLevel::Warn => "warn",
        LogLevel::Error => "error",

    };
    let path = get_log_path(program_name);
    let log_path = path.to_str().unwrap();
    let log_path = flexi_logger::FileSpec::default().directory(log_path);
    Logger::try_with_str(level)?
        .log_to_file(log_path)
        .start()?;
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
        init_logger($program_name, $level);
    };
    ($program_name:expr) => {
        init_logger($program_name, $crate::LogLevel::Info);
    };
}

/// Used to set the minimum log level displayed in the log file.
pub enum LogLevel {
    Trace,
    Debug,
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
        let path = path.to_str().unwrap();
        assert!(path.contains("my_app"));
        assert!(path.contains("log"));
    }

    #[test]
    fn test_init_logger() {
        init_logger!("my_app");
    }
}
