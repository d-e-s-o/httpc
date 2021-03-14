// Copyright (C) 2021 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: (Apache-2.0 OR MIT)

#[cfg(not(target_arch = "wasm32"))]
pub use tokio::test;
#[cfg(target_arch = "wasm32")]
pub use wasm_bindgen_test::wasm_bindgen_test as test;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::wasm_bindgen_test_configure;
// By default wasm-bindgen-test attempts to run in Node.js. That's no
// good for us, because it does not have a "window" there. So tell it to
// actually run in a (headless) browser.
#[cfg(target_arch = "wasm32")]
wasm_bindgen_test_configure!(run_in_browser);


/// Retrieve the address of the HTTP server to use for testing.
pub fn server() -> &'static str {
  option_env!("HTTPC_TEST_SERVER").expect("HTTPC_TEST_SERVER environment variable not found")
}
