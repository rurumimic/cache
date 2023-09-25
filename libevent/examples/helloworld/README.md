# Hello, World

## Build

```bash
make clean
make
```

```bash
clang -v -o a.out -levent -Wall -W -pedantic main.c
```

## Run

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
telnet 127.0.0.1 9995

Trying 127.0.0.1...
Connected to 127.0.0.1.
Escape character is '^]'.
Hello, World!
Connection closed by foreign host.
```

### Curl

```bash
curl telnet:://127.0.0.1:9995
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
