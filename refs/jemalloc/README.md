# jemalloc

## Links

- [jemalloc](https://jemalloc.net/)
  - [source](https://github.com/jemalloc/jemalloc)
  - [wiki](https://github.com/jemalloc/jemalloc/wiki/Getting-Started)

## Build

Configure:

```bash
./autogen.sh \
--prefix=/<ABSOLUTE_PATH>/dist \
--enable-debug \
--enable-prof \
--enable-log
```

Build:

```bash
make
bear -- make
```

Install:

```bash
make install
```

```bash
dist/
├── bin
│   ├── jemalloc-config
│   ├── jemalloc.sh
│   └── jeprof
├── include
│   └── jemalloc
│       └── jemalloc.h
├── lib
│   ├── libjemalloc.a
│   ├── libjemalloc_pic.a
│   ├── libjemalloc.so -> libjemalloc.so.2
│   ├── libjemalloc.so.2
│   └── pkgconfig
│       └── jemalloc.pc
└── share
    ├── doc
    │   └── jemalloc
    │       └── jemalloc.html
    └── man
        └── man3
```

#### clangd

- [bear](https://github.com/rizsotto/Bear)

```bash
make clean
bear -- make
```

##### .clangd

```yml
CompileFlags:
  Add:
    - "-include=/<ABSOLUTE_PATH>/dist/include/jemalloc/jemalloc.h"
```

