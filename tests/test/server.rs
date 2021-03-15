// Copyright (C) 2021 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use warp::path;
use warp::serve;
use warp::Filter as _;

use tokio::runtime::Builder;

fn main() {
  let get = path("get").map(|| "GET success");
  let routes = get;

  let rt = Builder::new_multi_thread().enable_io().build().unwrap();
  rt.block_on(async move {
    let (addr, serve) = serve(routes).bind_ephemeral(([127, 0, 0, 1], 0));
    println!("{}", addr);
    serve.await
  })
}
