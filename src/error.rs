// Copyright (C) 2021 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use thiserror::Error as ThisError;


/// Error type for issues encountered when issuing a request.
#[derive(Debug, ThisError)]
pub enum Error {
  /// An error as used by the hyper crate.
  #[cfg(not(target_arch = "wasm32"))]
  #[error("the hyper crate reported an error")]
  Hyper(
    #[from]
    #[source]
    hyper::Error,
  ),
}


#[cfg(test)]
mod tests {
  use super::*;


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
}
