# Copyright (C) 2021 Daniel Mueller <deso@posteo.net>
# SPDX-License-Identifier: (Apache-2.0 OR MIT)

TMPDIR := $(patsubst %/,%,$(if $(TMPDIR),$(TMPDIR),/tmp))
WASMDIR := $(TMPDIR)/wasm-bindgen-test-runner
WASMTAR := $(WASMDIR)/wasm-bindgen.tar.gz
VERSION := $(WASMDIR)/version


# Run the test suite, using the locally installed
# wasm-bindgen-test-runner if available (or assuming a system-installed
# one if not).
.PHONY: test
test:
	@PATH="$(WASMDIR)/bin:$(PATH)" bash test.sh


# Ensure that a suitable wasm-bindgen-test-runner binary is available
# in the temporary directory.
.PHONY: test-runner
test-runner: CLIDIR := $(WASMDIR)/wasm-bindgen-$$(cat $(VERSION))/crates/cli/
test-runner: $(WASMTAR)
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
