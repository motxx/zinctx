extern crate libc;
extern crate protobuf;

use protobuf::Message;
use crate::protos::zinctx::{QueryRequest, QueryResponse};
use std::ffi::{CStr, CString};

// TODO: コントラクト別の実装は隠蔽
// TODO: exampleをcontracts配下にする
use crate::protos::example::GetFeeOutput;
use protobuf::well_known_types::Any;
use std::io::BufReader;

#[no_mangle]
pub extern "C" fn ffi_send_query_request(ptr: *const libc::c_char) -> *const libc::c_char {
    let slice = unsafe { CStr::from_ptr(ptr) };
    let proto = QueryRequest::parse_from_bytes(slice.to_bytes()).unwrap();
    let res = send_request(proto);
    let s = CString::new(res.write_to_bytes().unwrap()).unwrap();
    let p = s.as_ptr();
    std::mem::forget(s);
    p
}

pub fn send_request(proto: QueryRequest) -> QueryResponse {
  let mut response = QueryResponse::new();

  // TODO: コントラクト別の実装は隠蔽
  let mut fee = GetFeeOutput::new();
  fee.set_fee(123);
  let out = Any::pack(&fee).unwrap();
  response.set_output(out);

  response
}
