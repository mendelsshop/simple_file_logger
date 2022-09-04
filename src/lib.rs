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
/// use simple_file_logger::init_logger;
/// use log::info;
///
/// init_logger("my_app", None);
/// info!("Hello, world!");
/// ```

pub fn init_logger(program_name: &str, level: Option<&str>) -> Result<(), Box<dyn Error>> {
    if program_name.is_empty() {
        return Err("Program name must not be empty.".into());
    }
    if let Some(level) = level {
        match level {
            "trace" => {}
            "debug" => {}
            "info" => {}
            "warn" => {}
            "error" => {}
            other => {
                return Err(format!("Invalid log level: {}", other).into());
            }
        }
    }
    let path = get_log_path(program_name);
    let log_path = path.to_str().unwrap();
    let log_path = flexi_logger::FileSpec::default().directory(log_path);
    Logger::try_with_str(level.unwrap_or("info"))?
        .log_to_file(log_path)
        .start()?;
    Ok(())
}
