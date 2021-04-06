use crate::protos::zinctx::{QueryRequest, QueryResponse, Value};

pub fn send_request(proto: QueryRequest) -> QueryResponse {
  let mut response = QueryResponse::new();
  let mut val = Value::new();
  val.set_stringval("hello".to_string());
  response.set_output(val);
  response
}
