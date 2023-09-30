# Tracing

## Examples

- [lib.rs](lib.rs)

## Tracing

```bash
cargo add console-subscriber
cargo add opentelemetry tracing-opentelemetry opentelemetry-jaeger
```

### Tokio Console

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

### OpenTelemetry

Run [Jaeger](https://www.jaegertracing.io):

```bash
docker run -d -p6831:6831/udp -p6832:6832/udp -p16686:16686 -p14268:14268 jaegertracing/all-in-one:latest
```

Open Jaeger: [http://localhost:16686](http://localhost:16686)

