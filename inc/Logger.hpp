
#include "cxx_bridge/lib.h"

#define LOGTRACE(x) rust_logger::log(x, LogLevel::Trace, __FILE__, __FUNCTION__, __LINE__);
#define LOGDEBUG(x) rust_logger::log(x, LogLevel::Debug, __FILE__, __FUNCTION__, __LINE__);
#define LOGINFO(x) rust_logger::log(x, LogLevel::Info, __FILE__, __FUNCTION__, __LINE__);
#define LOGWARN(x) rust_logger::log(x, LogLevel::Warn, __FILE__, __FUNCTION__, __LINE__);
#define LOGERROR(x) rust_logger::log(x, LogLevel::Error, __FILE__, __FUNCTION__, __LINE__);
