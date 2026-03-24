ifeq ($(OS),Windows_NT)
    BINARY_NAME := $(shell powershell -Command "(Get-Content Cargo.toml | Select-String 'name =').ToString().Split('\"')[1]").exe
    CARGO := cargo
    RM := del /Q
    MKDIR := powershell -Command "New-Item -ItemType Directory -Force"
    INSTALL_DIR := $(USERPROFILE)\.local\bin
    EXE := .exe
else
    BINARY_NAME := $(shell grep -m 1 '^name =' Cargo.toml | cut -d '"' -f 2)
    CARGO := cargo
    RM := rm -f
    MKDIR := mkdir -p
    INSTALL_DIR := $(HOME)/.local/bin
    EXE := 
endif

TEST_DIR := tests

.PHONY: all setup build release run clean install help test test-unit test-integration

all: help

setup:
	@echo "Checking toolchain..."
	@$(CARGO) --version || (echo "Error: Rust/Cargo not found." && exit 1)
	@$(MKDIR) $(INSTALL_DIR)
	@echo "Setup complete. Binary path ensured at: $(INSTALL_DIR)"

build:
	@$(CARGO) build

release:
	@$(CARGO) build --release

run:
	@$(CARGO) run --

test:
	@echo "Executing all tests..."
	@$(CARGO) test

clean:
	@$(CARGO) clean
	@echo "Artifacts removed."

install: release
	@echo "Installing $(BINARY_NAME) via cargo..."
	@$(CARGO) install --path .
	@echo "Success: Binary installed in your cargo bin directory."

help:
	@echo "Cross-Platform Management Menu"
	@echo "----------------------------"
	@echo "  setup            Prepare directories (OS-aware)"
	@echo "  build            Standard debug compilation"
	@echo "  release          Optimized release compilation"
	@echo "  run              Execute current build"
	@echo "  test             Run all tests"
	@echo "  clean            Remove build data"
	@echo "  install          Standard Rust installation (Portable)"
	@echo "  help             Show this information"