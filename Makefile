CARGO = cargo
CARGO_TEST = cargo nextest run
# CARGO_TEST = cargo test
OCTEZ_SMART_ROLLUP_WASM_DEBUGGER = octez-smart-rollup-wasm-debugger



.PHONY:
all: build

.PHONY: fmt
fmt:
	$(CARGO) +nightly fmt

.PHONY: clean
clean:
	$(CARGO) clean

.PHONY: test-release
test-release:
	$(CARGO_TEST) --release

.PHONY: build
build:
	$(CARGO) build --release --target wasm32-unknown-unknown
