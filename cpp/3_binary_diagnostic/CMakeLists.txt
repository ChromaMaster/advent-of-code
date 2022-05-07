cmake_minimum_required(VERSION 3.21)
project(3_binary_iagnostic)

set(CMAKE_CXX_STANDARD 17)

add_compile_options(-Werror -Wall -Wextra -Wpedantic)

add_executable(3_binary_iagnostic main.cpp)
