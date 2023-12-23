use cxx::CxxString;

use log::{LevelFilter, logger, Record, Level};
use log4rs::{
    config::{load_config_file, runtime::Config, Appender, Root},
    append::console::ConsoleAppender,
    encode::json::JsonEncoder,
};
use std::path::Path;

use crate::ffi::LogLevel;

#[cxx::bridge]
mod ffi {

    enum LogLevel {
        Error,
        Warn,
        Info,
        Debug,
        Trace,
    }

    #[namespace = "rust_logger"]
    extern "Rust" {
        fn log(msg: &CxxString, level: LogLevel, file: &CxxString, function: &CxxString, line: u32);
        fn init_logger(filename: &CxxString);
    }
}


fn log(msg: &CxxString, level: LogLevel, file: &CxxString, function: &CxxString, line: u32) {
    let mut builder: log::RecordBuilder<'_> = Record::builder();
    let file: String = file.to_string();
    let module_path: String = function.to_string();
    let target: &str = "";

    let file: &str = Path::new(&file).file_name().unwrap().to_str().unwrap();

    let level: Level = match level {
        LogLevel::Trace => Level::Trace,
        LogLevel::Debug => Level::Debug,
        LogLevel::Info => Level::Info,
        LogLevel::Warn => Level::Warn,
        LogLevel::Error => Level::Error,
        _ => Level::Error,
    };

    logger().log(&builder
        .args(format_args!("{}", msg))
        .level(level)
        .target(target)
        .module_path(Some(&module_path))
        .file(Some(file))
        .line(Some(line))
        .build());
}

fn default_logger_config() -> Config {
    let stdout: ConsoleAppender = ConsoleAppender::builder()
        .encoder(Box::new(JsonEncoder::new()))
        .build();
    let log_config: Config = log4rs::config::Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(LevelFilter::Error))
        .unwrap();

    log_config    
}

fn init_logger(filename: &CxxString) {
    let filename: String = filename.to_string();
    let log_config: Config = match load_config_file(filename, Default::default()) {
        Ok(config) => config,
        Err(error) => {
            println!("There was a problem opening the logger config: {:?}. Defaulting to JSON logger.", error);
            default_logger_config()
        },
    };

    log4rs::init_config(log_config).unwrap();
}
