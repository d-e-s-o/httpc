// Copyright (C) 2021 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use async_trait::async_trait;

use bytes::Bytes;

use http::response::Builder;
use http::Request;
use http::Response;
use http::StatusCode;

use js_sys::ArrayBuffer;
use js_sys::DataView;

use wasm_bindgen::JsCast as _;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;

use web_sys::window;
use web_sys::Headers;
use web_sys::Request as WebRequest;
use web_sys::RequestInit;
use web_sys::RequestMode;
use web_sys::Response as WebResponse;
use web_sys::Window;

use crate::Error;
use crate::Issue;


/// Convert an `http::Request` into one as used by the Fetch API.
fn into_web_request(request: Request<Option<String>>) -> Result<WebRequest, Error> {
  let (parts, body) = request.into_parts();
  let headers = Headers::new().map_err(|err| Error::web("failed to create Headers object", err))?;
  let headers =
    parts
      .headers
      .iter()
      .try_fold::<_, _, Result<_, Error>>(headers, |headers, (k, v)| {
        let _ = headers.append(k.as_str(), v.to_str()?);
        Ok(headers)
      })?;

  let value;
  let body = if let Some(body) = body {
    value = JsValue::from_str(&body);
    Some(&value)
  } else {
    None
  };
  let uri = parts.uri;

  let mut opts = RequestInit::new();
  opts.mode(RequestMode::Cors);
  opts.method(parts.method.as_str());
  opts.headers(&headers);
  opts.body(body);

  let request = WebRequest::new_with_str_and_init(&uri.to_string(), &opts).map_err(|err| {
    Error::web(
      format!("failed to create GET request for {}", uri.to_string()),
      err,
    )
  })?;

  Ok(request)
}


/// Create a `http::Response` from one produced by the Fetch API.
async fn into_http_response(response: WebResponse) -> Result<Response<Bytes>, Error> {
  let status = response.status();
  let status = StatusCode::from_u16(status)?;

  // TODO: It is conceivable that using a `ReadableStream` through the
  //       `body` method may be a better way, but it appears that the
  //       stream API is not yet available.
  let buffer = response
    .array_buffer()
    .map_err(|err| Error::web("failed to read HTTP body as ArrayBuffer", err))?;
  let buffer = JsFuture::from(buffer)
    .await
    .map_err(|err| Error::web("failed to retrieve HTTP body from response", err))?;
  let buffer = buffer
    .dyn_into::<ArrayBuffer>()
    .map_err(|err| Error::web("future did not resolve into an js-sys ArrayBuffer", err))?;
  let length = buffer.byte_length() as usize;

  let data_view = DataView::new(&buffer, 0, length);
  let body = (0..length).fold(Vec::with_capacity(length), |mut body, i| {
    body.push(data_view.get_uint8(i));
    body
  });
  let bytes = Bytes::from(body);

  // TODO: We should also set headers and various other fields.
  let response = Builder::new().status(status).body(bytes)?;
  Ok(response)
}

async fn request(
  client: &Window,
  request: Request<Option<String>>,
) -> Result<Response<Bytes>, Error> {
  let request = into_web_request(request)?;
  let response = JsFuture::from(client.fetch_with_request(&request))
    .await
    .map_err(|err| Error::web("failed to issue GET request", err))?;
  let response = response
    .dyn_into::<WebResponse>()
    .map_err(|err| Error::web("future did not resolve into a web-sys Response", err))?;

  into_http_response(response).await
}


/// An HTTP client for usage in WASM environments.
#[derive(Debug)]
#[deprecated(note = "use Issue trait instead")]
pub struct Client(Window);

impl Client {
  /// Create a new WASM HTTP client.
  pub fn new() -> Self {
    let window = window().expect("no window found; not running inside a browser?");
    Self(window)
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

impl From<Window> for Client {
  /// Create a `Client` from a `Window`.
  fn from(window: Window) -> Self {
    Self(window)
  }
}

impl Into<Window> for Client {
  /// Extract the `Window` from a `Client`.
  fn into(self) -> Window {
    self.0
  }
}

#[async_trait(?Send)]
impl Issue for Window {
  async fn issue(&self, request: Request<Option<String>>) -> Result<Response<Bytes>, Error> {
    self::request(&self, request).await
  }
}
