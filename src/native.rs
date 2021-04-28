// Copyright (C) 2021 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use async_trait::async_trait;

use bytes::Bytes;

use http::Request;
use http::Response;

use hyper::body::to_bytes;
use hyper::body::Body;
use hyper::client::connect::Connect;
use hyper::client::connect::HttpConnector;
use hyper::Client as HyperClient;

use crate::Error;
use crate::Issue;


/// Issue a request and retrieve a response.
async fn request<C>(
  client: &HyperClient<C>,
  request: Request<Option<String>>,
) -> Result<Response<Bytes>, Error>
where
  C: Connect + Clone + Send + Sync + 'static,
{
  let (parts, body) = request.into_parts();
  let body = if let Some(body) = body {
    Body::from(body)
  } else {
    Body::empty()
  };
  let request = Request::from_parts(parts, body);

  let response = HyperClient::request(client, request).await?;
  let (parts, body) = response.into_parts();
  let bytes = to_bytes(body).await?;
  Ok(Response::from_parts(parts, bytes))
}


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
  pub async fn request(&self, request: Request<Option<String>>) -> Result<Response<Bytes>, Error> {
    self::request(&self.0, request).await
  }
}

impl Default for Client {
  fn default() -> Self {
    Self::new()
  }
}

impl From<HyperClient<HttpConnector, Body>> for Client {
  /// Create a `Client` from a `hyper::Client`.
  fn from(client: HyperClient<HttpConnector, Body>) -> Self {
    Self(client)
  }
}

impl Into<HyperClient<HttpConnector, Body>> for Client {
  /// Extract the `hyper::Client` from a `Client`.
  fn into(self) -> HyperClient<HttpConnector, Body> {
    self.0
  }
}

#[async_trait(?Send)]
impl<C> Issue for HyperClient<C>
where
  C: Connect + Clone + Send + Sync + 'static,
{
  async fn issue(&self, request: Request<Option<String>>) -> Result<Response<Bytes>, Error> {
    self::request(&self, request).await
  }
}
