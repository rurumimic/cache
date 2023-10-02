# Hello, World

## Build

```bash
make clean
make
```

```bash
# use system
clang -v -o a.out -levent -Wall -W -pedantic main.c

# or custom
clang -v -o a.out \
-L/<ABSOLUTE_PATH>/dist/lib \
-I/<ABSOLUTE_PATH>/dist/include \
-levent \
-Wall -W -pedantic \
main.c
```

## Run

```bash
# use system
./a.out

# macos
DYLD_LIBRARY_PATH=/<ABSOLUTE_PATH>/dist/lib ./a.out

# linux
LD_LIBRARY_PATH=/<ABSOLUTE_PATH>/dist/lib ./a.out
```

```bash
./a.out 

flushed answer
flushed answer
flushed answer

# CTRL-C

Caught an interrupt signal; exiting cleanly in two seconds.
done
```

## Events

### Telnet

```bash
sudo port install inetutils
brew install inetutils
```

```bash
telnet 127.0.0.1 9995
# or
gtelent 127.0.0.1 9995

Trying 127.0.0.1...
Connected to 127.0.0.1.
Escape character is '^]'.
Hello, World!
Connection closed by foreign host.
```

### Curl

```bash
curl telnet://127.0.0.1:9995
Hello, World!

# CTRL-C
```

```bash
curl --max-time 2 telnet://127.0.0.1:9995
Hello, World!

# Enter

curl: (28) Time-out
```

### Netcat

```bash
nc 127.0.0.1 9995
Hello, World!

# CTRL-C or CTRL-D
```

```bash
nc -w 2 127.0.0.1 9995
Hello, World!
```

