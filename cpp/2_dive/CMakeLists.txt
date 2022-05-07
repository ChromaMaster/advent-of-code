cmake_minimum_required(VERSION 3.21)
project(2_dive)

set(CMAKE_CXX_STANDARD 17)

add_compile_options(-Werror -Wall -Wextra -Wpedantic)

add_executable(2_dive main.cpp)
