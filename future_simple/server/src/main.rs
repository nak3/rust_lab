extern crate service_api;
extern crate tarpc;
extern crate tokio_core;

use std::cell::RefCell;
use tarpc::util::{Never, FirstSocketAddr};
use std::sync::{Arc, Mutex};
use tarpc::future::server::Options;
use service_api::{FutureServiceExt, FutureService};

static SERVER: &'static str = "localhost:3000";

#[derive(Clone, Debug)]
pub struct ServiceServer {
    customer_service: Arc<Mutex<RefCell<Vec<service_api::Person>>>>,
}

impl FutureService for ServiceServer {
    type AddFut = Result<(), Never>;
    fn add(&self, name: String, age: u64) -> Self::AddFut {
        let person = service_api::Person { name, age };
        let customer_service = self.customer_service.clone();
        let vec = customer_service.lock().unwrap();
        vec.borrow_mut().push(person);
        Ok(())
    }

    type ListFut = Result<Vec<service_api::Person>, Never>;
    fn list(&self) -> Self::ListFut {
        let customer_service = self.customer_service.clone();
        let list = customer_service.lock().unwrap();
        let ret = list.borrow().to_vec();
        Ok(ret)
    }
}

fn main() {
    println!("starting server at {}", SERVER);

    let ve: Vec<service_api::Person> = Vec::new();
    let mut reactor = tokio_core::reactor::Core::new().unwrap();

    let (_, server) = ServiceServer { customer_service: Arc::new(Mutex::new((RefCell::new(ve)))) }
        .listen(
            SERVER.first_socket_addr(),
            &reactor.handle(),
            Options::default(),
        )
        .unwrap();

    reactor.run(server).unwrap();
}
