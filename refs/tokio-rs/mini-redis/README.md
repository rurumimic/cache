# mini-redis

- github: [tokio-rs/mini-redis](https://github.com/tokio-rs/mini-redis)

```bash
git clone https://github.com/tokio-rs/mini-redis.git
```

## Run

### Server

```bash
RUST_LOG=debug cargo run --bin mini-redis-server
```

### Client

#### Hello World

```bash
cargo run --example hello_world

got value from the server; success=true
```

```log
2023-09-28T05:57:49.535866Z  INFO mini_redis::server: accepting inbound connections

2023-09-28T05:57:54.321619Z DEBUG run: mini_redis::server: cmd=Set(Set { key: "hello", value: b"world", expire: None })
2023-09-28T05:57:54.321727Z DEBUG run:apply: mini_redis::cmd::set: response=Simple("OK")
2023-09-28T05:57:54.321986Z DEBUG run: mini_redis::server: cmd=Get(Get { key: "hello" })
2023-09-28T05:57:54.322066Z DEBUG run:apply: mini_redis::cmd::get: response=Bulk(b"world")

2023-09-28T05:58:33.641035Z  INFO mini_redis::server: shutting down
```

#### Set And Get

```bash
cargo run --bin mini-redis-cli set foo bar

OK
```

```bash
cargo run --bin mini-redis-cli get foo

"bar"
```

```log
2023-09-28T05:59:12.359868Z  INFO mini_redis::server: accepting inbound connections

2023-09-28T05:59:16.298668Z DEBUG run: mini_redis::server: cmd=Set(Set { key: "foo", value: b"bar", expire: None })
2023-09-28T05:59:16.298773Z DEBUG run:apply: mini_redis::cmd::set: response=Simple("OK")

2023-09-28T05:59:41.627563Z DEBUG run: mini_redis::server: cmd=Get(Get { key: "foo" })
2023-09-28T05:59:41.627628Z DEBUG run:apply: mini_redis::cmd::get: response=Bulk(b"bar")

2023-09-28T06:00:31.162311Z  INFO mini_redis::server: shutting down
```

