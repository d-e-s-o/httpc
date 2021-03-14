// Copyright (C) 2021 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use thiserror::Error as ThisError;


/// Error type for issues encountered when issuing a request.
#[derive(Debug, ThisError)]
pub enum Error {
  #[cfg(target_arch = "wasm32")]
  #[error("an HTTP error occurred")]
  Http(
    #[from]
    #[source]
    http::Error,
  ),
  /// An error as used by the hyper crate.
  #[cfg(not(target_arch = "wasm32"))]
  #[error("the hyper crate reported an error")]
  Hyper(
    #[from]
    #[source]
    hyper::Error,
  ),
  #[cfg(target_arch = "wasm32")]
  #[error("an invalid HTTP status was received")]
  InvalidStatusCode(
    #[from]
    #[source]
    http::status::InvalidStatusCode,
  ),
  #[cfg(target_arch = "wasm32")]
  #[error("{context}")]
  WebSys {
    /// Some crate-provided context to the error.
    context: String,
    /// The originally reported `JsValue` in some textual form.
    // We do not keep the `JsValue` around because they are a pain to
    // work with (just extracting something useful) and they case
    // everything they touch to be not `Send`.
    #[source]
    source: WebError,
  },
}

#[cfg(target_arch = "wasm32")]
#[derive(Debug, ThisError)]
#[error("{0}")]
pub struct WebError(String);


#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;

#[cfg(target_arch = "wasm32")]
impl Error {
  /// Create a new `Error::WebSys` variant.
  pub(crate) fn web<S>(context: S, error: JsValue) -> Error
  where
    S: Into<String>,
  {
    let error = if let Some(error) = error.as_string() {
      error
    } else {
      format!("{:?}", error)
    };

    Self::WebSys {
      context: context.into(),
      source: WebError(error),
    }
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[cfg(target_arch = "wasm32")]
  use wasm_bindgen_test::wasm_bindgen_test as test;


  /// Ensure that our `Error` type implements `Send` and `Sync`.
  #[test]
  fn ensure_error_trait_impls() {
    fn err() -> Error {
      unimplemented!()
    }

    if false {
      &err() as &(dyn Send + Sync);
    }
  }


  #[cfg(target_arch = "wasm32")]
  mod wasm {
    use super::test;
    use super::*;

    use std::error::Error as _;


    #[test]
    fn web_sys_variant_formatting() {
      let source = JsValue::from_str("the one and only source");
      let error = Error::web("error context", source);

      assert_eq!(error.to_string(), "error context");
      assert_eq!(
        error.source().unwrap().to_string(),
        "the one and only source"
      );
    }
  }
}
