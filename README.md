# Cache

## Getting Started

```bash
cargo new cache
```

### Libraries

#### Tokio

- [tokio.rs](https://tokio.rs/)
  - [github](https://github.com/tokio-rs/tokio)
  - [tutorial](https://tokio.rs/tokio/tutorial)
- examples
  - [mini-redis](/tokio-rs/mini-redis/README.md): [github](https://github.com/tokio-rs/mini-redis)

```bash
cargo add tokio --features full
cargo add tokio-util
```

---

## Ref

### Cache Servers

- [memcached](/memcached/README.md)
  - read [memcached.c](/memcached/docs/README.md)
- tokio
  - [mini-redis](/tokio-rs/mini-redis/README.md): [github](https://github.com/tokio-rs/mini-redis)

### Event Handler

- [libevent](/libevent/README.md)
  - examples
    - [Hello, World!](/libevent/examples/helloworld/README.md): [main.c](/libevent/examples/helloworld/main.c)
    - [Signal](/libevent/examples/signal/README.md): [main.c](/libevent/examples/signal/main.c)
    - [Websocket](/libevent/examples/websocket/README.md): [main.c](/libevent/examples/websocket/main.c)
  - releases
    - [2.2.1-alpha](https://github.com/libevent/libevent/releases/tag/release-2.2.1-alpha): websockets

