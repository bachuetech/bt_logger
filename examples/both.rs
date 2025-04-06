use bt_logger::{build_logger, get_error, get_fatal, LogLevel, LogTarget}; 
use bt_logger::{log_debug, log_error, log_fatal, log_info, log_trace, log_verbose, log_warning, };

fn main() {
    build_logger("BACHUETECH", "LOGGER.TEST", LogLevel::VERBOSE, LogTarget::STD_BOTH );
    log_fatal!("test_level","2x FATAL from {}","BT Logger");
    log_error!("test_level","2x ERROR from {}","BT Logger");
    log_warning!("test_level","2x WARN from {}","BT Logger");
    log_info!("test_level","2x INFO from {}","BT Logger");
    log_debug!("test_level","2x DEBUG from {}","BT Logger");
    log_trace!("test_level","2x TRACE from {}","BT Logger");
    log_verbose!("test_level","2x VERBOSE from {}","BT Logger");

    let msg = get_fatal!("test_level","FATAL from {}","BT Logger");
    println!("Fatal: '{:?}'", msg);

    let msg = get_error!("test_level","ERROR from {}","BT Logger");
    println!("ERROR: '{:?}'", msg);        
}