#include <signal.h>
#include <stdio.h>

#include <netinet/in.h>
#include <sys/socket.h>

#include <event2/event.h>
#include <event2/listener.h>

static const unsigned short PORT = 9995;

static void listener_callback(struct evconnlistener *listener,
                              evutil_socket_t fd, struct sockaddr *sa,
                              int socklen, void *user_data);
static void signal_callback(evutil_socket_t, short, void *);

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

  printf("done\n");

  return 0;
}

static void listener_callback(struct evconnlistener *listener,
                              evutil_socket_t fd, struct sockaddr *sa,
                              int socklen, void *user_data) {

  // handle event
}

static void signal_callback(evutil_socket_t sig, short events,
                            void *user_data) {
  // handle signal
}
