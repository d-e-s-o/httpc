// Copyright (C) 2021 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: (Apache-2.0 OR MIT)

#[cfg(not(target_arch = "wasm32"))]
pub use hyper;
#[cfg(target_arch = "wasm32")]
pub use web_sys;

mod error;
mod issue;
#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(target_arch = "wasm32")]
mod wasm;

pub use error::Error;
pub use issue::Issue;
#[cfg(not(target_arch = "wasm32"))]
pub use native::Client;
#[cfg(target_arch = "wasm32")]
pub use wasm::Client;
