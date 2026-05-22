#[cfg(test)]
mod logger_tests_macros {
    use bt_logger::{LogLevel, LogTarget, build_logger, get_error, get_fatal, log_debug, log_error, log_fatal, log_info, log_trace, log_verbose, log_warning};

    #[test]
    fn test_all_macros(){
        build_logger("BACHUETECH", "LOGGER.TEST", LogLevel::VERBOSE, LogTarget::STD_OUT, None);
        log_verbose!("sec1","Test Sec1");
        log_debug!("sec1","Test Sec1");
        log_error!("sec1","Test Sec1");
        log_fatal!("sec1","Test Sec1");
        log_info!("sec1","Test Sec1");
        log_trace!("sec1","Test Sec1");
        log_warning!("sec1","Test Sec1");
    }

    #[test]
    fn test_all_get_macros(){
        build_logger("BACHUETECH", "LOGGER.TEST", LogLevel::VERBOSE, LogTarget::STD_OUT, None);
        println!("Get: {}", get_fatal!("sec1","Test Sec1"));
        println!("Get: {}", get_error!("sec1","Test Sec1"));
    }

    #[test]    
    fn test_all_macros_empty_section(){
        build_logger("BACHUETECH", "LOGGER.TEST", LogLevel::VERBOSE, LogTarget::STD_OUT, None);
        log_verbose!("","Test No Section");
        log_debug!("","Test No Section");
        log_error!("","Test No Section");
        log_fatal!("","Test No Section");
        log_info!("","Test No Section");
        log_trace!("","Test No Section");
        log_warning!("","Test No Section");
    }  

    #[test]
    fn test_all_get_macros_empty_section(){
        build_logger("BACHUETECH", "LOGGER.TEST", LogLevel::VERBOSE, LogTarget::STD_OUT, None);
        println!("Get: {}", get_fatal!("","Test No Section"));
        println!("Get: {}", get_error!("","Test No Section"));
    }      
}