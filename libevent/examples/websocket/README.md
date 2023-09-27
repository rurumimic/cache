# Websocket

- websockets layer
  - [release-2.2.1-alpha](https://github.com/libevent/libevent/releases/tag/release-2.2.1-alpha)
  - [pull request #1322](https://github.com/libevent/libevent/pull/1322): Add minimal WebSocket server implementation for evhttp

## Build

### without ws.h

```bash
clang -Wall -W -pedantic -o a.out main.c -levent
main.c:5:10: fatal error: 'event2/ws.h' file not found
    5 | #include <event2/ws.h>
      |          ^~~~~~~~~~~~~
1 error generated.
make: *** [Makefile:8: all]
```

### with ws.h

#### Latest Version

[How to build libevent](/libevent/README.md)

```bash
CompileFlags:
  Add:
    - "-I/<ABSOLUTE_PATH>/libevent/build/include/" # evconfig-private.h
    - "-I/<ABSOLUTE_PATH>/dist/include"
```

#### Build with ws.h

```bash
ABSOLUTE_PATH=/<ABSOLUTE_PATH>

make clean

make \
CFLAGS="-I${ABSOLUTE_PATH}/dist/include -I${ABSOLUTE_PATH}/libevent/build/include/" \
LIBS="-L${ABSOLUTE_PATH}/dist/lib -levent"
```

## Server

```bash
# linux
LD_LIBRARY_PATH="${ABSOLUTE_PATH}/dist/lib" ./a.out

Server runs
```

## Client

Open: [index.html](index.html)

## Log

### Server Log

```log
Server runs

New client joined from 127.0.0.1:57648
[127.0.0.1:57648] Hello?
'127.0.0.1:57648' renamed itself to 'Keanu'
[Keanu] Hi, I'm Keanu

New client joined from 127.0.0.1:57656
'127.0.0.1:57656' renamed itself to 'Denzel'
[Denzel] Hi, I'm Denzel

# after 1 sec

[Denzel] Bye, Keanu
'Denzel' left the chat
'Denzel' disconnected

# after 1 sec

[Keanu] Bye, Denzel

# CTRL-C

Interrupt signal received
Active connections: 1
'Keanu' disconnected
```

### Client Log

```log
[Keanu Open]
[Message] [127.0.0.1:57648] Hello?
[Message] '127.0.0.1:57648' renamed itself to 'Keanu'
[Message] [Keanu] Hi, I'm Keanu

[Denzel Open]
[Message] '127.0.0.1:57656' renamed itself to 'Denzel'
[Message] [Denzel] Hi, I'm Denzel

# after 1 sec

[Message] [Denzel] Bye, Keanu
WebSocket connection to 'ws://localhost:3030/ws' failed: Data frame received after close
[Denzel Error]
[Denzel Dead] code=1006 reason=
[Message] 'Denzel' left the chat

# after 1 sec

[Message] [Keanu] Bye, Denzel

# CTRL-C

[Keanu Dead] code=1006 reason=
```

