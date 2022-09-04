# Simple File Logger

A simple file logger for rust.

Very basic setup, just provide an app name and an optional log level.

```rust
use simple_file_logger::init_logger;
use log::info;

fn main() {
    init_logger("my_app", None);
    info!("Hello, world!");
}
```

The default log level is `info`.

The log levels are: `trace`, `debug`, `info`, `warn`, `error`.

The log file is located:

| OS | Path | Example |
| --- | --- | --- |
|Windows| %FOLDERID_LocalAppData%\program_name\log\program_nametime_stamp.log | C:\Users\username\AppData\Local\program_name\log\program_name_2020-05-01T12-34-56.log|
|Linux| $XDG_DATA_HOME/program_name/log/program_name_time_stamp.log |/home/username/.local/share/program_name/log/program_name_2020-05-01T12-34-56.log|
|macOS| $HOME/Library/Application Support/program_name/log/program_nametime_stamp.log |Users/username/Library/Application Support/program_name/log/program_name_2020-05-01T12-34-56.log|
