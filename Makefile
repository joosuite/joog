.DEFAULT_GOAL := build

CARGO ?= $(if $(wildcard $(HOME)/.cargo/bin/cargo),$(HOME)/.cargo/bin/cargo,cargo)
NODE ?= node --experimental-strip-types
BIN_DIR := $(CURDIR)/bin
INSTALL_BIN ?= $(if $(CARGO_INSTALL_ROOT),$(CARGO_INSTALL_ROOT),$(if $(CARGO_HOME),$(CARGO_HOME),$(HOME)/.cargo))/bin
EXE := $(if $(filter Windows_NT,$(OS)),.exe,)

.PHONY: build install joog fmt fmt-check lint test live-test live-cleanup live-coverage ci audit clean schemas reference tooling-check

schemas:
	$(NODE) scripts/generate_cli_schemas.ts

reference: schemas
	$(CARGO) build --locked
	$(NODE) scripts/generate_cli_reference.ts
	$(NODE) scripts/generate_wrapper_reference.ts

tooling-check:
	npm run typecheck
	$(NODE) scripts/generate_cli_schemas.ts --check
	$(NODE) scripts/generate_wrapper_reference.ts --check

build:
	$(CARGO) build --release --locked
	@mkdir -p $(BIN_DIR)
	cp target/release/joog$(EXE) $(BIN_DIR)/joog$(EXE)
	$(CARGO) run --release --locked --example install_wrappers -- --directory "$(BIN_DIR)"

install:
	$(CARGO) install --path . --locked
	$(CARGO) run --release --locked --example install_wrappers -- --directory "$(INSTALL_BIN)"

joog:
	$(CARGO) run --locked -- $(ARGS)

fmt:
	$(CARGO) fmt --all

fmt-check:
	$(CARGO) fmt --all -- --check

lint:
	$(CARGO) clippy --locked --all-targets --all-features -- -D warnings

test:
	$(CARGO) test --locked

live-test:
	$(CARGO) test --locked --features live-test-diagnostics --test live full_account_coverage -- --ignored --nocapture --test-threads=1

live-cleanup:
	$(CARGO) test --locked --features live-test-diagnostics --test live cleanup_recorded_resources -- --ignored --nocapture --test-threads=1

live-coverage:
	$(CARGO) test --locked --test live recorded_command_coverage -- --ignored --nocapture

ci: fmt-check lint test

audit:
	$(CARGO) audit --deny warnings

clean:
	$(CARGO) clean
