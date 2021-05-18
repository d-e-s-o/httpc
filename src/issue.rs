// Copyright (C) 2021 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use async_trait::async_trait;

use bytes::Bytes;

use http::Request;
use http::Response;

use crate::Error;


/// A trait for issuing HTTP requests.
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
pub trait Issue {
  /// Issue a request and retrieve a response.
  async fn issue(&self, request: Request<Option<String>>) -> Result<Response<Bytes>, Error>;
}
