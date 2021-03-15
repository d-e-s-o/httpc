#!/bin/bash

# Copyright (C) 2021 Daniel Mueller <deso@posteo.net>
# SPDX-License-Identifier: (Apache-2.0 OR MIT)

set -e -u -o pipefail

# Start the test-server example in the back ground, providing its output
# as file descriptor 3.
exec 3< <(cargo run --example test-server --quiet -- --nocapture)
# Wait for and read the first line of output, which is the address we
# serve on, and store it in HTTPC_TEST_SERVER.
read -r HTTPC_TEST_SERVER <&3;
# Now run the actual test suite.
HTTPC_TEST_SERVER="${HTTPC_TEST_SERVER}" cargo test

kill $!
