- This is an example to get introspec. It is same output with:

~~~
  # dbus-send --system --dest=org.freedesktop.DBus \
    --type=method_call --print-reply /org/freedesktop/DBus \
    org.freedesktop.DBus.Introspectable.Introspect
~~~
