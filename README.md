# Unsafe Raw Pointer Access in Rust Vector

This repository demonstrates a common error in Rust: using raw pointers to access vector data directly without proper handling.  This can lead to undefined behavior, memory corruption, and crashes if not done carefully.

The `bug.rs` file contains the unsafe code.  The `bugSolution.rs` file shows a safer alternative using Rust's safe abstractions.

**Note:** This is for educational purposes to illustrate potential pitfalls.  Always favor safe abstractions in Rust unless absolutely necessary to use unsafe code.