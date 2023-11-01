#include "cxx_bridge/lib.h"
#include <string>

int main()
{
    std::string lcPath = "cfg/log4rs.yml";
    rust_logger::init_logger(lcPath);

    std::string lcLog = "This is my special C++ log";
    rust_logger::log(lcLog, LogLevel::Trace);
    rust_logger::log(lcLog, LogLevel::Debug);
    rust_logger::log(lcLog, LogLevel::Info);
    rust_logger::log(lcLog, LogLevel::Warn);
    rust_logger::log(lcLog, LogLevel::Error);
}
