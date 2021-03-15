// Copyright (C) 2021 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: (Apache-2.0 OR MIT)

#[cfg(not(target_arch = "wasm32"))]
pub use tokio::test;


/// Retrieve the address of the HTTP server to use for testing.
pub fn server() -> &'static str {
  option_env!("HTTPC_TEST_SERVER").expect("HTTPC_TEST_SERVER environment variable not found")
}
