// Copyright (C) 2021 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use bytes::Bytes;

use http::Request;
use http::Response;

use hyper::body::to_bytes;
use hyper::body::Body;
use hyper::client::connect::HttpConnector;
use hyper::Client as HyperClient;

use crate::Error;


/// An HTTP client for native usage.
// TODO: We should support other connectors to enable HTTPS usage.
#[derive(Debug)]
pub struct Client(HyperClient<HttpConnector, Body>);

impl Client {
  /// Create a new "native" HTTP client.
  pub fn new() -> Self {
    Self(HyperClient::new())
  }

  /// Issue a request and retrieve a response.
  // TODO: Need to support bodies other than `()`.
  pub async fn request(&self, request: Request<()>) -> Result<Response<Bytes>, Error> {
    let (parts, ()) = request.into_parts();
    let request = Request::from_parts(parts, Body::default());

    let response = HyperClient::request(&self.0, request).await?;
    let (parts, body) = response.into_parts();
    let bytes = to_bytes(body).await?;
    Ok(Response::from_parts(parts, bytes))
  }
}
