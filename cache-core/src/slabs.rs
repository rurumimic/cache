#[allow(dead_code)]
extern "C" {
    fn abs(input: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::alloc::{alloc, dealloc, handle_alloc_error, Layout};

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
