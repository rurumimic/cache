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
./a.out > leaked 2>&1
```

## Output

- [leaked](leaked)
- [clean](clean)

### leaked

```bash
Allocated: 54784344, active: 58597376, metadata: 4352272 (n_thp 0, edata 124928, rtree 2097152), resident: 63168512, mapped: 65134592, retained: 7217152
```

### clean

```bash
Allocated: 110592, active: 118784, metadata: 4238352 (n_thp 0, edata 11008, rtree 2097152), resident: 5529600, mapped: 7606272, retained: 782336
```

