extern crate service_api;
extern crate tarpc;

use std::cell::RefCell;
use service_api::SyncServiceExt;
use tarpc::util::Never;
use tarpc::sync::server;
use std::sync::{Arc, Mutex};

static SERVER: &'static str = "localhost:3000";

#[derive(Clone, Debug)]
pub struct ServiceServer {
    customer_service: Arc<Mutex<RefCell<Vec<service_api::Person>>>>,
}

impl service_api::SyncService for ServiceServer {
    fn add(&self, name: String, age: u64) -> Result<(), Never> {
        let person = service_api::Person { name, age };
        let customer_service = self.customer_service.clone();
        let vec = customer_service.lock().unwrap();
        vec.borrow_mut().push(person);
        Ok(())
    }

    fn list(&self) -> Result<Vec<service_api::Person>, Never> {
        let customer_service = self.customer_service.clone();
        let list = customer_service.lock().unwrap();
        let ret = list.borrow().to_vec();
        Ok(ret)
    }
}

fn main() {
    println!("starting server at {}", SERVER);
    let ve: Vec<service_api::Person> = Vec::new();
    let handle = ServiceServer { customer_service: Arc::new(Mutex::new((RefCell::new(ve)))) }
        .listen(SERVER, server::Options::default())
        .unwrap();
    handle.run();
}
