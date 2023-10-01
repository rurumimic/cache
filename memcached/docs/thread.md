# Thread

- [void accept_new_conns(const bool do_accept)](#accept_new_conns)
- [static void thread_libevent_process(evutil_socket_t fd, short which, void *arg)](#thread_libevent_process)
- [void dispatch_conn_new(int sfd, enum conn_states init_state, int event_flags, int read_buffer_size, enum network_transport transport, void *ssl, uint64_t conntag, enum protocol bproto)](#dispatch_conn_new)
- [void memcached_thread_init(int nthreads, void *arg)](#memcached_thread_init)

## thread.c

### accept_new_conns

```c
/*
 * Sets whether or not we accept new connections.
 */
void accept_new_conns(const bool do_accept) {
    pthread_mutex_lock(&conn_lock);
    do_accept_new_conns(do_accept);
    pthread_mutex_unlock(&conn_lock);
}
```

### thread_libevent_process

```c
/*
 * Processes an incoming "connection event" item. This is called when
 * input arrives on the libevent wakeup pipe.
 */
// Syscalls can be expensive enough that handling a few of them once here can
// save both throughput and overall latency.
#define MAX_PIPE_EVENTS 32
static void thread_libevent_process(evutil_socket_t fd, short which, void *arg) {
    LIBEVENT_THREAD *me = arg;
    CQ_ITEM *item;
    conn *c;
    uint64_t ev_count = 0; // max number of events to loop through this run.
#ifdef HAVE_EVENTFD
    // NOTE: unlike pipe we aren't limiting the number of events per read.
    // However we do limit the number of queue pulls to what the count was at
    // the time of this function firing.
    if (read(fd, &ev_count, sizeof(uint64_t)) != sizeof(uint64_t)) {
        if (settings.verbose > 0)
            fprintf(stderr, "Can't read from libevent pipe\n");
        return;
    }
#else
    char buf[MAX_PIPE_EVENTS];

    ev_count = read(fd, buf, MAX_PIPE_EVENTS);
    if (ev_count == 0) {
        if (settings.verbose > 0)
            fprintf(stderr, "Can't read from libevent pipe\n");
        return;
    }
#endif

    for (int x = 0; x < ev_count; x++) {
        item = cq_pop(me->ev_queue);
        if (item == NULL) {
            return;
        }

        switch (item->mode) {
            case queue_new_conn:
                c = conn_new(item->sfd, item->init_state, item->event_flags,
                                   item->read_buffer_size, item->transport,
                                   me->base, item->ssl, item->conntag, item->bproto);
                if (c == NULL) {
                    if (IS_UDP(item->transport)) {
                        fprintf(stderr, "Can't listen for events on UDP socket\n");
                        exit(1);
                    } else {
                        if (settings.verbose > 0) {
                            fprintf(stderr, "Can't listen for events on fd %d\n",
                                item->sfd);
                        }
#ifdef TLS
                        if (item->ssl) {
                            SSL_shutdown(item->ssl);
                            SSL_free(item->ssl);
                        }
#endif
                        close(item->sfd);
                    }
                } else {
                    c->thread = me;
                    conn_io_queue_setup(c);
#ifdef TLS
                    if (settings.ssl_enabled && c->ssl != NULL) {
                        assert(c->thread && c->thread->ssl_wbuf);
                        c->ssl_wbuf = c->thread->ssl_wbuf;
                    }
#endif
                }
                break;
            case queue_pause:
                /* we were told to pause and report in */
                register_thread_initialized();
                break;
            case queue_timeout:
                /* a client socket timed out */
                conn_close_idle(conns[item->sfd]);
                break;
            case queue_redispatch:
                /* a side thread redispatched a client connection */
                conn_worker_readd(conns[item->sfd]);
                break;
            case queue_stop:
                /* asked to stop */
                event_base_loopexit(me->base, NULL);
                break;
            case queue_return_io:
                /* getting an individual IO object back */
                conn_io_queue_return(item->io);
                break;
#ifdef PROXY
            case queue_proxy_reload:
                proxy_worker_reload(settings.proxy_ctx, me);
                break;
#endif
        }

        cqi_free(me->ev_queue, item);
    }
}
```

### dispatch_conn_new

```c
/*
 * Dispatches a new connection to another thread. This is only ever called
 * from the main thread, either during initialization (for UDP) or because
 * of an incoming connection.
 */
void dispatch_conn_new(int sfd, enum conn_states init_state, int event_flags,
                       int read_buffer_size, enum network_transport transport, void *ssl,
                       uint64_t conntag, enum protocol bproto) {
    CQ_ITEM *item = NULL;
    LIBEVENT_THREAD *thread;

    if (!settings.num_napi_ids)
        thread = select_thread_round_robin();
    else
        thread = select_thread_by_napi_id(sfd);

    item = cqi_new(thread->ev_queue);
    if (item == NULL) {
        close(sfd);
        /* given that malloc failed this may also fail, but let's try */
        fprintf(stderr, "Failed to allocate memory for connection object\n");
        return;
    }

    item->sfd = sfd;
    item->init_state = init_state;
    item->event_flags = event_flags;
    item->read_buffer_size = read_buffer_size;
    item->transport = transport;
    item->mode = queue_new_conn;
    item->ssl = ssl;
    item->conntag = conntag;
    item->bproto = bproto;

    MEMCACHED_CONN_DISPATCH(sfd, (int64_t)thread->thread_id);
    notify_worker(thread, item);
}
```

### memcached_thread_init

```c
/*
 * Initializes the thread subsystem, creating various worker threads.
 *
 * nthreads  Number of worker event handler threads to spawn
 */
void memcached_thread_init(int nthreads, void *arg) {
    int         i;
    int         power;

    for (i = 0; i < POWER_LARGEST; i++) {
        pthread_mutex_init(&lru_locks[i], NULL);
    }
    pthread_mutex_init(&worker_hang_lock, NULL);

    pthread_mutex_init(&init_lock, NULL);
    pthread_cond_init(&init_cond, NULL);

    /* Want a wide lock table, but don't waste memory */
    if (nthreads < 3) {
        power = 10;
    } else if (nthreads < 4) {
        power = 11;
    } else if (nthreads < 5) {
        power = 12;
    } else if (nthreads <= 10) {
        power = 13;
    } else if (nthreads <= 20) {
        power = 14;
    } else {
        /* 32k buckets. just under the hashpower default. */
        power = 15;
    }

    if (power >= hashpower) {
        fprintf(stderr, "Hash table power size (%d) cannot be equal to or less than item lock table (%d)\n", hashpower, power);
        fprintf(stderr, "Item lock table grows with `-t N` (worker threadcount)\n");
        fprintf(stderr, "Hash table grows with `-o hashpower=N` \n");
        exit(1);
    }

    item_lock_count = hashsize(power);
    item_lock_hashpower = power;

    item_locks = calloc(item_lock_count, sizeof(pthread_mutex_t));
    if (! item_locks) {
        perror("Can't allocate item locks");
        exit(1);
    }
    for (i = 0; i < item_lock_count; i++) {
        pthread_mutex_init(&item_locks[i], NULL);
    }

    threads = calloc(nthreads, sizeof(LIBEVENT_THREAD));
    if (! threads) {
        perror("Can't allocate thread descriptors");
        exit(1);
    }

    for (i = 0; i < nthreads; i++) {
#ifdef HAVE_EVENTFD
        threads[i].notify_event_fd = eventfd(0, EFD_NONBLOCK);
        if (threads[i].notify_event_fd == -1) {
            perror("failed creating eventfd for worker thread");
            exit(1);
        }
#else
        int fds[2];
        if (pipe(fds)) {
            perror("Can't create notify pipe");
            exit(1);
        }

        threads[i].notify_receive_fd = fds[0];
        threads[i].notify_send_fd = fds[1];
#endif
#ifdef EXTSTORE
        threads[i].storage = arg;
#endif
        threads[i].thread_baseid = i;
        setup_thread(&threads[i]);
        /* Reserve three fds for the libevent base, and two for the pipe */
        stats_state.reserved_fds += 5;
    }

    /* Create threads after we've done all the libevent setup. */
    for (i = 0; i < nthreads; i++) {
        create_worker(worker_libevent, &threads[i]);
    }

    /* Wait for all the threads to set themselves up before returning. */
    pthread_mutex_lock(&init_lock);
    wait_for_thread_registration(nthreads);
    pthread_mutex_unlock(&init_lock);
}
```
