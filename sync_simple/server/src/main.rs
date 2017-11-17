extern crate hello_api;
extern crate tarpc;

use hello_api::SyncServiceExt;
use tarpc::util::Never;
use tarpc::sync::server;

#[derive(Clone)]
pub struct HelloServer;

impl hello_api::SyncService for HelloServer {
    fn hello(&self, name: String) -> Result<String, Never> {
        Ok(format!("Hey {}!", name))
    }
}

fn main() {
    println!("starting server at localhost:3000");
    let handle = HelloServer
        .listen("localhost:3000", server::Options::default())
        .unwrap();
    handle.run();
}
