# Create a project

## New cargo project

```bash
cargo new cache
```

## Libraries

### Tokio

```bash
cargo add tokio --features full
cargo add tokio-util
cargo add tracing
cargo add tracing-subscriber
```

### clap

- repo: [clap](https://github.com/clap-rs/clap)
- docs: [clap](https://docs.rs/clap/latest/clap/)

```bash
cargo add clap -F derive env
```

