cmake_minimum_required(VERSION 3.12)
project(rs-llama-cpp-wrapper)

set(LLAMA_BUILD_EXAMPLES ON CACHE INTERNAL "" FORCE)
add_subdirectory(${CMAKE_CURRENT_SOURCE_DIR}/llama.cpp)

set(LIBRARY_NAME rs-llama-cpp-wrapper)
add_library(${LIBRARY_NAME} STATIC run-inference.h run-inference.cpp rs-llama-cpp-wrapper.h rs-llama-cpp-wrapper.cpp)
target_link_libraries(${LIBRARY_NAME} PRIVATE common llama ${CMAKE_THREAD_LIBS_INIT})
target_include_directories(${LIBRARY_NAME} PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/llama.cpp)