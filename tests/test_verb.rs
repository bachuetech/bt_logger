#[cfg(test)]
mod logger_tests_none {
    use bt_logger::{build_logger_args, get_error, get_fatal, LogLevel};
    use regex::Regex;

    #[test]
    fn test_get_all_msg_wparams_err(){
        println!("test {} GET Msg",LogLevel::VERBOSE);
        let args: Vec<String> = vec!["LOGLVL=V".to_string(),"LOGDST=STDERR".to_string()];
        let patter_err = "BACHUETECH LOGGER.TEST .+ E test::logger_tests::test_level|>|ERROR MSG from BT Logger";
        build_logger_args("BACHUETECH", "LOGGER.TEST", &args);   
        println!("Msg E: {:?}", get_error!("test_level","ERROR MSG from {}","BT Logger"));
        assert!(Regex::new(patter_err).unwrap().is_match( get_error!("test_level","ERROR MSG from {}","BT Logger").as_str() ) );
    }

    #[test]
    fn test_get_all_msg_wparams_fatal(){
        println!("test {} GET Msg",LogLevel::VERBOSE);
        let args: Vec<String> = vec!["LOGLVL=V".to_string(),"LOGDST=STDERR".to_string()];
        let patter_fatal = "BACHUETECH LOGGER.TEST .+ F test::logger_tests::test_level|>|FATAL MSG from BT Logger";
        build_logger_args("BACHUETECH", "LOGGER.TEST", &args);             
        println!("Msg F: {:?}", get_fatal!("test_level","FATAL MSG from {}","BT Logger"));
        assert!(Regex::new(patter_fatal).unwrap().is_match( get_fatal!("test_level","MSG from {}","BT Logger").as_str() ) );
    }

    #[test]
    fn test_get_all_msg_noparams_err(){
        println!("test {} GET Msg",LogLevel::VERBOSE);
        let args: Vec<String> = Vec::new(); //vec!["LOGLVL=V".to_string(),"LOGDST=STDERR".to_string()];
        let patter_err = "BACHUETECH LOGGER.TEST .+ E test::logger_tests::test_level|>|ERROR MSG from BT Logger";
        build_logger_args("BACHUETECH", "LOGGER.TEST", &args);         
        println!("Msg E: {:?}", get_error!("test_level","ERROR MSG from {}","BT Logger"));
        assert!(Regex::new(patter_err).unwrap().is_match( get_error!("test_level","ERROR MSG from {}","BT Logger").as_str() ) );
    }

    #[test]
    fn test_get_all_msg_noparams_fatal(){
        println!("test {} GET Msg",LogLevel::VERBOSE);
        let args: Vec<String> = Vec::new(); //vec!["LOGLVL=V".to_string(),"LOGDST=STDERR".to_string()];
        let patter_fatal = "BACHUETECH LOGGER.TEST .+ F test::logger_tests::test_level|>|FATAL MSG from BT Logger";
        build_logger_args("BACHUETECH", "LOGGER.TEST", &args);              
        println!("Msg F: {:?}", get_fatal!("test_level","FATAL MSG from {}","BT Logger"));
        assert!(Regex::new(patter_fatal).unwrap().is_match( get_fatal!("test_level","MSG from {}","BT Logger").as_str() ) );
    }    

    #[test]
    fn test_get_all_msg_wrongparams_err(){
        println!("test {} GET Msg",LogLevel::VERBOSE);
        let args: Vec<String> = vec!["LOGLVLinvalid=V".to_string(),"LOGDSTinvalid=STDERR".to_string()];
        let patter_err = "BACHUETECH LOGGER.TEST .+ E test::logger_tests::test_level|>|ERROR MSG from BT Logger";
        build_logger_args("BACHUETECH", "LOGGER.TEST", &args);           
        println!("Msg E: {:?}", get_error!("test_level","ERROR MSG from {}","BT Logger"));
        assert!(Regex::new(patter_err).unwrap().is_match( get_error!("test_level","ERROR MSG from {}","BT Logger").as_str() ) );
    }

    #[test]
    fn test_get_all_msg_wrongparams(){
        println!("test {} GET Msg",LogLevel::VERBOSE);
        let args: Vec<String> = vec!["LOGLVLinvalid=V".to_string(),"LOGDSTinvalid=STDERR".to_string()];
        let patter_fatal = "BACHUETECH LOGGER.TEST .+ F test::logger_tests::test_level|>|FATAL MSG from BT Logger";
        build_logger_args("BACHUETECH", "LOGGER.TEST", &args);         
        println!("Msg F: {:?}", get_fatal!("test_level","FATAL MSG from {}","BT Logger"));
        assert!(Regex::new(patter_fatal).unwrap().is_match( get_fatal!("test_level","MSG from {}","BT Logger").as_str() ) );
    }  

        #[test]
    fn test_get_all_msg_levell_x(){
        println!("test {} GET Msg",LogLevel::VERBOSE);
        let args: Vec<String> = vec!["LOGLVL=X".to_string(),"LOGDST=UNKNOWN".to_string()];
        let patter_fatal = "BACHUETECH LOGGER.TEST .+ F test::logger_tests::test_level|>|FATAL MSG from BT Logger";
        build_logger_args("BACHUETECH", "LOGGER.TEST", &args);          
        println!("Msg F: {:?}", get_fatal!("test_level","FATAL MSG from {}","BT Logger"));
        assert!(Regex::new(patter_fatal).unwrap().is_match( get_fatal!("test_level","MSG from {}","BT Logger").as_str() ) );
    } 


    #[test]
    fn test_get_all_msg_levell_badparams(){
        println!("test {} GET Msg",LogLevel::VERBOSE);
        let args: Vec<String> = vec!["LOGLVLX".to_string(),"LOGDSTUNKNOWN".to_string()];
        let patter_fatal = "BACHUETECH LOGGER.TEST .+ F test::logger_tests::test_level|>|FATAL MSG from BT Logger";
        build_logger_args("BACHUETECH", "LOGGER.TEST", &args);          
        println!("Msg F: {:?}", get_fatal!("test_level","FATAL MSG from {}","BT Logger"));
        assert!(Regex::new(patter_fatal).unwrap().is_match( get_fatal!("test_level","MSG from {}","BT Logger").as_str() ) );
    }    
}