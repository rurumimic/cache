# Architecture

- wiki: [extstore](https://github.com/memcached/memcached/wiki/Extstore#architecture)

```text
               THREADS            +            STRUCTURES
 +----------------------+         |
 |-----------|          |         |     +----------+
+------+  +--v-----+ +--v-----+   |     |hash table|
|listen|  |worker 1| |worker 2|   |     +----------+
+------+  +--------+ +--------+   |
                        +--------->     +---+
                                  |     |LRU|
+--------------+                  |     +---+
|lru maintainer+------------------>
+--------------+                  |     +--------------+
                                  |     |slab allocator|
+-----------+                     |     +--------------+
|lru crawler+--------------------->
+-----------+                     |
                                  |
+---------------+                 |
|slab page mover+----------------->
+---------------+                 |
                                  +
```

