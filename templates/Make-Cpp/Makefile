# Compiler
CXX = g++
# Compiler flags
CXXFLAGS = -std=c++11 -Wall

# Target executable
TARGET = MyExecutable
# Source files
SRCS = main.cpp

# Build the target
$(TARGET): $(SRCS)
	$(CXX) $(CXXFLAGS) -o $(TARGET) $(SRCS)

# Clean target
clean:
	rm -f $(TARGET)
