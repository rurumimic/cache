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
sudo apt isntall memcached
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
# mac
export CC=clang
export CXX=clang++
export LDFLAGS="-L/opt/local/lib"
export CPPFLAGS="-I/opt/local/include"
export PKG_CONFIG_PATH="/opt/local/lib/pkgconfig:$PKG_CONFIG_PATH"
```

#### Make

```bash
./autogen.sh
./configure
make
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
```

### Library

- [libevent](/libevent/README.md)

