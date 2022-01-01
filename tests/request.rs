// Copyright (C) 2021-2022 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: (Apache-2.0 OR MIT)

mod test;

use bytes::Bytes;

use http::header::CONTENT_TYPE;
use http::Method;
use http::Request;
use http::StatusCode;
use http::Uri;

use httpc::Issue as _;

use test::client;
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

  let request = Request::get(uri).body(None).unwrap();
  let client = client();
  let response = client.issue(request).await.unwrap();
  assert_eq!(response.status(), StatusCode::OK);
  assert_eq!(response.body(), "GET success");
}

/// Check that we can properly handle binary data in a response.
#[test]
async fn get_binary() {
  let uri = Uri::builder()
    .scheme("http")
    .authority(server())
    .path_and_query("/get-binary")
    .build()
    .unwrap();

  let request = Request::get(uri).body(None).unwrap();
  let client = client();
  let response = client.issue(request).await.unwrap();
  assert_eq!(response.status(), StatusCode::OK);
  assert_eq!(
    response.body(),
    &Bytes::from_static(b"\x00\x01\x02\x03\x04\x05")
  );
}

/// Check that a request header is handled properly.
#[test]
async fn get_with_request_header() {
  let uri = Uri::builder()
    .scheme("http")
    .authority(server())
    .path_and_query("/get-with-request-header")
    .build()
    .unwrap();

  let request = Request::builder()
    .uri(uri)
    .header(CONTENT_TYPE, "text/plain")
    .body(None)
    .unwrap();
  let client = client();
  let response = client.issue(request).await.unwrap();
  assert_eq!(response.status(), StatusCode::OK);
  assert_eq!(response.body(), &Bytes::from_static(b"text/plain"));
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

  let request = Request::get(uri).body(None).unwrap();
  let client = client();
  let response = client.issue(request).await.unwrap();
  assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

/// Check that we can make a HTTP POST request with a proper body.
#[test]
async fn post_with_body() {
  let uri = Uri::builder()
    .scheme("http")
    .authority(server())
    .path_and_query("/post")
    .build()
    .unwrap();

  let request = Request::builder()
    .method(Method::POST)
    .uri(uri)
    .body(Some("!^*&@42!%*^".into()))
    .unwrap();
  let client = client();
  let response = client.issue(request).await.unwrap();
  assert_eq!(response.status(), StatusCode::OK);
  assert_eq!(response.body(), &Bytes::from_static(b"!^*&@42!%*^"));
}

/// Check that we can make a live request to a site using HTTPS.
#[test]
async fn live_get_with_tls() {
  let uri = Uri::from_static("https://whatwg.org/");
  let request = Request::get(uri).header("Origin", "*").body(None).unwrap();
  let client = client();
  let response = client.issue(request).await.unwrap();
  assert_eq!(response.status(), StatusCode::OK);
}
