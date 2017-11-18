#![feature(plugin, use_extern_macros)]
#![plugin(tarpc_plugins)]

#[macro_use]
extern crate tarpc;

#[macro_use]
extern crate serde_derive;

service! {
    rpc list() -> Vec<Person>;
    rpc add(naem: String, age: u64);
}

#[derive(Clone, Debug)]
#[derive(Serialize, Deserialize)]
pub struct Person {
    pub name: String,
    pub age: u64,
}
