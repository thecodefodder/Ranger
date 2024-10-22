# Compiler
CXX = g++
# Compiler flags
CXXFLAGS = -std=c++11 -Wall

# Target executable
TARGET = ${EXECUTABLE_NAME}
# Source files
SRCS = src/main.cpp

# Build the target
all: $(TARGET)

$(TARGET): $(SRCS)
	$(CXX) $(CXXFLAGS) -o $@ $^

# Clean target
clean:
	rm -f $(TARGET)