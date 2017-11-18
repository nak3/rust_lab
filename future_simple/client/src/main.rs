extern crate clap;
extern crate tarpc;
extern crate service_api;

use clap::{App, SubCommand, Arg};
use tarpc::sync::client::ClientExt;
use tarpc::sync::client;

static SERVER: &'static str = "localhost:3000";

fn serve() -> Result<Vec<service_api::Person>, String> {
    let client = service_api::SyncClient::connect(SERVER, client::Options::default()).unwrap();

    let app = App::new("client")
        .subcommand(SubCommand::with_name("list").about("list all people."))
        .subcommand(
            SubCommand::with_name("add")
                .about("add data person.")
                .arg(Arg::with_name("name"))
                .arg(Arg::with_name("age")),
        )
        .get_matches();

    if let Some(ref matches) = app.subcommand_matches("add") {
        let name = matches.value_of("name").unwrap();
        let age = matches.value_of("age").unwrap().parse::<u64>().unwrap();
        let _ = client.add(name.into(), age);
        return Ok(Vec::new());
    }

    if let Some(ref _matches) = app.subcommand_matches("list") {
        return Ok(client.list().unwrap());
    }

    Err("FATAL: Should not reach here.".to_string())
}

fn main() {
    match serve() {
        Err(why) => panic!("{:?}", why),
        Ok(ret) => {
            for i in 0..ret.len() {
                println!("{:?}", ret[i]);
            }
        }
    }
}
