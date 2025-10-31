// Copyright (C) 2025 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: (Apache-2.0 OR MIT)

// See https://github.com/wasm-bindgen/wasm-bindgen/blob/018641b4dd6752d6d27910661f80e6646f776ce2/crates/cli/src/bin/wasm-bindgen-test-runner.rs
#[cfg(not(target_arch = "wasm32"))]
fn main() -> anyhow::Result<()> {
  use std::env;
  use wasm_bindgen_cli::wasm_bindgen_test_runner;

  wasm_bindgen_test_runner::run_cli_with_args(env::args_os())
}

// Cargo may try to compile this example and bails out if there is no
// main function. So stub it out.
#[cfg(target_arch = "wasm32")]
fn main() {}
