# Project Title
BT Logger

## Description
A simple and lightweight logger for Rust with various features such as logging to different output destinations (stdout, stderr), formatting log messages, and level checking.

## Usage
To use the bt_logger module, you would create a logger instance with the desired configuration and then use the macros to log messages at different levels. For example:
```
build_logger("BACHUETECH", "My Application", LogLevel::INFO, LogTarget::StdOut);
 .......
log_info!("function_name","Hello, {}", "Bachuetech User");
 .......
let msg = get_fatal!("test_level","FATAL from {}","BT Logger");
```

## Version History
* 0.1.0
    * Initial Release
* 0.2.0
    * Get a formated message for Fatal or Error as String without out logging the error.

## License
GPL-3.0-only
