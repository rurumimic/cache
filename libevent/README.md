# libevent

## Links

- [libevent.org](https://libevent.org)
  - [source](https://github.com/libevent/libevent)
  - [docs](https://libevent.org/libevent-book/)
- packages
  - [macports](https://ports.macports.org/port/libevent/details/): [Portfile](https://github.com/macports/macports-ports/blob/master/devel/libevent/Portfile)

---

## Install

```bash
# ubuntu
sudo apt install libevent-dev

# mac
sudo port install libevent
```

---

## Source Code

```bash
git clone https://github.com/libevent/libevent.git
```

### Build

#### Environment variables

```bash
export CC=clang
export CXX=clang++
```

#### Install Prefix

Anywhere you want to install:

```bash
mkdir /<ABSOLUTE_PATH>/dist
```

#### CMake

```bash
mkdir build && cd build
```

```bash
cmake -S .. -B . --install-prefix=/<ABSOLUTE_PATH>/dist

# ...
CMAKE_INSTALL_PREFIX:     /<ABSOLUTE_PATH>/dist
# ...
-- Configuring done
-- Generating done
-- Build files have been written to: …/libevent/build
```

```bash
make
```

#### clangd

- [bear](https://github.com/rizsotto/Bear)

```bash
make clean
bear -- make
```

#### Install

```bash
make install

# ...
Install the project...
-- Install configuration: "Release"
```

#### Installed libevent

```bash
<ABSOLUTE_PATH>/dist
├── bin/
│   └── event_rpcgen.py
├── include/
│   ├── evdns.h
│   ├── event.h
│   ├── event2/*.h
│   ├── evhttp.h
│   ├── evrpc.h
│   └── evutil.h
└── lib/
    ├── cmake/libevent/*.cmake
    ├── libevent.a
    ├── libevent_core.a
    ├── libevent_extra.a
    ├── libevent_openssl.a
    ├── libevent_pthreads.a
    └── pkgconfig/*.pc
```

##### .clangd

```yml
CompileFlags:
  Add:
    - "-I/<ABSOLUTE_PATH>/libevent/build/include/" # evconfig-private.h
    - "-I/<ABSOLUTE_PATH>/dist/include"
```

---

## Examples

libevent.git: `/sample`

- [Hello, World!](/libevent/examples/helloworld/README.md): [main.c](/libevent/examples/helloworld/main.c)

