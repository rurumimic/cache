# libevent

## Links

- [libevent.org](https://libevent.org)
  - [source](https://github.com/libevent/libevent)
  - [docs](https://libevent.org/libevent-book/)
- packages
  - [macports](https://ports.macports.org/port/libevent/details/): [Portfile](https://github.com/macports/macports-ports/blob/master/devel/libevent/Portfile)

---

## Examples

libevent.git: `/sample`

- [Hello, World!](examples/helloworld/README.md): [main.c](examples/helloworld/main.c)
- [Signal](examples/signal/README.md): [main.c](examples/signal/main.c)
- [Websocket](examples/websocket/README.md): [main.c](examples/websocket/main.c)

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

## ChangeLogs

### 2.2.1-alpha

[release-2.2.1-alpha](https://github.com/libevent/libevent/releases/tag/release-2.2.1-alpha)

- wepoll backend (by Nick Grifka, @nigriMSFT, #1006)
- signalfd backend (by Dmitry Antipov, @dmantipov, #1342)
- DNS over TCP for evdns (by ayuseleznev, @seleznevae, #1004)
- websockets layer (by Dmitry Ilyin, @widgetii, #1322)
- "prepare" and "check" watchers (by Dan Rosen, @mergeconflict, #793)
- MbedTLS support (by okhowang, @okhowang, #1028)
- unix domain sockets for HTTP (by Sean Young, @seanyoung, #322)
- cmake over autotools/automake
- extensive CI (significant work by converting to github actions had been done by @ygj6, #951)
- documentation deploy - https://libevent.org/doc/

