
#include <string>
#include <myLib.hpp>
#include <Logger.hpp>

int main()
{
    std::string lcPath = "cfg/log4rs.yml";
    rust_logger::init_logger(lcPath);

    LOGTRACE("This is my special C++ trace log");
    LOGDEBUG("This is my special C++ debug log");
    LOGINFO("This is my special C++ info log");
    LOGWARN("This is my special C++ warn log");
    LOGERROR("This is my special C++ error log");

    myFunc();
}
