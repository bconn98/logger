#include "cxx_bridge/lib.h"
#include <string>

int main()
{
    std::string lcPath = "cfg/log4rs.yml";
    rust_logger::init_logger(lcPath);

    std::string lcLog = "This is my special C++ log";
    rust_logger::log_trace(lcLog);
    rust_logger::log_debug(lcLog);
    rust_logger::log_info(lcLog);
    rust_logger::log_warn(lcLog);
    rust_logger::log_error(lcLog);
}
