use std::env;

use bt_logger::{build_logger, build_logger_args, get_error, get_fatal, LogLevel, LogTarget}; 
use bt_logger::{log_debug, log_error, log_fatal, log_info, log_trace, log_verbose, log_warning, };

fn main() {
    // Get all command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 1{
        build_logger("BACHUETECH", "BT.AI.UI", LogLevel::VERBOSE, LogTarget::STD_ERROR );
    }else{
        
        let p = args.get(1);
        println!("param: {:?}",p);
        build_logger("BACHUETECH", "BT.AI.UI", LogLevel::VERBOSE, LogTarget::STD_ERROR );
    }

    build_logger_args("BACHUETECH", "LOGGER.TEST", &args );
    log_fatal!("test_level","FATAL from {}","BT Logger");
    log_error!("test_level","ERROR from {}","BT Logger");
    log_warning!("test_level","WARN from {}","BT Logger");
    log_info!("test_level","INFO from {}","BT Logger");
    log_debug!("test_level","DEBUG from {}","BT Logger");
    log_trace!("test_level","TRACE from {}","BT Logger");
    log_verbose!("test_level","VERBOSE from {}","BT Logger");

    let msg = get_fatal!("test_level","FATAL from {}","BT Logger");
    println!("Fatal: {:?}", msg);

    let msg = get_error!("test_level","ERROR from {}","BT Logger");
    println!("ERROR: {:?}", msg);    
}