cmake_minimum_required(VERSION 3.28)
project(annasul)

set(CMAKE_C_STANDARD 11)
find_package(PkgConfig REQUIRED)
pkg_check_modules(GTK4 REQUIRED gtk4)

include_directories(${GTK4_INCLUDE_DIRS})
link_directories(${GTK4_LIBRARY_DIRS})

add_executable(test_gtk4 main.cpp)
target_link_libraries(test_gtk4 ${GTK4_LIBRARIES})