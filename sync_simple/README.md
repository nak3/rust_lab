sync_simple
---

sync_simple is an example for the usage of `sync` API.

Build
---

~~~
cd sync_simple
make
~~~

Run
---

#### Run server
~~~
make run_server
~~~

#### Run client

~~~
./client/target/debug/sync_cli add John 27
./client/target/debug/sync_cli add Alice 24

./client/target/debug/sync_cli list
Person { name: "John", age: 27 }
Person { name: "Alice", age: 24 }
~~~
