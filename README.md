This is a simple test case for https://github.com/fermyon/spin/issues/413.

To run it, first install [spin](https://github.com/fermyon/spin),
[bindle](https://github.com/deislabs/bindle), and
[heaptrack](https://github.com/KDE/heaptrack), then run:

```
bindle-server --address 127.0.0.1:8000 --directory . --unauthenticated
```

and, in another terminal:

```bash
spin build
spin bindle push --file spin.toml
# this will block waiting for connections; just Ctrl-C it after a second or two:
heaptrack spin up --bindle spin-test/1.0.0
heaptrack_gui heaptrack.spin.*
```
