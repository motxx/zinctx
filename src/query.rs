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
    let res = send_request(endpoint_url, proto);
    let s = CString::new(res.write_to_bytes().unwrap()).unwrap();
    let p = s.as_ptr();
    std::mem::forget(s);
    p
}

// TODO: コントラクト別の実装を隠蔽
use protobuf::well_known_types::Any;

use crate::protos::example::GetFeeOutput;

fn send_request(endpoint_url: &str, req: QueryRequest) -> QueryResponse {
  let mut res = QueryResponse::new();

  let address = req.get_address();
  let method = req.get_method();
  let input = req.get_input();

  let msg = input.get_msg();
  let args = input.get_arguments();

  match address {
    "contract-example" => {
      match method {
        "get_fee" => {
          // TODO: Use http client to request to Zandbox server
          println!("endpoint: {}", endpoint_url);
          let mut fee = GetFeeOutput::new();
          fee.set_fee(123);
          let out = Any::pack(&fee).unwrap();
          res.set_output(out);
        },
        _ => (),
      }
    },
    _ => (),
  }

  res
}
