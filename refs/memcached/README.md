# Memcached

## Contents

- [Architecture](architecture.md)
- [DTrace](dtrace.md)

## Read

Read [memcached.c](docs/README.md), memcached.h
- [items.c](docs/items.md)
- [thread.c](docs/thread.md)
- [proto_text.c](docs/proto_text.md)
- [slabs.c](docs/slabs.md)

---

## Links

- [memcached.org](https://memcached.org/)
  - [source](https://github.com/memcached/memcached)
- services
  - aws [memcached](https://aws.amazon.com/memcached/?nc1=h_ls)
- video
  - [Facebook and memcached - Tech Talk](https://youtu.be/UH7wkvcf0ys?si=FOHJg_3YAtTGD68r)

### Library

- [libevent](/refs/libevent/README.md)

---

## Install

```bash
# ubuntu
sudo apt install memcached
```

---

## Run

### Server Logs

```bash
./memcached
```

### Client Logs

```bash
telnet localhost 11211
```

```bash
Trying ::1...
Connected to localhost.
Escape character is '^]'.

stat

set hello 0 30 5 # set key flag 30s 5 
world
STORED

get hello
VALUE hello 0 5
world
END

^]
telnet>
```

---

## Source Code

```bash
git clone https://github.com/memcached/memcached.git
```

### Build

- How to [BUILD](https://github.com/memcached/memcached/blob/master/BUILD)
- wiki: [Install from source](https://github.com/memcached/memcached/wiki/Install#from-source)

#### Environment variables

```bash
export CC=clang
export CXX=clang++
```

```bash
# macports
export LDFLAGS="-L/opt/local/lib"
export CPPFLAGS="-I/opt/local/include"
export PKG_CONFIG_PATH="/opt/local/lib/pkgconfig:$PKG_CONFIG_PATH"
```

#### Install Prefix

Anywhere you want to install:

```bash
mkdir /<ABSOLUTE_PATH>/dist
```

#### Make

```bash
./autogen.sh
```

```bash
./configure \
# CFLAGS="-g" \
# --enable-asan \
# --enable-dtrace \
# --enable-64bit \
--prefix=/<ABSOLUTE_PATH_MEMCACHED>/dist \
--with-libevent=/<ABSOLUTE_PATH_LIBEVENT>/dist

checking for gcc... clang
checking for libevent directory... …/libevent/dist
```

```bash
make
make install
```

```bash
<ABSOLUTE_PATH>/dist
├── bin
│   └── memcached
├── include
│   └── memcached
│       ├── protocol_binary.h
│       └── xxhash.h
└── share
    └── man
        └── man1
            └── memcached.1
```

#### clangd

- [bear](https://github.com/rizsotto/Bear)

```bash
make clean
bear -- make
```

##### .clangd

```yml
CompileFlags:
  Add:
    - "-include=config.h"
    - "-I/<ABSOLUTE_PATH>/dist/include"
```
---

## Debug

1. Use `lldb-vscode`
1. compile with `-g`
1. Set breakpoints
1. execute `dist/bin/memcached -A`
  - `-A`: `--enable-shutdown`
1. `telnet localhost 11211`
1. commands
  1. `set hello 0 30 5`, `world`
  1. `get hello`
  1. `shutdown`

