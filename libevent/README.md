# libevent

## Links

- [libevent.org](https://libevent.org)
  - [source](https://github.com/libevent/libevent)
  - [docs](https://libevent.org/libevent-book/)
- packages
  - [macports](https://ports.macports.org/port/libevent/details/): [Portfile](https://github.com/macports/macports-ports/blob/master/devel/libevent/Portfile)

---

## Install

```bash
# ubuntu
sudo apt install libevent-dev

# mac
sudo port install libevent
```

---

## Source Code

```bash
git clone https://github.com/libevent/libevent.git
```

### Build

```bash
mkdir build && cd build
cmake ..     # Default to Unix Makefiles.
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
    - "-I./include"
    - "-I./build/include/"
```

---

## Examples

libevent.git: `/sample`

- [Hello, World!](/libevent/examples/helloworld/README.md): [main.c](/libevent/examples/helloworld/main.c)

