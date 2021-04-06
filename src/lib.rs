mod protos;
mod query;

extern crate libc;
extern crate protobuf;

use std::ffi::{CStr, CString};
use protobuf::Message;
use protos::zinctx::QueryRequest;

#[no_mangle]
pub extern "C" fn ffi_send_query_request(ptr: *const libc::c_char) -> *const libc::c_char {
    let slice = unsafe { CStr::from_ptr(ptr) };
    let proto = QueryRequest::parse_from_bytes(slice.to_bytes()).unwrap();
    let res = query::send_request(proto);
    let s = CString::new(res.write_to_bytes().unwrap()).unwrap();
    let p = s.as_ptr();
    std::mem::forget(s);
    p
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut sq = QueryRequest::new();
        sq.set_address("1234567890abcdef".to_string());
    }
}
