use cxx::CxxString;

use log::{trace, debug, info, warn, error, LevelFilter};
use log4rs::{
    config::{load_config_file, runtime::Config, Appender, Root},
    append::console::ConsoleAppender,
    encode::json::JsonEncoder,
};

use crate::ffi::LogLevel;

#[cxx::bridge]
mod ffi {
    enum LogLevel {
        Trace,
        Debug,
        Info,
        Warn,
        Error,
    }

    #[namespace = "rust_logger"]
    extern "Rust" {
        fn log(msg: &CxxString, level: LogLevel);
        fn init_logger(filename: &CxxString);
    }
}

fn log(msg: &CxxString, level: LogLevel) {
    match level {
        LogLevel::Trace => trace!("{}", msg),
        LogLevel::Debug => debug!("{}", msg),
        LogLevel::Info => info!("{}", msg),
        LogLevel::Warn => warn!("{}", msg),
        LogLevel::Error => error!("{}", msg),
        _ => {},
    };
}

fn default_logger_config() -> Config {
    let stdout: ConsoleAppender = ConsoleAppender::builder()
        .encoder(Box::new(JsonEncoder::new()))
        .build();
    let log_config = log4rs::config::Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(LevelFilter::Error))
        .unwrap();

    log_config    
}

fn init_logger(filename: &CxxString) {
    let filename: String = filename.to_string();
    let log_config = match load_config_file(filename, Default::default()) {
        Ok(config) => config,
        Err(error) => {
            println!("There was a problem opening the logger config: {:?}. Defaulting to JSON logger.", error);
            default_logger_config()
        },
    };

    log4rs::init_config(log_config).unwrap();
}
