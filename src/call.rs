extern crate libc;
extern crate protobuf;

use protobuf::Message;
use std::ffi::{CStr, CString};

use crate::protos::zinctx::{CallRequest, CallResponse};

#[no_mangle]
pub extern "C" fn ffi_send_call_request(endpoint_url_ptr: *const libc::c_char, proto_ptr: *const libc::c_char) -> *const libc::c_char {
    let endpoint_url = unsafe { CStr::from_ptr(endpoint_url_ptr) }.to_str().unwrap();
    let proto_slice = unsafe { CStr::from_ptr(proto_ptr) };
    let proto = CallRequest::parse_from_bytes(proto_slice.to_bytes()).unwrap();
    let res = send_call_request(endpoint_url, proto);
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

use crate::protos::example::DepositOutput;

fn send_call_request(endpoint_url: &str, call_request: CallRequest) -> CallResponse {
  let address = call_request.get_address();
  let method = call_request.get_method();
  let input = call_request.get_input();

  let msg = input.get_msg();
  let args = input.get_arguments();

  let url = Url::parse(endpoint_url).unwrap();
  let url = url.join(&format!("/api/v1/contract/call?address={}&method={}", address, method)).unwrap();
  let url = url.as_str();

  let mut call_response = CallResponse::new();

  match address {
    // ConstantPrice
    // See: https://zinc.zksync.io/07-smart-contracts/02-minimal-example.html
    "0x1f81df95c5478059e0e85f7594467bbfe511792a" => {
      match method {
        "deposit" => {
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
              .expect("\"output\" value type is not json str");

            let out = DepositOutput::new();
            let out = Any::pack(&out)
              .expect("cannot pack output into Any type proto");

            call_response.set_output(out);
          }
        },
        _ => (),
      }
    },
    _ => (),
  }

  call_response
}
/*
{
  "arguments": {
    "value": "42"
  },
  "transaction": {
    "tx": {
      "type": "Transfer",
      "accountId": 1,
      "from": "0x36615cf349d7f6344891b1e7ca7c72883f5dc049",
      "to": "0x1234567812345678123456781234567812345678",
      "token": 0,
      "amount": "0",
      "fee": "37500000000000",
      "nonce": 2,
      "signature": {
        "pubKey": "07f86efb9bf58d5ebf23042406cb43e9363879ff79223be05b7feac1dbc58c86",
        "signature": "042c7356c3970c5ab620e1eaf0a9e39563edc9383072ac33a29398f11678b2a3acdc40ff05acd225b6a71962cfabfa6012fae8492106987bcd48135fefa09c02"
      }
    },
    "ethereumSignature": {
      "type": "EthereumSignature",
      "signature": "0xbe7a011c0b03a2ab8eceb3f51ec3055e5998b025e3e41a320f6b00532a4c49604608fe7b9c36d837c36817bbaf5570197484281dd45d83f2d9ef867b7454b91e1b"
    }
  }
}
*/