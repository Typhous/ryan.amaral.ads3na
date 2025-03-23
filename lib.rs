pub unsafe fn multiply_array(ptr: *const i32, len: usize) -> i32 {
    if len == 0 {
        panic!("Array vazio");
    }

    let mut product = unsafe { *ptr }; // Inicia com o primeiro elemento
    for i in 1..len {
        unsafe {
            product *= *ptr.offset(i as isize);
        }
    }
    product
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply_array() {
        let arr = [2, 3, 4];
        let product = unsafe { multiply_array(arr.as_ptr(), arr.len()) };
        assert_eq!(product, 24);
    }
}

fn main() {
    println!("Hello, world!");
}