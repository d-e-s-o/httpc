[![pipeline](https://gitlab.com/d-e-s-o/httpc/badges/master/pipeline.svg)](https://gitlab.com/d-e-s-o/httpc/commits/master)
[![crates.io](https://img.shields.io/crates/v/httpc.svg)](https://crates.io/crates/httpc)
[![Docs](https://docs.rs/httpc/badge.svg)](https://docs.rs/httpc)
[![rustc](https://img.shields.io/badge/rustc-1.46+-blue.svg)](https://blog.rust-lang.org/2020/08/27/Rust-1.46.0.html)

httpc
=====

- [Documentation][docs-rs]
- [Changelog](CHANGELOG.md)

**httpc** is a Rust library crate providing an HTTP client that works
both natively as well as in a bare bones WASM environment as used in
web browsers. That is to say, the client code looks exactly the same and
all that is needed to switch between the two is to compile for a
different target platform (`wasm32-unknown-unknown`).

When compiling natively, the crate relies on [`hyper`][hyper] as the
underlying HTTP client. When targeting WASM it works directly with the
[Fetch API][fetch-api] as exposed through [`web-sys`][web-sys].

The crate does not yet provide functionality for all potential HTTP
client use cases, though that is mostly a matter of implementing it: the
interface should be able to cater to most use cases.

One of the main pain points of making HTTP requests in a certain
application from a WASM environment is arguably testing it properly.
**httpc** comes with a test suite covering supported functionality and
it includes all the necessary setup code to make it run both natively as
well as when targeting WASM.


[docs-rs]: https://docs.rs/crate/httpc
[hyper]: https://crates.io/crates/hyper
[fetch-api]: https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API
[web-sys]: https://crates.io/crates/web-sys
