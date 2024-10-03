# Valgrind

## cargo-valgrind

- github: [cargo-valgrind](https://github.com/jfrimmel/cargo-valgrind)

```bash
cargo install cargo-valgrind
```

### Memory leaks

```bash
cargo valgrind test slabs

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.10s

       Error leaked 1.0 MiB in 1 block
        Info stack trace (user code at the bottom)
             at memalign
             at posix_memalign
             at __rdl_alloc (alloc.rs:84)
             at alloc::alloc::alloc (alloc.rs:100)
             at cache_core::slabs::Slab::new (slabs.rs:19)
             at cache_core::slabs::tests::init_slab (slabs.rs:43)
             at cache_core::slabs::tests::init_slab::{{closure}} (slabs.rs:41)
             at core::ops::function::FnOnce::call_once (function.rs:250)
             at test::__rust_begin_short_backtrace (function.rs:250)
             at test::run_test::{{closure}} (lib.rs:648)
             at std::sys_common::backtrace::__rust_begin_short_backtrace (lib.rs:599)
             at core::ops::function::FnOnce::call_once{{vtable.shim}} (mod.rs:542)
     Summary Leaked 1.0 MiB total (0 other errors)
error: test failed, to rerun pass `-p cache-core --lib`
```

