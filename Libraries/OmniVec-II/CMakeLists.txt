cmake_minimum_required(VERSION 3.20)
project(OmniVecII LANGUAGES CXX)

add_library(omnivec src/vector.cpp)

target_include_directories(omnivec PUBLIC include)
target_compile_features(omnivec PUBLIC cxx_std_20)
target_compile_options(omnivec PRIVATE -Wall -Wextra -Wpedantic)

enable_testing()

add_executable(omnivec-test tests/vector_test.cpp)
target_link_libraries(omnivec-test PRIVATE omnivec)

add_test(NAME omnivec-unit COMMAND omnivec-test)
