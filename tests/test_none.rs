#[cfg(test)]
mod logger_tests_none {
    use bt_logger::{build_logger, get_error, LogLevel, LogTarget};

    #[test]
    fn test_get_error_msg_when_none(){
        println!("test {} GET Msg",LogLevel::ERROR);
        build_logger("BACHUETECH", "LOGGER.TEST", LogLevel::NONE, LogTarget::STD_OUT);          
        let msg = get_error!("test_level","ERROR MSG from {}","BT Logger");
        println!("Msg: {:?}", &msg);
        assert_eq!(msg,"" );
    }      
}