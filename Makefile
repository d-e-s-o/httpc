# Copyright (C) 2021-2025 Daniel Mueller <deso@posteo.net>
# SPDX-License-Identifier: (Apache-2.0 OR MIT)

TMPDIR := $(patsubst %/,%,$(if $(TMPDIR),$(TMPDIR),/tmp))


.PHONY: test
test: test-native test-wasm

.PHONY: test-native
test-native:
	@bash test-target.sh

# Run the test suite. Note that in order to debug issues, you can run
# with NO_HEADLESS=1.
.PHONY: test-wasm
test-wasm:
	@bash test-target.sh --target=wasm32-unknown-unknown
