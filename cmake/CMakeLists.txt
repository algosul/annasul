cmake_minimum_required(VERSION 3.28)
project(annasul)

set(CMAKE_C_STANDARD 11)               # 使用C11标准
find_package(PkgConfig REQUIRED)       # 依赖PkgConfig
pkg_check_modules(GTK4 REQUIRED gtk4)  # 查找GTK4库[1,3](@ref)

include_directories(${GTK4_INCLUDE_DIRS})  # 添加头文件路径
link_directories(${GTK4_LIBRARY_DIRS})     # 添加库文件路径

add_executable(test_gtk4 main.cpp)
target_link_libraries(test_gtk4 ${GTK4_LIBRARIES})  # 链接GTK4库