cmake_minimum_required(VERSION 3.18)
project(4_giant_squid)

set(CMAKE_CXX_STANDARD 17)
add_compile_options(-Werror -Wall -Wextra -Wpedantic)

########################################################################################################################
########################################################################################################################

add_library(DocTest INTERFACE)
target_include_directories(DocTest INTERFACE include)

include_directories(src)

set(_SRCS src/*)
set(_SRCS_TEST src/*)

add_executable(4_giant_squid main.cpp ${_SRCS} ${_HEADERS})

add_executable(4_giant_squid_test main_test.cpp ${_SRCS} ${_SRCS_TEST} ${_HEADERS})
target_link_libraries(4_giant_squid_test DocTest)