cmake_minimum_required(VERSION 3.18)
project(1_sonar_sweeper)

set(CMAKE_CXX_STANDARD 17)

add_compile_options(-Werror -Wall -Wextra -Wpedantic)

add_executable(1_sonar_sweeper main.cpp)
