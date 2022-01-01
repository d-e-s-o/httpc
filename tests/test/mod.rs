// Copyright (C) 2021-2022 Daniel Mueller <deso@posteo.net>
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


/// Create a `hyper` client.
#[cfg(not(target_arch = "wasm32"))]
pub fn client() -> hyper::Client<
  hyper_tls::HttpsConnector<hyper::client::connect::HttpConnector>,
  hyper::body::Body,
> {
  let https = hyper_tls::HttpsConnector::new();
  hyper::Client::builder().build(https)
}

/// Create a WASM client.
#[cfg(target_arch = "wasm32")]
pub fn client() -> web_sys::Window {
  web_sys::window().expect("no window found; not running inside a browser?")
}

/// Retrieve the address of the HTTP server to use for testing.
pub fn server() -> &'static str {
  option_env!("HTTPC_TEST_SERVER").expect("HTTPC_TEST_SERVER environment variable not found")
}
