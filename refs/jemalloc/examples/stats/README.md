# Print stats

- [Getting Started](https://github.com/jemalloc/jemalloc/wiki/Getting-Started)

## Setup

### PATH

Set PATH where `jemalloc-config` is located:

```bash
export PATH="/<ABSOLUTE_PATH>/dist/bin:$PATH"
```

Help:

```bash
jemalloc-config -h
```

### .clangd
```yml
CompileFlags:
 Add:
   - "-I=/<ABSOLUTE_PATH>/dist/include"
```

## Build

- [Makefile](Makefile)

```bash
make clean
make
```

## Run

```bash
./a.out > output 2>&1
```

### Output

- [output](output)

