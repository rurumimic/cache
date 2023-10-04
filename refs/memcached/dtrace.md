# Dtrace

- [DTrace](http://dtrace.org)

## Run memcached

```bash
./memcached -vv -A -P memcached.pid
```

## List Probes

```bash
pgrep memcached

1609
```

```bash
sudo dtrace -l -m memcached

   ID   PROVIDER            MODULE                          FUNCTION NAME
73200 memcached1609         memcached               process_get_command command-get
73207 memcached1609         memcached              complete_nread_ascii command-set
73209 memcached1609         memcached                          conn_new conn-allocate
73213 memcached1609         memcached                        conn_close conn-release
```


## Script

```d
#!/usr/sbin/dtrace -s

#pragma D option quiet

memcached$target:memcached:conn_new:conn-allocate
{
  printf("Connection allocated: connid=%d\n", arg0);
}

memcached$target:memcached:conn_close:conn-release
{
  printf("Connection released: connid=%d\n", arg0);
}

memcached$target:memcached:complete_nread_ascii:command-set
{
  printf("SET command: connid=%d, key=%s, keylen=%d, size=%d, casid=%d\n", arg0, copyinstr(arg1), arg2, arg3, arg4);
}

memcached$target:memcached:process_get_command:command-get
{
  printf("GET command: connid=%d, key=%s, keylen=%d, size=%d, casid=%d\n", arg0, copyinstr(arg1), arg2, arg3, arg4);
}
```

### Run Script

```bash
sudo dtrace -s ./memcached.d -p 1609

Connection allocated: connid=14
SET command: connid=14, key=hello, keylen=5, size=7, casid=1
GET command: connid=14, key=hello, keylen=5, size=7, casid=1
Connection released: connid=14
```

## Client

```bash
telnet localhost 11211

Trying 127.0.0.1...
Connected to localhost.
Escape character is '^]'.

set hello 0 30 5
world
STORED

get hello
VALUE hello 0 5
world
END

quit

Connection closed by foreign host.
```

