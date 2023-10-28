#include <jemalloc/jemalloc.h>
#include <stdlib.h>
#include <stdio.h>

void do_something(size_t i) {
  // Leak some memory.
  malloc(i * 100);
}

void clean(size_t i) {
  int * p = malloc(i * 100);
  if (p == NULL) {
    perror("malloc");
    exit(1);
  }

  free(p);
}

int main(int argc, char **argv) {
  for (size_t i = 0; i < 1000; i++) {
    // do_something(i);
    clean(i);
  }

  // Dump allocator statistics to stderr.
  malloc_stats_print(NULL, NULL, NULL);

  return 0;
}

