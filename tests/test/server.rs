// Copyright (C) 2021 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: (Apache-2.0 OR MIT)

#[cfg(not(target_arch = "wasm32"))]
fn main() {
  use bytes::Bytes;

  use http::Response;
  use http::StatusCode;

  use warp::any;
  use warp::cors;
  use warp::path;
  use warp::reply::reply;
  use warp::reply::with_status;
  use warp::serve;
  use warp::Filter as _;

  use tokio::runtime::Builder;

  let get = path("get").map(|| "GET success");
  let get_binary = path("get-binary").map(|| {
    Response::builder()
      .status(StatusCode::OK)
      .body(Bytes::from_static(b"\x00\x01\x02\x03\x04\x05"))
      .unwrap()
  });
  let reject = any().map(|| with_status(reply(), StatusCode::NOT_FOUND));
  let routes = get
    .or(get_binary)
    .or(reject)
    .with(cors().allow_any_origin());

  let rt = Builder::new_multi_thread().enable_io().build().unwrap();
  rt.block_on(async move {
    let (addr, serve) = serve(routes).bind_ephemeral(([127, 0, 0, 1], 0));
    println!("{}", addr);
    serve.await
  })
}

// Clever wasm-bindgen-test tries to compile this example when it
// shouldn't and bails out if there is no main function. So stub it out.
#[cfg(target_arch = "wasm32")]
fn main() {}
