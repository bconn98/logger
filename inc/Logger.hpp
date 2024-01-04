#include <source_location>
#include "cxx_bridge/lib.h"
#include <sstream>

/// General LOG macro that uses stringstream to combine all the inputs.
#define LOG(x, level) \
    do \
    { \
        std::stringstream lcLogMsg; \
        lcLogMsg << x; \
        rust_logger::log(lcLogMsg.str(), level, \
                         std::source_location::current().file_name(), \
                         std::source_location::current().function_name(), \
                         std::source_location::current().line(), \
                         program_invocation_name); \
    } while (false)

/// Call the LOG macro at Trace
#define LOGTRACE(x) LOG(x, LogLevel::Trace)
/// Call the LOG macro at Debug
#define LOGDEBUG(x) LOG(x, LogLevel::Debug)
/// Call the LOG macro at Info
#define LOGINFO(x) LOG(x, LogLevel::Info)
/// Call the LOG macro at Warn
#define LOGWARN(x) LOG(x, LogLevel::Warn)
/// Call the LOG macro at Error
#define LOGERROR(x) LOG(x, LogLevel::Error)
