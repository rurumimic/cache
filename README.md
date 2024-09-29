# Cache

In-memory key-value store.

## Contents

- [docs](docs/README.md)

## Getting Started

### Server

```bash
cargo run --example server -- --host 127.0.0.1 --port 11211
```

### Client

```bash
telnet 127.0.0.1 11211
```

```bash
Trying 127.0.0.1...
Connected to 127.0.0.1.
Escape character is '^]'.
hello
^]
telnet> quit
```

