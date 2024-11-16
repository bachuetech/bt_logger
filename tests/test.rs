use bt_logger::{build_logger, LogLevel, LogTarget}; 
use bt_logger::{log_debug, log_error, log_fatal, log_info, log_trace, log_verbose, log_warning};

#[test]
fn test_info_both(){
    println!("test {} {:?}",LogLevel::INFO, LogTarget::STD_BOTH);
    build_logger("BACHUETECH", "LOGGER.TEST", LogLevel::INFO, LogTarget::STD_BOTH );
    log_fatal!("test_level","FATAL from {}","BT Logger");
    log_error!("test_level","ERROR from {}","BT Logger");
    log_warning!("test_level","WARN from {}","BT Logger");
    log_info!("test_level","INFO from {}","BT Logger");
    log_debug!("test_level","DEBUG from {}","BT Logger");
    log_trace!("test_level","TRACE from {}","BT Logger");
    log_verbose!("test_level","VERBOSE from {}","BT Logger");
}

#[test]
fn test_info_error(){
    println!("test {} {:?}",LogLevel::INFO, LogTarget::STD_ERROR);
    build_logger("BACHUETECH", "LOGGER.TEST", LogLevel::INFO, LogTarget::STD_ERROR);
    log_fatal!("test_level","FATAL from {}","BT Logger");
    log_error!("test_level","ERROR from {}","BT Logger");
    log_warning!("test_level","WARN from {}","BT Logger");
    log_info!("test_level","INFO from {}","BT Logger");
    log_debug!("test_level","DEBUG from {}","BT Logger");
    log_trace!("test_level","TRACE from {}","BT Logger");
    log_verbose!("test_level","VERBOSE from {}","BT Logger");
}

#[test]
fn test_info_out(){
    println!("test {} {:?}",LogLevel::INFO, LogTarget::STD_OUT);
    build_logger("BACHUETECH", "LOGGER.TEST", LogLevel::INFO, LogTarget::STD_OUT);
    log_fatal!("test_level","FATAL from {}","BT Logger");
    log_error!("test_level","ERROR from {}","BT Logger");
    log_warning!("test_level","WARN from {}","BT Logger");
    log_info!("test_level","INFO from {}","BT Logger");
    log_debug!("test_level","DEBUG from {}","BT Logger");
    log_trace!("test_level","TRACE from {}","BT Logger");
    log_verbose!("test_level","VERBOSE from {}","BT Logger");
}