# Basic CMake setup
cmake_minimum_required(VERSION 3.19)
file(WRITE "${CMAKE_BINARY_DIR}/CACHEDIR.TAG" "Signature: 8a477f597d28d172789f06886806bc55")


# Include SDK and rust library
include(sdk/pico_sdk_init.cmake)
add_subdirectory(rust)


# Define project
project(ws2812b C CXX ASM)
set(CMAKE_C_STANDARD 11)
set(CMAKE_CXX_STANDARD 17)
add_compile_options(-Wall -Wextra -Werror)

pico_sdk_init()
set(PICO_USE_STACK_GUARDS 1)


# Define binary
file(
    GLOB SRC_FILES
    ${PROJECT_SOURCE_DIR}/c/*.c
    ${PROJECT_SOURCE_DIR}/c/**/*.c)
add_executable(${PROJECT_NAME} ${SRC_FILES})

file(
    GLOB PIO_FILES
    ${PROJECT_SOURCE_DIR}/c/*.pio
    ${PROJECT_SOURCE_DIR}/c/**/*.pio)
pico_generate_pio_header(${PROJECT_NAME} ${PIO_FILES})

target_link_libraries(
    ${PROJECT_NAME}
    pico_stdlib
    hardware_pio
    pico_multicore
    rust)


# Enable USB and build map/bin/hex file etc.
pico_enable_stdio_usb(${PROJECT_NAME} 1)
pico_enable_stdio_uart(${PROJECT_NAME} 1)
pico_enable_stdio_semihosting(${PROJECT_NAME} 0)
pico_add_extra_outputs(${PROJECT_NAME})
