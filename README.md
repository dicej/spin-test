This is a simple test case for https://github.com/fermyon/spin/issues/413.

To run it, first install [spin](https://github.com/fermyon/spin),
[bindle](https://github.com/deislabs/bindle),
[heaptrack](https://github.com/KDE/heaptrack), and optionally
[siege](https://github.com/JoeDog/siege) then run:

```
bindle-server --address 127.0.0.1:8000 --directory . --unauthenticated
```

and, in another terminal:

```bash
spin build
spin bindle push --file spin.toml
# This will block waiting for connections; just Ctrl-C it after a second or two.
# Alternatively, run `timeout 60s siege http://127.0.0.1:3000/test` in a third terminal
# and Ctrl-C once it's done.
rm -f heaptrack.spin.* && heaptrack spin up --bindle spin-test/1.0.0
heaptrack_gui heaptrack.spin.*
```
