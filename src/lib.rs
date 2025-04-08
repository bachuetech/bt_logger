/// A simple and lightweight logger for Rust with various features such as logging to different output destinations (stdout, stderr), formatting log messages, and level checking.
/// 
/// LogLevel
/// Represents the different levels of log severity. The values are ordered from lowest to highest severity.
/// NONE = No logging,
/// FATAL = Fatal error,
/// ERROR = Error message,
/// WARN = Warning message,
/// INFO = Informational message,
/// DEBUG = Debugging message,
/// TRACE = Trace message,
/// VERBOSE = Very verbose message
/// 
/// LogTarget
/// Represents the different output destinations for log messages.
/// STD_ERROR: Logs to stderr
/// STD_OUT: Logs to stdout
/// STD_BOTH: Logs to both stdout and stderr
/// 
/// Macros
/// bt_logger provides several macros for logging different levels of messages. Each macro:
/// - Checks if the log level is sufficient to log the message.
/// - If sufficient, formats the message using get_formatted_msg.
/// - Calls log_msg with the formatted message.
/// 
/// log_fatal!(function name, message, message arguments)
/// log_error!(function name, message, message arguments)
/// log_warning!(function name, message, message arguments)
/// log_info!(function name, message, message arguments)
/// log_debug!(function name, message, message arguments)
/// log_trace!(function name, message, message arguments)
/// log_verbose!(function name, message, message arguments)
/// get_fatal!(function name, message, message arguments)
/// get_error!(function name, message, message arguments)
/// 
/// Usage
/// To use the bt_logger module, you would create a logger instance with the desired configuration and then use the macros to log messages at different levels. For example:
/// build_logger("BACHUETECH", "My Application", LogLevel::INFO, LogTarget::StdOut);
/// .......
/// log_info!("function_name","Hello, {}", "Bachuetech User");
///
/// This code builds a logger instance with the tag BACHUETECH, application name My Application, and log level INFO. It then uses the log_info! macro to log a message to stdout.
/// If the logger is not create the application will stop with the first call of a loggin macro
/// build the logger as early as possible to avoid issue using the build_logger
/// 
/// The log output is:
/// TAG APPLICATION CURRENT_UTC_TIME(YYYY-mm-ddTHH:MM:SS.3fz) level(one capital letter) source(module path::function)|>|message

    use std::sync::Mutex;
    use std::{env, fmt};
    use lazy_static::lazy_static;
    use chrono::prelude::*;

    #[macro_use]
    pub mod macros;


    const LOG_ENV_VAR_LOG_LEVEL: &str = "btlogger_log_level";
    const LOG_ENV_VAR_LOG_OUTPUT: &str = "btlogger_log_output";

/// LogLevel
/// Represents the different levels of log severity. The values are ordered from lowest to highest severity.
/// NONE = No logging,
/// FATAL = Fatal error,
/// ERROR = Error message,
/// WARN = Warning message,
/// INFO = Informational message,
/// DEBUG = Debugging message,
/// TRACE = Trace message,
/// VERBOSE = Very verbose message
    #[repr(u8)]
    #[derive(Clone)]
    pub enum LogLevel{
        NONE = 100,
        FATAL = 60,
        ERROR = 50,
        WARN = 40,
        INFO = 30,
        DEBUG = 20,
        TRACE = 10,
        VERBOSE = 0,
    }
    
    impl fmt::Display for LogLevel{
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self{
                LogLevel::NONE    => write!(f, "N"),
                LogLevel::FATAL   => write!(f, "F"),
                LogLevel::ERROR   => write!(f, "E"),
                LogLevel::WARN    => write!(f, "W"),
                LogLevel::INFO    => write!(f, "I"),
                LogLevel::DEBUG   => write!(f, "D"),
                LogLevel::TRACE   => write!(f, "T"),
                LogLevel::VERBOSE => write!(f, "V"),
            }
        }
    }

    impl LogLevel {
        fn from_str(log_level: &str) -> LogLevel{
            match log_level.to_uppercase().as_str(){
                "N" | "NONE" => LogLevel::NONE,
                "F" | "FATAL" => LogLevel::FATAL,
                "E" | "ERR" | "ERROR" => LogLevel::ERROR,
                "W" | "WARN" | "WARNING"=> LogLevel::WARN,
                "I" | "INFO" => LogLevel::INFO,
                "D" | "DEBUG" => LogLevel::DEBUG,
                "T" | "TRACE" => LogLevel::TRACE,
                "V" | "VERB" | "VERBOSE"=> LogLevel::VERBOSE,
                _ => LogLevel::NONE,
            }
        }
    
    }
/// LogTarget
/// Represents the different output destinations for log messages.
/// STD_ERROR: Logs to stderr
/// STD_OUT: Logs to stdout
/// STD_BOTH: Logs to both stdout and stderr
    #[derive(Debug, Clone)]
    pub enum LogTarget{
        STD_ERROR,
        STD_OUT,
        STD_BOTH,
    }
    
    impl LogTarget {
        pub fn from_str(log_destination: &str) -> LogTarget{
            match log_destination.to_uppercase().as_str(){
                "ERR" | "ERROR" | "STDERR" | "E" => LogTarget::STD_ERROR,
                "STANDARD" | "STD" | "STDOUT" | "S" => LogTarget::STD_OUT,
                _ => LogTarget::STD_ERROR
            }
        }
    }
/// Logger
/// Represents a logger instance with its configuration.
/// log_tag: The tag used in log messages
/// log_app: The application name used in log messages
/// current_log_level_value: The current log level value (0-100)
/// output_destination: The output destination for log messages (StdError, StdOut, or StdBoth)
    #[derive(Clone)]
    pub struct Logger{ //ToDo, check if this can be private
        log_tag: String,
        log_app: String,
        current_log_level_value: u8,
        output_destination: LogTarget,
    }
    
    impl Logger{
        ///Creates a new logger instance with the given configuration.
        fn new(tag: &str, application: &str, level: LogLevel, output:LogTarget) -> Self{
            Logger { log_tag: tag.to_owned(), log_app: application.to_owned(),
                     current_log_level_value: level as u8, output_destination: output}
        }
    
        ///Returns the current time as a string in the format "YYYY-MM-DDTHH:MM:SS.SSSZ".
        fn get_current_time(&self) -> String{
            let current_time: DateTime<Utc> = Utc::now();
            // Format the current time as a string
            current_time.format("%Y-%m-%dT%H:%M:%S%.3f%z").to_string()
        }
    
        ///Formats a log message with the given level, source, and message.
        fn get_formatted_msg(&self, level: LogLevel, source: &str, msg: &String) -> String{
            format!("{} {} {} {} {}|>|{}", self.log_tag, self.log_app, self.get_current_time(), level, source, msg)
        }
    
        ///Logs a message to stdout.
        fn log_stdout(&self, message: &String ){
            println!("{}", message);
        }
    
        ///Logs a message to stderr.
        fn log_stderr(&self, message: &String){
            eprintln!("{}", message);
        }
    
        ///Logs a message with the given level, message, and module and function name.
        pub fn log_msg(&self, msg: &String, level: LogLevel, module: &str, function: &str){ //source: &str){
    
            let log_msg = self.get_formatted_msg(level, &format!("{}::{}",module, function), msg);
    
            match self.output_destination {
                LogTarget::STD_ERROR =>{self.log_stderr(&log_msg);},
                LogTarget::STD_OUT =>  {self.log_stdout(&log_msg);},
                LogTarget::STD_BOTH => {  self.log_stderr(&log_msg);
                                          self.log_stdout(&log_msg); }, 
            }
        }
    
        ///Get formatted message with the given level, message, and module and function name.
        pub fn get_msg(&self, msg: &String, level: LogLevel, module: &str, function: &str) -> String { //source: &str){
            let log_msg = self.get_formatted_msg(level, &format!("{}::{}",module, function), msg);
            log_msg
        }

        //Check if a particular log_level has to be logged based on the currect configuration
        pub fn log_this(&self, log_level: LogLevel) -> bool{
            if log_level as u8 >= self.current_log_level_value {
                return true
            }
            false
        }
    }
    
    lazy_static! {
        static ref LOGGER: Mutex<Option<Logger>> = Mutex::new(None);
    }
    

    ///Build the logger. Run this function first
/// Configures a global logger instance based on the provided environment variables and options.
///
/// This function takes in a tag, an application name, a log level, and a log target,
/// and uses the `LOG_ENV_VAR_LOG_LEVEL` (btlogger_log_level) and `LOG_ENV_VAR_LOG_OUTPUT` (btlogger_log_output) environment variables
/// to override or set default values for the logger configuration.
///
/// If no environment variables are set, it will use the provided options to configure the logger.
///
/// # Arguments
///
/// * `tag`: The tag or identifier for the logger.
/// * `application`: The application name associated with the logger.
/// * `level`:  An enum log level to use for this logger instance.
/// * `output`:  An enum target to output logs to (e.g. standard error, file, etc.).
    pub fn build_logger(tag: &str, application: &str, level: LogLevel, output:LogTarget){
        let int_level: LogLevel;
        let int_dest: LogTarget;
        match env::var(LOG_ENV_VAR_LOG_LEVEL){
            Ok(levll) => int_level = LogLevel::from_str(&levll),
            Err(_) => int_level = level,
        }

        match env::var(LOG_ENV_VAR_LOG_OUTPUT){
            Ok(levlo) => int_dest = LogTarget::from_str(&levlo),
            Err(_) => int_dest = output,
        }

        let mut _logger = LOGGER.lock().unwrap();
        if _logger.is_none(){
            *_logger = Some(Logger::new(tag, application, int_level, int_dest));
        }
    }
    
    ///Build the logger. Run this function first
/// Builds a logger configuration based on the provided arguments.
///
/// This function takes in a tag and an application name as required parameters,
/// and uses the provided vector of strings to override or set default values for
/// the log level and target.
///
/// If no arguments are provided, it will build a logger with the verbose log level
/// and standard error target. Otherwise, it will use the specified argument values
/// to configure the logger.
///
/// # Arguments
///
/// * `tag`: The tag or identifier for the logger.
/// * `application`: The application name associated with the logger.
/// * `args`: A vector of strings containing key-value pairs for configuring the logger. The key is "LOGLVL" or "LOGDST", 
///           it sets the log level (level) or output target (out_target)
    pub fn build_logger_args(tag: &str, application: &str, args: Vec<String>){
        if args.len() < 1{
            build_logger(tag, application, LogLevel::VERBOSE, LogTarget::STD_ERROR );
        }else{
            let mut level = LogLevel::VERBOSE;
            let mut out_target = LogTarget::STD_ERROR;
            for param in &args{
                match param.split_once("="){
                    Some(t) => {
                        match t.0.to_uppercase().as_str() {
                            "LOGLVL" => level = LogLevel::from_str(t.1),
                            "LOGDST" => out_target = LogTarget::from_str(t.1),
                            _ => (),
                        }
                    }
                    None => () ,
                }

            }

            build_logger(tag, application, level, out_target);
        }
    }

    pub fn get_logger() -> Logger{
        let _logger = LOGGER.lock().unwrap();
        _logger.clone().unwrap()
    }
