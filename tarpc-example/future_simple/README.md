future_simple
---

sync_simple is an example for the usage of `future` API.

Build
---

~~~
cd future_simple
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
./client/target/debug/future_cli add John 27
./client/target/debug/future_cli add Alice 24

./client/target/debug/future_cli list
Person { name: "John", age: 27 }
Person { name: "Alice", age: 24 }
~~~
