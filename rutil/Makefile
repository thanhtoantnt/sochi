.PHONY: all discover zucchini tembusu visualizer prophet debug release clean format clippy docs test
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
	@echo "Output files were compiled to the folder: target/debug"

# Build all targets in release mode
release:
	cargo +stable build --release
	@echo "Output files were compiled to the folder: target/release"

# Build core
core:
	cargo +stable build --package core

# Build discover
discover:
	cargo +stable build --package discover
	@echo "Output files were compiled to the folder: target/debug"

# Build visualizer
visualizer:
	cargo +stable build --package visualizer
	@echo "Output files were compiled to the folder: target/debug"

# Build zucchini
zucchini:
	cargo +stable build --package zucchini
	@echo "Output files were compiled to the folder: target/debug"

# Build tembusu
tembusu:
	cargo +stable build --package tembusu
	@echo "Output files were compiled to the folder: target/debug"

# Build prophet
prophet:
	cargo +stable build --package prophet
	@echo ""
	@echo "Output files were compiled to the folder: target/debug"

# Build API documentation in Rustdoc
docs:
	cargo +stable doc --no-deps
	@echo "API docs were generated to: target/doc/core/index.html"
	$(BROWSER) ${MAKEFILE_DIR}target/doc/core/index.html > /dev/null 2>&1 &

# Build project book in Mdbook
book:
	mdbook-mermaid install docs
	mdbook build docs
	@echo "Project docs were generated to: docs/book/index.html"
	$(BROWSER) ${MAKEFILE_DIR}docs/book/index.html > /dev/null 2>&1 &

# Format code
format:
	cargo +stable fmt

# Check source code linting rules
linting: format
	cargo +stable clippy --tests --benches --features linting

# Check source code linting rules of only `discover`
linting-discover:
	cargo +stable clippy --package discover --features linting

# Check source code linting rules of only `inkwell`
linting-inkwell:
	cargo +stable clippy --package discover --features linting

# Check documentation linting rules
linting-docs:
	cargo +stable doc --no-deps --features linting

# Check project book linting rules
linting-book:
	markdownlint docs core zucchini tembusu discover

# Run unit tests
test:
	cargo +stable test --workspace --features linting

# Run unit tests on only `discover`
test-discover:
	cargo +stable test --package discover

# Run unit tests on only `inkwell`
test-inkwell:
	cargo +stable test --package inkwell

# Run unit tests on only `tinyevm`
test-tinyevm:
	cargo +stable test --package tinyevm

# Clean code
clean:
	cargo +stable clean
