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

New client joined from 127.0.0.1:37212
'127.0.0.1:37212' renamed itself to 'Keanu'
[Keanu] Hi, I'm Keanu

New client joined from 127.0.0.1:37228
'127.0.0.1:37228' renamed itself to 'Denzel'
[Denzel] Hi, I'm Denzel
[Denzel] Bye, Keanu
'Denzel' left the chat
'Denzel' disconnected

[Keanu] Bye, Denzel

^C Interrupt signal received

Active connections: 1
'Keanu' disconnected
```

### Client Log

```log
[Open]
index.js:26 [Message] '127.0.0.1:37212' renamed itself to 'Keanu'
index.js:16 [Open]
index.js:46 [Message] '127.0.0.1:37228' renamed itself to 'Denzel'
index.js:26 [Message] [Keanu] Hi, I'm Keanu
index.js:26 [Message] '127.0.0.1:37228' renamed itself to 'Denzel'
index.js:26 [Message] [Denzel] Hi, I'm Denzel
index.js:46 [Message] [Denzel] Hi, I'm Denzel
index.js:46 [Message] [Denzel] Bye, Keanu
index.js:26 [Message] [Denzel] Bye, Keanu
index.js:2 WebSocket connection to 'ws://localhost:3030/ws' failed: Data frame received after close
index.js:59 [Error]
index.js:54 [Dead] code=1006 reason=
index.js:26 [Message] 'Denzel' left the chat
index.js:26 [Message] [Keanu] Bye, Denzel
index.js:34 [Dead] code=1006 reason=
```

