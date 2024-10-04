# Makefile for Linux Command Search Project

# Compiler settings
CC = gcc
CFLAGS = -Wall -Wextra -std=c11

# Directories
SRC_DIR = src
BUILD_DIR = build

# Source and object files
SRC = $(SRC_DIR)/main.c
OBJ = $(BUILD_DIR)/main.o

# Binary name
TARGET = commands

# Default target
all: $(TARGET)

# Create build directory
$(BUILD_DIR):
	mkdir -p $(BUILD_DIR)

# Compile source files
$(BUILD_DIR)/%.o: $(SRC_DIR)/%.c | $(BUILD_DIR)
	$(CC) $(CFLAGS) -c $< -o $@

# Link object files
$(TARGET): $(OBJ)
	$(CC) $(CFLAGS) $^ -o $@

# Clean build files
clean:
	rm -rf $(BUILD_DIR) $(TARGET)

# Install
install: $(TARGET)
	cp $(TARGET) /usr/local/bin

# Uninstall
uninstall:
	rm -f /usr/local/bin/$(TARGET)

# Phony targets
.PHONY: all clean install uninstall
