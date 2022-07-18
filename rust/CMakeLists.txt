cmake_minimum_required(VERSION 3.19)
file(WRITE "${CMAKE_BINARY_DIR}/CACHEDIR.TAG" "Signature: 8a477f597d28d172789f06886806bc55")


# Config
project(rust)
set(ARCH thumbv6m-none-eabi)
set(LIBNAME lib${PROJECT_NAME}.a)


# Execute cargo
file(REAL_PATH ./ CARGO_DIR)
add_custom_target(
    ${PROJECT_NAME}-cargo ALL
    COMMAND cargo build --release --target ${ARCH} --target-dir ${CMAKE_BINARY_DIR}
    WORKING_DIRECTORY ${CARGO_DIR}
    VERBATIM)
set(LIB ${CMAKE_BINARY_DIR}/${ARCH}/release/${LIBNAME})


# Create library reference for artifact
add_library(${PROJECT_NAME} STATIC IMPORTED GLOBAL)
add_dependencies(${PROJECT_NAME} ${PROJECT_NAME}-cargo)
set_target_properties(
    ${PROJECT_NAME} PROPERTIES
    IMPORTED_LOCATION ${LIB})
