#include <errno.h>
#include <signal.h>
#include <stdio.h>
#include <string.h>

#include <netinet/in.h>
#include <sys/socket.h>

#include <event2/buffer.h>
#include <event2/bufferevent.h>
#include <event2/event.h>
#include <event2/listener.h>
#include <event2/util.h>

static const char MESSAGE[] = "Hello, World!\n";

static const unsigned short PORT = 9995;

static void listener_callback(struct evconnlistener *listener,
                              evutil_socket_t fd, struct sockaddr *sa,
                              int socklen, void *user_data);
static void conn_write_callback(struct bufferevent *bev, void *user_data);
static void conn_event_callback(struct bufferevent *bev, short events,
                                void *user_data);
static void signal_callback(evutil_socket_t sig, short events, void *user_data);

int main(int argc, char **argv) {
  struct event_base *base;

  // event.c -> event_base_new(void)
  // @see event_base_free()
  base = event_base_new(); // @see event_base_free()
  if (base == NULL) {
    fprintf(stderr, "Could not initialize libevent\n");
    return 1;
  }

  struct sockaddr_in sin = {0};
  sin.sin_family = AF_INET;
  sin.sin_port = htons(PORT); // netinet/in.h - sys/_endian.h

  struct evconnlistener *listener;
  listener = evconnlistener_new_bind(base, listener_callback, (void *)base,
                                     LEV_OPT_REUSEABLE | LEV_OPT_CLOSE_ON_FREE,
                                     -1, (struct sockaddr *)&sin, sizeof(sin));
  if (listener == NULL) {
    fprintf(stderr, "Could not create a listener!\n");
    return 1;
  }

  struct event *signal_event;
  signal_event = evsignal_new(base, SIGINT, signal_callback, (void *)base);
  if (signal_event == NULL || event_add(signal_event, NULL) < 0) {
    fprintf(stderr, "Could not create/add a signal event!\n");
    return 1;
  }

  event_base_dispatch(base); // event dispatching loop

  evconnlistener_free(listener);
  event_free(signal_event);
  event_base_free(base);

  printf("Finished.\n");

  return 0;
}

static void listener_callback(struct evconnlistener *listener,
                              evutil_socket_t fd, struct sockaddr *sa,
                              int socklen, void *user_data) {
  struct event_base *base = user_data;

  struct bufferevent *bev;
  bev = bufferevent_socket_new(base, fd, BEV_OPT_CLOSE_ON_FREE);
  if (bev == NULL) {
    fprintf(stderr, "Error constructing bufferevent!");
    event_base_loopbreak(base);
    return;
  }

  bufferevent_setcb(bev, NULL, conn_write_callback, conn_event_callback, NULL);
  bufferevent_enable(bev, EV_WRITE);
  bufferevent_disable(bev, EV_READ);

  bufferevent_write(bev, MESSAGE, strlen(MESSAGE));
}

static void conn_write_callback(struct bufferevent *bev, void *user_data) {
  struct evbuffer *output;
  output = bufferevent_get_output(bev);
  if (evbuffer_get_length(output) == 0) {
    printf("flushed answer\n");
    bufferevent_free(bev);
  }
}

static void conn_event_callback(struct bufferevent *bev, short events,
                                void *user_data) {
  if (events & BEV_EVENT_EOF) {
    printf("Connection closed.\n");
  } else if (events & BEV_EVENT_ERROR) {
    printf("Got an error on the connection: %s\n", strerror(errno));
  }

  // Non of the other events can happen here
  // timeouts is disabled
  bufferevent_free(bev);
}

static void signal_callback(evutil_socket_t sig, short events,
                            void *user_data) {
  struct event_base *base = user_data;
  struct timeval delay = {2, 0};

  printf("Caught an interrupt signal; exiting cleanly in two seconds.\n");

  event_base_loopexit(base, &delay);
}

