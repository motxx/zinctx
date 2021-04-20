extern crate protoc_rust;

use protoc_rust::Customize;
use std::error::Error;

fn main() -> Result<(), Box<Error>> {
    let proto_files = vec!["src/zinc.proto", "src/zinctx.proto", "src/contracts/example.proto"];

    protoc_rust::run(protoc_rust::Args {
        input: &proto_files[..],
        out_dir: "src/protos",
        includes: &["src"],
        customize: Customize {
            serde_derive: Some(true),
            ..Default::default()
        },
    })?;

    /*
    let proto_files = vec!["src/contracts/example.proto"];

    protoc_rust::run(protoc_rust::Args {
        input: &proto_files[..],
        out_dir: "src/protos/contracts",
        includes: &["src"],
        customize: Customize {
            ..Default::default()
        },
    })?;
    */
    Ok(())
}
