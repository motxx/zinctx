extern crate libc;
extern crate protobuf;

use protobuf::Message;
use std::ffi::{CStr, CString};

use crate::protos::zinctx::{QueryRequest, QueryResponse};

#[no_mangle]
pub extern "C" fn ffi_send_query_request(endpoint_url_ptr: *const libc::c_char, proto_ptr: *const libc::c_char) -> *const libc::c_char {
    let endpoint_url = unsafe { CStr::from_ptr(endpoint_url_ptr) }.to_str().unwrap();
    let proto_slice = unsafe { CStr::from_ptr(proto_ptr) };
    let proto = QueryRequest::parse_from_bytes(proto_slice.to_bytes()).unwrap();
    let res = send_query_request(endpoint_url, proto);
    let s = CString::new(res.write_to_bytes().unwrap()).unwrap();
    let p = s.as_ptr();
    std::mem::forget(s);
    p
}

// TODO: コントラクト別の実装を隠蔽
use protobuf::well_known_types::Any;
use url::Url;
use serde_json::Value;
use std::collections::HashMap;

use crate::protos::example::GetFeeOutput;

fn send_query_request(endpoint_url: &str, query_request: QueryRequest) -> QueryResponse {
  let address = query_request.get_address();
  let method = query_request.get_method();
  let input = query_request.get_input();

  let msg = input.get_msg();
  let args = input.get_arguments();

  let url = Url::parse(endpoint_url).unwrap();
  let url = url.join(&format!("/api/v1/contract/query?address={}&method={}", address, method)).unwrap();
  let url = url.as_str();

  let mut query_response = QueryResponse::new();

  match address {
    // ConstantPrice
    // See: https://zinc.zksync.io/07-smart-contracts/02-minimal-example.html
    "0x1f81df95c5478059e0e85f7594467bbfe511792a" => {
      match method {
        "get_fee" => {
          let mut req_http: HashMap<&str, HashMap<&str, &str>> = HashMap::new();
          req_http.insert("arguments", HashMap::new());

          let res_http = reqwest::blocking::Client::new()
            .put(url)
            .json(&req_http)
            .send()
            .expect("cannot create http request");

          if res_http.status().is_success() {
            let res_json = res_http
              .text()
              .expect("cannot read http response");

            let res_json: serde_json::Value = serde_json::from_str(&res_json)
              .expect("cannot parse response as json");

            let val = res_json.get("output")
              .expect("unexpected response value: not found \"output\" key")
              .as_str()
              .expect("\"output\" value type is not json str")
              .parse()
              .expect("cannot parse as integer");

            let mut out = GetFeeOutput::new();
            out.set_fee(val);
            let out = Any::pack(&out)
              .expect("cannot pack output into Any type proto");

            query_response.set_output(out);
          }
        },
        _ => (),
      }
    },
    _ => (),
  }

  query_response
}
