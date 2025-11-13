///Log a Fatal Error. log_fatal!(function name, message, message arguments)
#[macro_export]
macro_rules! log_fatal {
    ($function_name:expr, $($arg:tt)+) => {{
        if let Some(log) = bt_logger::get_logger(){ //fail silently
            if log.log_this(bt_logger::LogLevel::FATAL){
                let module = module_path!().to_owned();
                let msg =  std::fmt::format(format_args!($($arg)+));
                log.log_msg(&msg, bt_logger::LogLevel::FATAL, &module,$function_name);
            }
        }
    }};
}

///Log an Error Message. log_error!(function name, message, message arguments)
#[macro_export]
macro_rules! log_error {
    ($function_name:expr, $($arg:tt)+) => {{
        if let Some(log) = bt_logger::get_logger(){ //fail silently
            if log.log_this(bt_logger::LogLevel::ERROR){
                let module = module_path!().to_owned();
                let msg =  std::fmt::format(format_args!($($arg)+));
                log.log_msg(&msg, bt_logger::LogLevel::ERROR, &module, $function_name);
            }
        }
    }};
}

///Log a Warning message. log_warning!(function name, message, message arguments)
#[macro_export]
macro_rules! log_warning{
    ($function_name:expr, $($arg:tt)+) => {{
        if let Some(log) = bt_logger::get_logger(){ //fail silently
            if log.log_this(bt_logger::LogLevel::WARN){
                let module = module_path!().to_owned();
                let msg =  std::fmt::format(format_args!($($arg)+));
                log.log_msg(&msg, bt_logger::LogLevel::WARN, &module,$function_name);
            }
        }
    }};
}

///Log an Informational message. log_info!(function name, message, message arguments)
#[macro_export]
macro_rules! log_info {
    ($function_name:expr, $($arg:tt)+) => {{
        if let Some(log) = bt_logger::get_logger(){ //fail silently
            if log.log_this(bt_logger::LogLevel::INFO){
                let module = module_path!().to_owned();
                let msg =  std::fmt::format(format_args!($($arg)+));
                log.log_msg(&msg, bt_logger::LogLevel::INFO, &module,$function_name);
            }
        }
    }};
}

///Debugging message. log_debug!(function name, message, message arguments)
#[macro_export]
macro_rules! log_debug {
    ($function_name:expr, $($arg:tt)+) => {{
        if let Some(log) = bt_logger::get_logger(){ //fail silently
            if log.log_this(bt_logger::LogLevel::DEBUG){
                let module = module_path!().to_owned();
                let msg =  std::fmt::format(format_args!($($arg)+));
                log.log_msg(&msg, bt_logger::LogLevel::DEBUG, &module,$function_name);
            }
        }
    }};
}

///Trace message. log_trace!(function name, message, message arguments)
#[macro_export]
macro_rules! log_trace {
    ($function_name:expr, $($arg:tt)+) => {{
        if let Some(log) = bt_logger::get_logger(){ //fail silently
            if log.log_this(bt_logger::LogLevel::TRACE){
                let module = module_path!().to_owned();
                let msg =  std::fmt::format(format_args!($($arg)+));
                log.log_msg(&msg, bt_logger::LogLevel::TRACE, &module,$function_name);
            }
        }
    }};
}

///Very verbose message. log_verbose!(function name, message, message arguments)
#[macro_export]
macro_rules! log_verbose {
    ($function_name:expr, $($arg:tt)+) => {{
        if let Some(log) = bt_logger::get_logger(){ //fail silently
            if log.log_this(bt_logger::LogLevel::VERBOSE){
                let module = module_path!().to_owned();
                let msg =  std::fmt::format(format_args!($($arg)+));
                log.log_msg(&msg, bt_logger::LogLevel::VERBOSE, &module,$function_name);
            }
        }
    }};
}    


///Get a Fatal Error Message. get_fatal!(function name, message, message arguments)
#[macro_export]
macro_rules! get_fatal {
    ($function_name:expr, $($arg:tt)+) => {{
        if let Some(log) = bt_logger::get_logger(){ //fail silently
            if log.log_this(bt_logger::LogLevel::FATAL){
                let module = module_path!().to_owned();
                let msg =  std::fmt::format(format_args!($($arg)+));
                log.get_msg(&msg, bt_logger::LogLevel::FATAL, &module,$function_name)
            }else{
                "".to_owned()
            }
        }else{
            "".to_owned()
        }
    }};
}

///Get a Error Message. get_fatal!(function name, message, message arguments)
#[macro_export]
macro_rules! get_error {
    ($function_name:expr, $($arg:tt)+) => {{
        if let Some(log) = bt_logger::get_logger(){ //fail silently
            if log.log_this(bt_logger::LogLevel::ERROR){
                let module = module_path!().to_owned();
                let msg =  std::fmt::format(format_args!($($arg)+));
                log.get_msg(&msg, bt_logger::LogLevel::ERROR, &module,$function_name)
            }else{
                "".to_owned()
            }
        }else{
            "".to_owned()
        }
    }};
}