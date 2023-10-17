#[allow(dead_code)]
extern "C" {
    fn abs(input: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unsafe_abs() {
        unsafe {
            let result = abs(-3);
            assert_eq!(result, 3);
        }
    }
}
