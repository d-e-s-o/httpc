// Copyright (C) 2021 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: (Apache-2.0 OR MIT)

mod test;

use http::Request;
use http::StatusCode;
use http::Uri;

use httpc::Client;

use test::server;
use test::test;


/// Check that performing an HTTP GET request works.
#[test]
async fn get_ok() {
  let uri = Uri::builder()
    .scheme("http")
    .authority(server())
    .path_and_query("/get")
    .build()
    .unwrap();

  let request = Request::get(uri).body(()).unwrap();
  let client = Client::new();
  let response = client.request(request).await.unwrap();
  assert_eq!(response.status(), StatusCode::OK);
  assert_eq!(response.body(), "GET success");
}

/// Check that a NOT_FOUND error is propagated up properly.
#[test]
async fn get_not_found() {
  let uri = Uri::builder()
    .scheme("http")
    .authority(server())
    .path_and_query("/not-found")
    .build()
    .unwrap();

  let request = Request::get(uri).body(()).unwrap();
  let client = Client::new();
  let response = client.request(request).await.unwrap();
  assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
