#include <sys/types.h>

#include <event2/event-config.h>

#include <sys/stat.h>
#include <sys/queue.h>
#include <sys/time.h>
#include <unistd.h>
#include <errno.h>
#include <fcntl.h>
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <event2/event.h>

int called = 0;

static void signal_cb(evutil_socket_t fd, short event, void *arg) {
  struct event *signal = arg;

  printf("signal_cb: got signal %d\n", event_get_signal(signal));

  if (called >= 2) {
    event_del(signal);
  }

  called++;
}

int main(int argc, char **argv) {
  struct event *signal_int = NULL;
  struct event_base *base;

  /* Initialize the event library */
  base = event_base_new();
  if (base == NULL) {
    fprintf(stderr, "Could not initialize libevent!\n");
    return 1;
  }

  /* Initialize one event */
  signal_int = evsignal_new(base, SIGINT, signal_cb, event_self_cbarg());
  if (signal_int == NULL) {
    fprintf(stderr, "Could not initialize signal!\n");
    event_base_free(base);
    return 2;
  }

  if (event_add(signal_int, NULL) == -1) {
    event_free(signal_int);
    event_base_free(base);
    return 3;
  }

  event_base_dispatch(base);

  event_free(signal_int);
  event_base_free(base);

  printf("Finished.\n");

  return 0;
}

