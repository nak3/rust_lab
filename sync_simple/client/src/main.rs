extern crate clap;
extern crate tarpc;
extern crate hello_api;

use tarpc::sync::client::ClientExt;
use tarpc::sync::client;

fn main() {
    let addr = "localhost:3000";
    let client = hello_api::SyncClient::connect(addr, client::Options::default()).unwrap();
    println!("{}", client.hello("Mom".to_string()).unwrap());
}
