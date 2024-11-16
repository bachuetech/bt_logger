pub mod logger {
    use std::sync::Mutex;
    use std::fmt;
    use lazy_static::lazy_static;
    use chrono::prelude::*;

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
    
    #[derive(Clone)]
    pub enum LogTarget{
        STD_ERROR,
        STD_OUT,
        STD_BOTH,
    }
    
    #[derive(Clone)]
    pub struct Logger{ //ToDo, check if this can be private
        log_tag: String,
        log_app: String,
        current_log_level_value: u8,
        output_destination: LogTarget,
    }
    
    impl Logger{
        fn new(tag: &str, application: &str, level: LogLevel, output:LogTarget) -> Self{
            Logger { log_tag: tag.to_owned(), log_app: application.to_owned(),
                     current_log_level_value: level as u8, output_destination: output}
        }
    
        fn get_current_time(&self) -> String{
            let current_time: DateTime<Utc> = Utc::now();
            // Format the current time as a string
            current_time.format("%Y-%m-%dT%H:%M:%S%.3f%z").to_string()
        }
    
        fn get_formatted_msg(&self, level: LogLevel, source: &str, msg: &String) -> String{
            format!("{} {} {} {} {}|>|{}", self.log_tag, self.log_app, self.get_current_time(), level, source, msg)
        }
    
        fn log_stdout(&self, message: &String ){
            println!("{}", message);
        }
    
        fn log_stderr(&self, message: &String){
            eprintln!("{}", message);
        }
    
        pub fn log_msg(&self, msg: &String, level: LogLevel, module: &str, function: &str){ //source: &str){
    
            let log_msg = self.get_formatted_msg(level, &format!("{}::{}",module, function), msg);
    
            match self.output_destination {
                LogTarget::STD_ERROR => {self.log_stderr(&log_msg);},
                LogTarget::STD_OUT => {self.log_stdout(&log_msg);},
                LogTarget::STD_BOTH => {  self.log_stderr(&log_msg);
                                               self.log_stdout(&log_msg); }, 
            }
        }
    
  
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
    

    pub fn build_logger(tag: &str, application: &str, level: LogLevel, output:LogTarget){
        let mut _logger = LOGGER.lock().unwrap();
        if _logger.is_none(){
            *_logger = Some(Logger::new(tag, application, level, output));
        }
    }
    
    pub fn get_logger() -> Logger{
        let _logger = LOGGER.lock().unwrap();
        _logger.clone().unwrap()
    }
    
    #[macro_export]
    macro_rules! log_fatal {
        ($function_name:expr, $($arg:tt)+) => {{
            let log = bt_logger::logger::get_logger();
            if log.log_this(bt_logger::logger::LogLevel::FATAL){
                let module = module_path!().to_owned();
                let msg =  std::fmt::format(format_args!($($arg)+));
                log.log_msg(&msg, bt_logger::logger::LogLevel::FATAL, &module,$function_name);
            }
        }};
    }
    
    #[macro_export]
    macro_rules! log_error {
        ($function_name:expr, $($arg:tt)+) => {{
            let log = bt_logger::logger::get_logger();
            if log.log_this(bt_logger::logger::LogLevel::ERROR){
                let module = module_path!().to_owned();
                let msg =  std::fmt::format(format_args!($($arg)+));
                log.log_msg(&msg, bt_logger::logger::LogLevel::ERROR, &module, $function_name);
            }
        }};
    }
    
    #[macro_export]
    macro_rules! log_warning{
        ($function_name:expr, $($arg:tt)+) => {{
            let log = bt_logger::logger::get_logger();
            if log.log_this(bt_logger::logger::LogLevel::WARN){
                let module = module_path!().to_owned();
                let msg =  std::fmt::format(format_args!($($arg)+));
                log.log_msg(&msg, bt_logger::logger::LogLevel::WARN, &module,$function_name);
            }
        }};
    }
    
    #[macro_export]
    macro_rules! log_info {
        ($function_name:expr, $($arg:tt)+) => {{
            let log = bt_logger::logger::get_logger();
            if log.log_this(bt_logger::logger::LogLevel::INFO){
                let module = module_path!().to_owned();
                let msg =  std::fmt::format(format_args!($($arg)+));
                log.log_msg(&msg, bt_logger::logger::LogLevel::INFO, &module,$function_name);
            }
        }};
    }
    
    #[macro_export]
    macro_rules! log_debug {
        ($function_name:expr, $($arg:tt)+) => {{
            let log = bt_logger::logger::get_logger();
            if log.log_this(bt_logger::logger::LogLevel::DEBUG){
                let module = module_path!().to_owned();
                let msg =  std::fmt::format(format_args!($($arg)+));
                log.log_msg(&msg, bt_logger::logger::LogLevel::DEBUG, &module,$function_name);
            }
        }};
    }
    
    #[macro_export]
    macro_rules! log_trace {
        ($function_name:expr, $($arg:tt)+) => {{
            let log = bt_logger::logger::get_logger();
            if log.log_this(bt_logger::logger::LogLevel::TRACE){
                let module = module_path!().to_owned();
                let msg =  std::fmt::format(format_args!($($arg)+));
                log.log_msg(&msg, bt_logger::logger::LogLevel::TRACE, &module,$function_name);
            }
        }};
    }
    
    #[macro_export]
    macro_rules! log_verbose {
        ($function_name:expr, $($arg:tt)+) => {{
            let log = bt_logger::logger::get_logger();
            if log.log_this(bt_logger::logger::LogLevel::VERBOSE){
                let module = module_path!().to_owned();
                let msg =  std::fmt::format(format_args!($($arg)+));
                log.log_msg(&msg, bt_logger::logger::LogLevel::VERBOSE, &module,$function_name);
            }
        }};
    }    

}
