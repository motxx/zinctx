extern crate libc;
extern crate protobuf;

use crate::protos::zinctx::{QueryRequest, QueryResponse};
use protobuf::Message;
use std::ffi::{CStr, CString};

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
  response
}
