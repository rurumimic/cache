use std::alloc::{alloc, dealloc, handle_alloc_error, Layout};

#[allow(dead_code)]
extern "C" {
    fn abs(input: i32) -> i32;
}

pub struct Slab {
    pub stack: *mut u8,
    pub layout: Layout,
}

impl Slab {
    #[inline(never)]
    pub fn new() -> Self {
        let stack_size: usize = 1 * 1024 * 1024; // 2MB
        let page_size = 4 * 1024; // 4KB
        let layout = Layout::from_size_align(stack_size, page_size).unwrap();
        let stack: *mut u8 = unsafe { alloc(layout) };
        if stack.is_null() {
            handle_alloc_error(layout);
        }
        Self { stack, layout }
    }
}

impl Drop for Slab {
    fn drop(&mut self) {
        unsafe {
            dealloc(self.stack, self.layout);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_slab() {
        let slab = Slab::new();
        dbg!(slab.stack);
        dbg!(slab.layout);

        assert!(!slab.stack.is_null(), "stack is null");

        assert_eq!(slab.layout.size(), 1 * 1024 * 1024);
        assert_eq!(slab.layout.align(), 4 * 1024);
    }

    #[test]
    fn unsafe_abs() {
        unsafe {
            let result = abs(-3);
            assert_eq!(result, 3);
        }
    }

    #[test]
    fn unsafe_array() {
        unsafe {
            let layout = Layout::new::<u16>();
            let ptr = alloc(layout);
            if ptr.is_null() {
                handle_alloc_error(layout);
            }
            *(ptr as *mut u16) = 42;
            assert_eq!(*(ptr as *mut u16), 42);

            dealloc(ptr, layout);
        }
    }
}
