# Project Title
BT Logger

## Description
A simple and lightweight logger for Rust with various features such as logging to different output destinations (stdout, stderr), formatting log messages, and level checking.

## Usage
To use the bt_logger module, you would create a logger instance with the desired configuration and then use the macros to log messages at different levels. For example:
```
build_logger("BACHUETECH", "My Application", LogLevel::INFO, LogTarget::StdOut, Some("../bt_logs/bachuetech_log.log".to_owned()) );
 .......
log_info!("function_name","Hello, {}", "Bachuetech User");
 .......
let msg = get_fatal!("test_level","FATAL from {}","BT Logger");
```

## Version History
* 0.1.0
    * Initial Release
* 0.2.0
    * Get a formatted message for Fatal or Error as String without logging the message.
* 0.2.2
    * Move to Rust 2024 Edition
* 0.2.3
    * Add passing args to builder to define log level (loglvl) and log destination (logdst)
    * Add using environment variables to setup Log Level and Log Output
        - Log Level Variable: btlogger_log_level
        - Log Output (Destination) Variable: btlogger_log_output
* 0.2.4
    * Add STDERR as a string option for the destination from the env variable (btlogger_log_output) or args (logdst)
* 0.2.5
    * Make args a reference &args
* 0.3.0
    * Added Target Log Destination None
    * Can now redirect any log level to a file, even if destination is None
    * Log and Save to a file will run async
* 0.3.1
    * Internal changes to improve performance. 
    * Remove Mutex in favor of Lazy.
    * Fail silently if logger is not initialized. println a warning
* 0.3.2
    * Internal changes, when using build_logger_args, to default to VERBOSE, STD_OUTPUT (dev) or WARN STD_ERROR (release)



## License
GPL-3.0-only
