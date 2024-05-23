# Copyright (C) 2024 Sidharth Rao and Ethan Uppal. All rights reserved.

BUILD	:= debug

.PHONY: build
build:
	@echo '[INFO] Building project'
	@cargo build
	@echo './target/$(BUILD)/main "$$@"' > ./main
	@chmod u+x ./main
	@mv ./main ..

.PHONY: test
test: build
	@echo '[INFO] Running tests'
	@cargo nextest run --features disable_color

.PHONY: deps
deps:
	@echo '[INFO] Installing dependencies'
	@cargo install cargo-nextest
	@curl -LsSf https://insta.rs/install.sh | sh
	@cargo install bevy

.PHONY: clean
clean:
	@echo '[INFO] Removing build files'
	@cargo clean

.PHONY: docs
docs:
	@echo '[INFO] Building and viewing documentation'
	@cargo doc --no-deps --open
