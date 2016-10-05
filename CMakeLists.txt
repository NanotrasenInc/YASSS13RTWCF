cmake_minimum_required(VERSION 3.0.0)
project(SS13)

set(CMAKE_MODULE_PATH "${CMAKE_SOURCE_DIR}/cmake_modules" ${CMAKE_MODULE_PATH})

set(SS13_VERSION_MAJOR 0)
set(SS13_VERSION_MINOR 0)
set(SS13_VERSION_PATCH 0)

configure_file(
  "${PROJECT_SOURCE_DIR}/src/config.hpp.in"
  "${PROJECT_BINARY_DIR}/config.hpp"
)

include_directories("${PROJECT_BINARY_DIR}" "${PROJECT_SOURCE_DIR}/src")

set(SFML_ROOT ${SFML_ROOT} "${PROJECT_SOURCE_DIR}/sfml")
find_package(SFML 2 REQUIRED system window graphics network audio)
if(SFML_FOUND)
  include_directories(${SFML_INCLUDE_DIR})
  link_libraries(${SFML_LIBRARIES})
endif()

add_subdirectory(src)
