fn main() {
    let mut v = vec![1, 2, 3];
    let ptr = v.as_mut_ptr();
    // ... some code that modifies the vector contents
    unsafe {
        // Directly access the vector's data through the raw pointer
        *ptr = 10;
    }
}