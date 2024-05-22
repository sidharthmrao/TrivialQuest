# Copyright (C) 2024 Sidharth Rao and Ethan Uppal. All rights reserved.

BUILD	:= debug

.PHONY: build
build:
	@echo '[INFO] Building project'
	@cd rust; cargo build
	@cd rust; echo './rust/target/$(BUILD)/main "$$@"' > ./main
	@cd rust; chmod u+x ./main
	@cd rust; mv ./main ..

.PHONY: test
test: build
	@echo '[INFO] Running tests'
	@cd rust; cargo nextest run --features disable_color

.PHONY: deps
deps:
	@echo '[INFO] Installing dependencies'
	@cd rust; cargo install cargo-nextest
	@curl -LsSf https://insta.rs/install.sh | sh

.PHONY: clean
clean:
	@echo '[INFO] Removing build files'
	@cd rust; cargo clean

.PHONY: docs
docs:
	@echo '[INFO] Building and viewing documentation'
	@cd rust; cargo doc --no-deps --open
