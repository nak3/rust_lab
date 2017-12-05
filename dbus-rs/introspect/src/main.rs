extern crate dbus;

use dbus::{Connection, BusType, Message, Path};
use dbus::arg;

fn main() {
    // Get object path by service name
    let c = Connection::get_private(BusType::System).unwrap();
    let m = Message::new_method_call("org.freedesktop.DBus",
                                     "/org/freedesktop/DBus",
                                     "org.freedesktop.DBus.Introspectable",
                                     "Introspect")
        .unwrap();
    let r = c.send_with_reply_and_block(m, 2000).unwrap();
    let result: String = r.read1().unwrap();
    println!("{:?}", result);
}
