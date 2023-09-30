# Cache

## Getting Started

```bash
cargo new cache
```

### Libraries

#### Tokio

- [tokio.rs](https://tokio.rs/)
  - [tutorial](https://tokio.rs/tokio/tutorial)
- repo
  - tokio: [docs](https://docs.rs/tokio/latest/tokio), [crates](https://crates.io/crates/tokio), [github](https://github.com/tokio-rs/tokio)
  - util: [docs](https://docs.rs/tokio-util/latest/tokio_util), [crates](https://crates.io/crates/tokio-util), [github](https://github.com/tokio-rs/tokio/tree/master/tokio-util)
  - tracing: [docs](https://docs.rs/tracing/latest/tracing), [crates](https://crates.io/crates/tracing), [github](https://github.com/tokio-rs/tracing)
  - mio: [docs](https://docs.rs/mio/latest/mio), [crates](https://crates.io/crates/mio), [github](https://github.com/tokio-rs/mio)
  - bytes: [docs](https://docs.rs/bytes/latest/bytes), [crates](https://crates.io/crates/bytes) [github]()
  - console: [docs](https://docs.rs/tokio-console/latest/tokio_console), [crates](https://crates.io/crates/tokio-console), [github](https://github.com/tokio-rs/console)
  - async-backtrace: [docs](https://docs.rs/async-backtrace/latest/async_backtrace), [crates](https://crates.io/crates/async-backtrace), [github](https://github.com/tokio-rs/async-backtrace)
  - io-uring: [docs](https://docs.rs/tokio-uring/latest/tokio_uring), [crates](https://crates.io/crates/tokio-uring) [github](https://github.com/tokio-rs/io-uring)
- examples
  - [mini-redis](/tokio-rs/mini-redis/README.md): [github](https://github.com/tokio-rs/mini-redis)

```bash
cargo add tokio --features full
cargo add tokio-util
cargo add tracing
cargo add tracing-subscriber
cargo add console-subscriber

cargo add --optional opentelemetry tracing-opentelemetry opentelemetry-jaeger
```

##### Tokio Console

Install Console:

```bash
cargo install --locked tokio-console
```

Run Console:

```bash
tokio-console
```

Run the project:

```bash
RUSTFLAGS="--cfg tokio_unstable" cargo run
```

##### OpenTelemetry

Run [Jaeger](https://www.jaegertracing.io):

```bash
docker run -d -p6831:6831/udp -p6832:6832/udp -p16686:16686 -p14268:14268 jaegertracing/all-in-one:latest
```

Open Jaeger: [http://localhost:16686](http://localhost:16686)

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

