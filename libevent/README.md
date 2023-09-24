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
