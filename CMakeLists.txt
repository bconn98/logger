cmake_minimum_required(VERSION 3.23)

set(CMAKE_TOOLCHAIN_FILE /usr/local/vcpkg/scripts/buildsystems/vcpkg.cmake)
set(CMAKE_EXPORT_COMPILE_COMMANDS true)
set(CMAKE_CXX_STANDARD 20)

project(log-example LANGUAGES CXX VERSION 0.0.1)

####################### Corrosion Linking ############################
find_package(Corrosion CONFIG REQUIRED)

find_program(CXXBRIDGE cxxbridge PATHS "$ENV{HOME}/.cargo/bin/")
if (CXXBRIDGE STREQUAL "CXXBRIDGE-NOTFOUND")
    message("Could not find cxxbridge, trying to install with `cargo install cxxbridge-cmd'")
    find_program(CARGO cargo PATHS "$ENV{HOME}/.cargo/bin/")
    if (CARGO STREQUAL "CARGO-NOTFOUND")
        message(FATAL_ERROR "Requires cargo available in path, install via rustup https://rustup.rs/")
    endif()
    execute_process(COMMAND ${CARGO} install cxxbridge-cmd)
    find_program(CXXBRIDGE cxxbridge PATHS "$ENV{HOME}/.cargo/bin/")
endif()

corrosion_import_crate(MANIFEST_PATH log-layer/Cargo.toml)
corrosion_add_cxxbridge(cxx_bridge CRATE log-layer MANIFEST_PATH log-layer FILES lib.rs)

####################### Corrosion Linking ############################

add_executable(${PROJECT_NAME} src/main.cpp src/test/mylib.cpp)
target_include_directories(${PROJECT_NAME} PRIVATE "inc" "inc/test")
target_link_libraries(${PROJECT_NAME} PRIVATE cxx_bridge)
