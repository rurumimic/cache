# Read memcached.c

## main function

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

