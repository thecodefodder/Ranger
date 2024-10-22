cmake_minimum_required(VERSION 3.10)

# Set the project name
project(${PROJECT_NAME} LANGUAGES CXX)

# Specify the C++ standard
set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED True)

# Add the executable target
add_executable(${EXECUTABLE_NAME} src/main.cpp)