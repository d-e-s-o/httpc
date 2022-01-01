# Copyright (C) 2021-2022 Daniel Mueller <deso@posteo.net>
# SPDX-License-Identifier: (Apache-2.0 OR MIT)

TMPDIR := $(patsubst %/,%,$(if $(TMPDIR),$(TMPDIR),/tmp))
WASMDIR := $(TMPDIR)/wasm-bindgen-test-runner
WASMTAR := $(WASMDIR)/wasm-bindgen.tar.gz
VERSION := $(WASMDIR)/version
TEST_RUNNER := $(WASMDIR)/bin/wasm-bindgen-test-runner


.PHONY: test
test: test-native test-wasm

.PHONY: test-native
test-native:
	@bash test-target.sh

# Run the test suite, using the locally installed
# wasm-bindgen-test-runner if available (or assuming a system-installed
# one if not).
# Note that in order to debug issues, you can run with NO_HEADLESS=1.
.PHONY: test-wasm
test-wasm:
	@PATH="$(WASMDIR)/bin:$(PATH)" bash test-target.sh --target=wasm32-unknown-unknown


# Ensure that a suitable wasm-bindgen-test-runner binary is available
# in the temporary directory.
.PHONY: test-runner
test-runner: $(TEST_RUNNER)

$(TEST_RUNNER): CLIDIR := $(WASMDIR)/wasm-bindgen-$$(cat $(VERSION))/crates/cli/
$(TEST_RUNNER): $(WASMTAR)
	tar --directory $(WASMDIR) --extract --gzip --file $<
	cargo install --quiet --path $(CLIDIR) --root $(WASMDIR) --bin=wasm-bindgen-test-runner


# Ensure that we have a tar archive containing the source code of the
# desired wasm-bindgen version.
$(WASMTAR): $(VERSION)
	wget "https://github.com/rustwasm/wasm-bindgen/archive/$$(cat $(VERSION)).tar.gz" \
		--quiet --output-document $@


# Ensure that a file is available that contains the wasm-bindgen version
# we use.
$(VERSION): Cargo.lock
	mkdir -p $(WASMDIR)
	grep 'name = "wasm-bindgen"' $< -A1 | \
		grep 'version' | \
		sed 's@[^"]*"\([^"]\+\)".*@\1@' > $@


# Ensure that we have a Cargo.lock file available.
Cargo.lock:
	cargo update --quiet --dry-run
