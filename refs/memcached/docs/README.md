# Read memcached.c

`memcached.h`

- [enum conn_states](#conn_states)
- [struct conn](#conn)

`memcached.c`

- [static void drive_machine(conn \*c)](#drive_machine)
- [int main (int argc, char \*\*argv)](#main)

## memcached.h

### conn_states

```c
enum conn_states {
    conn_listening,  /**< the socket which listens for connections */
    conn_new_cmd,    /**< Prepare connection for next command */
    conn_waiting,    /**< waiting for a readable socket */
    conn_read,       /**< reading in a command line */
    conn_parse_cmd,  /**< try to parse a command from the input buffer */
    conn_write,      /**< writing out a simple response */
    conn_nread,      /**< reading in a fixed number of bytes */
    conn_swallow,    /**< swallowing unnecessary bytes w/o storing */
    conn_closing,    /**< closing this connection */
    conn_mwrite,     /**< writing out many items sequentially */
    conn_closed,     /**< connection is closed */
    conn_watch,      /**< held by the logger thread as a watcher */
    conn_io_queue,   /**< wait on async. process to get response object */
    conn_max_state   /**< Max state value (used for assertion) */
};
```

### conn

```c
struct conn {
    sasl_conn_t *sasl_conn;
    int    sfd;
    // bool x 8
    // TLS: SSL, char*, bool
    enum conn_states  state;
    // ...
    struct event event;
    short  ev_flags;
    short  which;   /** which events were just triggered */

    char   *rbuf;   /** buffer to read commands into */
    char   *rcurr;  /** but if we parsed some already, this is where we stopped */
    int    rsize;   /** total allocated size of rbuf */
    int    rbytes;  /** how much data, starting from rcur, do we have unparsed */

    // ...

    void   *item;     /* for commands set/add/replace  */

    // ...

    LIBEVENT_THREAD *thread; /* Pointer to the thread object serving this connection */
    int (*try_read_command)(conn *c); /* pointer for top level input parser */
    ssize_t (*read)(conn  *c, void *buf, size_t count);
    ssize_t (*sendmsg)(conn *c, struct msghdr *msg, int flags);
    ssize_t (*write)(conn *c, void *buf, size_t count);
};
```

## memcached.c

### drive_machine

```c
static void drive_machine(conn *c) {
    bool stop = false;
    int sfd;
    socklen_t addrlen;
    struct sockaddr_storage addr;
    int nreqs = settings.reqs_per_event;
    int res;
    const char *str;

    while (!stop) {
        switch(c->state) {
        case conn_listening:
            addrlen = sizeof(addr);
            sfd = accept4(c->sfd, (struct sockaddr *)&addr, &addrlen, SOCK_NONBLOCK); 
            dispatch_conn_new(sfd, 
                              conn_new_cmd, 
                              EV_READ | EV_PERSIST, 
                              READ_BUFFER_CACHED, 
                              c->transport, 
                              ssl_v, 
                              c->tag, 
                              c->protocol);
            stop = true;
            break;
        // ...
        case conn_max_state:
            assert(false);
            break;
    }
}
```

### main

```c
static struct event_base *main_base; // event2/event_struct.h

int main (int argc, char **argv) {

    /* handle SIGINT, SIGTERM */
    signal(SIGINT, sig_handler);
    signal(SIGTERM, sig_handler);
    signal(SIGHUP, sighup_handler);
    signal(SIGUSR1, sig_usrhandler);

    /* init settings */
    settings_init();

    // read options
    while (-1 != (c = getopt(argc, argv, shortopts))) {};

    /* initialize main thread libevent instance */
#if defined(LIBEVENT_VERSION_NUMBER) && LIBEVENT_VERSION_NUMBER >= 0x02000101
    /* If libevent version is larger/equal to 2.0.2-alpha, use newer version */
    struct event_config *ev_config;
    ev_config = event_config_new();
    event_config_set_flag(ev_config, EVENT_BASE_FLAG_NOLOCK);
    main_base = event_base_new_with_config(ev_config);
    event_config_free(ev_config);
#else
    /* Otherwise, use older API */
    main_base = event_init();
#endif

    /* initialize other stuff */
    stats_init();
    logger_init();
    conn_init();
    assoc_init(settings.hashpower_init);
    slabs_init(settings.maxbytes, settings.factor, preallocate,
            use_slab_sizes ? slab_sizes : NULL, mem_base, reuse_mem);
    memcached_thread_init(settings.num_threads, NULL);
    init_lru_crawler(NULL);
    clock_handler(0, 0, 0);
    server_socket_unix(settings.socketpath,settings.access);
    server_sockets(settings.port, tcp_transport, portnumber_file);
    /* Give the sockets a moment to open. I know this is dumb, but the error
     * is only an advisory.
     */
    usleep(1000);
    save_pid(pid_file);
    /* Initialize the uriencode lookup table. */
    uriencode_init();

    /* enter the event loop */
    while (!stop_main_loop) {
        if (event_base_loop(main_base, EVLOOP_ONCE) != 0) {
            retval = EXIT_FAILURE;
            break;
        }
    }

    if (stop_main_loop == GRACE_STOP) {
        stop_threads();
        if (settings.memory_file != NULL) {
            restart_mmap_close();
        }
    }

    /* remove the PID file if we're a daemon */
    if (do_daemonize)
        remove_pidfile(pid_file);
    /* Clean up strdup() call for bind() address */
    if (settings.inter)
      free(settings.inter);

    /* cleanup base */
    event_base_free(main_base);

    free(meta);

    return retval;
}
```
