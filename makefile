# --- Configuration and Variables ---
BINARY_NAME := $(shell grep -m 1 '^name =' Cargo.toml | cut -d '"' -f 2)
CARGO := cargo
VENV := .venv
PIP := $(VENV)/bin/pip

# UI Colors (Using printf to ensure compatibility)
CYAN   := $(shell printf '\033[0;36m')
GREEN  := $(shell printf '\033[0;32m')
RED    := $(shell printf '\033[0;31m')
RESET  := $(shell printf '\033[0m')

.PHONY: all setup build release run clean install help

all: help

setup: ## Create python virtual environment and install Docling
	@echo "$(CYAN)📦 Setting up hybrid environment...$(RESET)"
	python3 -m venv $(VENV)
	$(PIP) install --upgrade pip
	$(PIP) install docling
	$(PIP) install tiktoken
	@echo "$(GREEN)✅ Environment ready.$(RESET)"

build: ## Compile in debug mode
	@echo "$(CYAN)🔨 Building debug binary...$(RESET)"
	@$(CARGO) build

release: ## Compile high-performance binary
	@echo "$(GREEN)🚀 Compiling release binary...$(RESET)"
	@$(CARGO) build --release

run: ## Execute the project
	@$(CARGO) run -- $(filter-all,$(MAKECMDGOALS))

clean: ## Remove build artifacts and virtual environment
	@echo "$(RED)🧹 Cleaning up...$(RESET)"
	@$(CARGO) clean
	rm -rf $(VENV)

install: release ## Install binary to ~/.local/bin
	@echo "$(CYAN)📦 Installing binary...$(RESET)"
	@mkdir -p $(HOME)/.local/bin
	@cp target/release/$(BINARY_NAME) $(HOME)/.local/bin/$(BINARY_NAME)
	@chmod +x $(HOME)/.local/bin/$(BINARY_NAME)
	@echo "$(GREEN)✅ Installed at ~/.local/bin/$(BINARY_NAME)$(RESET)"

help: ## Show this help message
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  $(CYAN)%-15s$(RESET) %s\n", $$1, $$2}'