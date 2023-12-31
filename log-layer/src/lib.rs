use log::{logger, Level, LevelFilter, Record};
use log4rs::{
    append::console::ConsoleAppender,
    config::{load_config_file, runtime::Config, Appender, Root},
    encode::json::JsonEncoder,
};
use std::path::Path;

use crate::ffi::LogLevel;
/// Bring the LogLevel enum and C++ Strings into scope of the library from
/// the ffi module.
use cxx::CxxString;

/// This module defines the interface between Rust and C++ code. The actual
/// code is generated by the tool cxx.rs.
#[cxx::bridge]
mod ffi {

    /// Redefinition of the Log4rs LogLevels
    enum LogLevel {
        Error,
        Warn,
        Info,
        Debug,
        Trace,
    }

    #[namespace = "rust_logger"]
    extern "Rust" {
        /// The log method definition.
        fn log(
            msg: &CxxString,
            level: LogLevel,
            file: &CxxString,
            function: &CxxString,
            line: u32,
            app_name: &CxxString,
        );

        /// The init logger defintion using the path to a configuration file.
        /// If the filename is not a valid path, a deafult logger is created
        /// using the JSON logger with a strictness of Error to console.
        fn init_logger(filename: &CxxString);
    }
}

/// Writes a log message bypassing the log macro from the Rust standard
/// library. This is required to override the fields: file, function, line,
/// and app_name from the location of the log message. Instead these fields
/// passed into the function.
fn log(
    msg: &CxxString,
    level: LogLevel,
    file: &CxxString,
    function: &CxxString,
    line: u32,
    app_name: &CxxString,
) {
    let mut builder: log::RecordBuilder<'_> = Record::builder();
    let file: String = file.to_string();
    let module_path: String = function.to_string();
    let target: &str = app_name.to_str().unwrap();

    let file: &str = Path::new(&file).file_name().unwrap().to_str().unwrap();

    let level: Level = match level {
        LogLevel::Trace => Level::Trace,
        LogLevel::Debug => Level::Debug,
        LogLevel::Info => Level::Info,
        LogLevel::Warn => Level::Warn,
        LogLevel::Error => Level::Error,
        _ => Level::Error,
    };

    logger().log(
        &builder
            .args(format_args!("{}", msg))
            .level(level)
            .target(target)
            .module_path(Some(&module_path))
            .file(Some(file))
            .line(Some(line))
            .build(),
    );
}

/// Create log4rs Config object using the code definition. This will always
/// create a JSON based logger to console with a log level of Error.
fn default_logger_config() -> Config {
    let appender: ConsoleAppender = ConsoleAppender::builder()
        .encoder(Box::new(JsonEncoder::new()))
        .build();
    let log_config: Config = log4rs::config::Config::builder()
        .appender(Appender::builder().build("appender", Box::new(appender)))
        .build(
            Root::builder()
                .appender("appender")
                .build(LevelFilter::Error),
        )
        .unwrap();

    log_config
}

/// Create a config and initialize log4rs either from a user supplied config
/// file or via a default definition.
fn init_logger(filename: &CxxString) {
    let filename: String = filename.to_string();
    let log_config: Config = match load_config_file(filename, Default::default()) {
        Ok(config) => config,
        Err(error) => {
            println!(
                "There was a problem opening the logger config: {:?}. Defaulting to JSON logger.",
                error
            );
            default_logger_config()
        }
    };

    log4rs::init_config(log_config).unwrap();
}
