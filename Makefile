.PHONY: all sochi debug release clean format clippy docs test
.DEFAULT_GOAL := all

########################################################
## Global variables

ifeq ($(OS),Windows_NT)
    BROWSER := start
else
    UNAME := $(shell uname -s)
    ifeq ($(UNAME),Linux)
        BROWSER := sensible-browser
    endif
endif

MAKEFILE_PATH := $(abspath $(lastword $(MAKEFILE_LIST)))
MAKEFILE_DIR := $(dir $(MAKEFILE_PATH))

########################################################
## Make' targets

all: debug

# Check code formatting, linting, and build all targets.
# This is useful to check the code base before committing.
precommit: format linting debug

# Build all targets in debug mode.
debug:
	cargo +stable build
	cp target/debug/sochi ./sochi -f
	@echo "Output files were compiled to the folder: target/debug"

# Build all targets in release mode
release:
	cargo +stable build --release
	cp target/release/sochi ./sochi -f
	@echo "Output files were compiled to the folder: target/release"

# Build sochi
sochi:
	cargo +stable build --package sochi
	cp target/debug/sochi ./sochi -f
	@echo "Output files were compiled to the folder: target/debug"

# Format code
format:
	cargo +stable fmt

# Check source code linting rules
linting: format
	cargo +stable clippy --tests --benches --features linting

# Run unit tests
test:
	cargo +stable test --workspace --features linting

# Update packages
update:
	cargo +stable update

# Clean code
clean:
	cargo +stable clean
