# Copyright (C) 2024 Sidharth Rao and Ethan Uppal. All rights reserved.

BUILD	:= debug

.PHONY: build
build:
	@echo '[INFO] Building project'
	@cargo build
	@echo './target/$(BUILD)/trivial_quest "$$@"' > ./main
	@chmod u+x ./main

.PHONY: run
run:
	@echo '[INFO] Building and running project'
	@cargo build
	@echo './target/$(BUILD)/trivial_quest "$$@"' > ./main
	@chmod u+x ./main
	@./main

.PHONY: test
test: build
	@echo '[INFO] Running tests'
	@cargo nextest run --features disable_color

.PHONY: deps
deps:
	@echo '[INFO] Installing dependencies'
	@cargo install cargo-nextest
	@curl -LsSf https://insta.rs/install.sh | sh

.PHONY: clean
clean:
	@echo '[INFO] Removing build files'
	@cargo clean

.PHONY: docs
docs:
	@echo '[INFO] Building and viewing documentation'
	@cargo doc --no-deps --open

.PHONY: format fmt
format fmt:
	@echo '[INFO] Formatting'
	@cargo +nightly fmt
	@echo '[INFO] Done'
