#[cfg(test)]
mod logger_tests {

    use bt_logger::{build_logger, get_error, get_fatal, LogLevel, LogTarget}; 
    use bt_logger::{log_debug, log_error, log_fatal, log_info, log_trace, log_verbose, log_warning};
    use regex::Regex;

    #[test]
    fn test_info_both(){
        println!("test {} {:?}",LogLevel::VERBOSE, LogTarget::STD_BOTH);
        build_logger("BACHUETECH", "LOGGER.TEST", LogLevel::VERBOSE, LogTarget::STD_BOTH, None );        
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
        println!("test {} {:?}",LogLevel::VERBOSE, LogTarget::STD_ERROR);        
        build_logger("BACHUETECH", "LOGGER.TEST", LogLevel::VERBOSE, LogTarget::STD_ERROR, None);        
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
        println!("test {} {:?}",LogLevel::VERBOSE, LogTarget::STD_OUT);        
        build_logger("BACHUETECH", "LOGGER.TEST", LogLevel::VERBOSE, LogTarget::STD_OUT, None);        
        log_fatal!("test_level","FATAL from {}","BT Logger");
        log_error!("test_level","ERROR from {}","BT Logger");
        log_warning!("test_level","WARN from {}","BT Logger");
        log_info!("test_level","INFO from {}","BT Logger");
        log_debug!("test_level","DEBUG from {}","BT Logger");
        log_trace!("test_level","TRACE from {}","BT Logger");
        log_verbose!("test_level","VERBOSE from {}","BT Logger");
    }

    #[test]
    fn test_get_fatal_msg(){
        println!("test {} GET Msg",LogLevel::FATAL);
        let pattern = "BACHUETECH LOGGER.TEST .+ F test::logger_tests::test_level|>|FATAL from BT Logger";
        build_logger("BACHUETECH", "LOGGER.TEST", LogLevel::FATAL, LogTarget::STD_OUT, None);          
        let msg =   get_fatal!("test_level","FATAL from {}","BT Logger");
        println!("Msg: {:?}", &msg);
        assert!(Regex::new(pattern).unwrap().is_match(&msg) );
    }

    #[test]
    fn test_get_error_msg_werr(){
        println!("test {} GET Msg",LogLevel::ERROR);
        let pattern = "BACHUETECH LOGGER.TEST .+ E test::logger_tests::test_level|>|ERROR MSG from BT Logger";
        build_logger("BACHUETECH", "LOGGER.TEST", LogLevel::ERROR, LogTarget::STD_ERROR, None);        
        let msg = get_error!("test_level","ERROR MSG from {}","BT Logger");        
        println!("test_get_error_msg_werr Msg: {:?}", &msg);
        assert!(Regex::new(pattern).unwrap().is_match( &msg ) );
    }


    #[test]
    fn test_log_error_msg_werr_file(){
        println!("test {} GET Msg",LogLevel::ERROR);
        //let pattern = "BACHUETECH LOGGER.TEST .+ E test::logger_tests::test_level|>|ERROR MSG from BT Logger";
        build_logger("BACHUETECH", "LOGGER.TEST", LogLevel::ERROR, LogTarget::STD_ERROR, Some("../logs/bachuetech_log.log".to_owned()));        
        log_error!("test_level","ERROR MSG from {}","BT Logger");        
        //println!("test_get_error_msg_werr Msg: {:?}", &msg);
        //assert!(Regex::new(pattern).unwrap().is_match( &msg ) );
    }
    
    #[test]
    fn test_get_error_msg_werr_file(){
        println!("test {} GET Msg",LogLevel::ERROR);
        let pattern = "BACHUETECH LOGGER.TEST .+ E test::logger_tests::test_level|>|ERROR MSG from BT Logger";
        build_logger("BACHUETECH", "LOGGER.TEST", LogLevel::ERROR, LogTarget::STD_ERROR, Some("../logs/bachuetech_log.log".to_owned()));        
        let msg = get_error!("test_level","ERROR MSG from {}","BT Logger");        
        println!("test_get_error_msg_werr Msg: {:?}", &msg);
        assert!(Regex::new(pattern).unwrap().is_match( &msg ) );
    }    

    #[test]
    fn test_get_error_msg_no_init_success(){
        println!("test {} GET Msg",LogLevel::ERROR);
    
        let msg = get_error!("test_level","ERROR MSG from {}","BT Logger");        
        println!("test_get_error_msg_werr Msg: '{}'", &msg);
        assert_eq!(msg,"");
    }       
}