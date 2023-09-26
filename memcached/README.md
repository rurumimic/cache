# Memcached

Read [memcached.c](docs/README.md)

## Links

- [memcached.org](https://memcached.org/)
  - [source](https://github.com/memcached/memcached)
- services
  - aws [memcached](https://aws.amazon.com/memcached/?nc1=h_ls)
- video
  - [Facebook and memcached - Tech Talk](https://youtu.be/UH7wkvcf0ys?si=FOHJg_3YAtTGD68r)

---

## Install

```bash
# ubuntu
sudo apt install memcached
```

---

## Source Code

```bash
git clone https://github.com/memcached/memcached.git
```

### Build

- How to [BUILD](https://github.com/memcached/memcached/blob/master/BUILD)
- wiki: [Install from source](https://github.com/memcached/memcached/wiki/Install#from-source)

#### Environment variables

```bash
export CC=clang
export CXX=clang++
```

```bash
# macports
export LDFLAGS="-L/opt/local/lib"
export CPPFLAGS="-I/opt/local/include"
export PKG_CONFIG_PATH="/opt/local/lib/pkgconfig:$PKG_CONFIG_PATH"
```

#### Install Prefix

Anywhere you want to install:

```bash
mkdir /<ABSOLUTE_PATH>/dist
```

#### Make

```bash
./autogen.sh
./configure --prefix=/<ABSOLUTE_PATH>/dist
make
make install
```

```bash
<ABSOLUTE_PATH>/dist
├── bin
│   └── memcached
├── include
│   └── memcached
│       ├── protocol_binary.h
│       └── xxhash.h
└── share
    └── man
        └── man1
            └── memcached.1
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
    - "-include=config.h"
    - "-I/<ABSOLUTE_PATH>/dist/include"
```

### Library

- [libevent](/libevent/README.md)

