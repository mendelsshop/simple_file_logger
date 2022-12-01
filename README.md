# [![crates.io](https://img.shields.io/crates/v/simple_file_logger.svg?label=latest%20version)](https://crates.io/crates/simple_file_logger) [![Crates.io](https://img.shields.io/crates/d/simple_file_logger?label=crates.io%20downloads)](https://crates.io/crates/simple_file_logger) [![docs.rs](https://docs.rs/simple_file_logger/badge.svg)](https://docs.rs/simple_file_logger/)

# Simple File Logger

A simple file logger for rust.

Very basic setup, just provide an app name and an optional log level.

```rust
use simple_file_logger::{init_logger, LogLevel};
use log::info;

fn main() {
    init_logger("my_app", Loglevel::Info).unwrap();
    info!("Hello, world!");
}
```

or if you want to use the default log level (and save typing 3 characters):

```rust
use simple_file_logger::init_logger;

fn main() {
    init_logger!("my_app").unwrap();
    info!("Hello, world!");
}
```

The log levels are: `trace`, `debug`, `info` , `warn`, `error`.

The log file is located:

| OS | Path | Example |
| --- | --- | --- |
|Windows| %FOLDERID_LocalAppData%\program_name\log\program_nametime_stamp.log | C:\Users\username\AppData\Local\program_name\log\program_name_2020-05-01T12-34-56.log|
|Linux| $XDG_DATA_HOME/program_name/log/program_name_time_stamp.log |/home/username/.local/share/program_name/log/program_name_2020-05-01T12-34-56.log|
|macOS| $HOME/Library/Application Support/program_name/log/program_nametime_stamp.log |Users/username/Library/Application Support/program_name/log/program_name_2020-05-01T12-34-56.log|
