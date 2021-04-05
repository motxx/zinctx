mod protos;

extern crate protobuf;

use protobuf::Message;
use protos::zinctx::SendQuery;
use zksync::Network;

pub fn send_query(sq: SendQuery) -> std::string::String {
    format!("address: {}, network: {}", sq.address, Network::Rinkeby)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut sq = SendQuery::new();
        sq.set_address("1234567890abcdef".to_string());
        let test = send_query(sq);
        assert_eq!(test, "address: 1234567890abcdef, network: rinkeby");
    }
}
