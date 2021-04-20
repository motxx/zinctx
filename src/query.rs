extern crate libc;

use crate::zandbox_const::CONTRACT_QUERY_URL;

use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use std::ffi::{CStr, CString};
use url::Url;

#[no_mangle]
pub extern "C" fn ffi_send_query_request(
  endpoint_url_ptr: *const libc::c_char,
  addres_ptr: *const libc::c_char,
  method_ptr: *const libc::c_char,
  input_json_ptr: *const libc::c_char
) -> *const libc::c_char {
  let endpoint_url = unsafe { CStr::from_ptr(endpoint_url_ptr) }.to_str().unwrap();
  let address = unsafe { CStr::from_ptr(addres_ptr) }.to_str().unwrap();
  let method = unsafe { CStr::from_ptr(method_ptr) }.to_str().unwrap();
  let input_json = unsafe { CStr::from_ptr(input_json_ptr) }.to_str().unwrap();
  let output_json = send_query_request(endpoint_url, address, method, input_json);
  let s = CString::new(output_json).unwrap();
  let p = s.as_ptr();
  std::mem::forget(s);
  p
}

fn send_query_request(endpoint_url: &str, address: &str, method: &str, input_json: &str) -> String {
  let url = Url::parse(endpoint_url).unwrap();
  let url = url.join(format!("{}?address={}&method={}", CONTRACT_QUERY_URL, address, method).as_str()).unwrap();
  let url = url.as_str();

  let mut headers = HeaderMap::new();
  headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

  let body = String::from(input_json);

  let client = reqwest::blocking::Client::new();
  let res = client.put(url)
    .headers(headers)
    .body(body)
    .send()
    .expect("cannot create http request");

  if res.status().is_success() {
    res.text().expect("cannot read http response")
  }
  else {
    "".to_string()
  }
}
