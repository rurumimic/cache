# Signal

## Build

```bash
make clean
make
```

```bash
# use system
clang -v -o a.out -levent -Wall -W -pedantic main.c
# -I/usr/local/include -L/usr/local/lib
```

## Run

```bash
./a.out
```

```bash
./a.out

^Csignal_cb: got signal 2
^Csignal_cb: got signal 2
^Csignal_cb: got signal 2
Finished.
```

