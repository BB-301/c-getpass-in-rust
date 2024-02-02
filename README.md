# Using C's `getpass` function in Rust

A simple wrapper library that allows using the non-standard `getpass` C function, which is available on some UNIX-like systems such as, for instance, various GNU/Linux distributions and macOS.

## Quick example

```rust
use c_getpass::getpass;

fn main() {
    // Using a prompt
    let max_len: usize = 5;
    let prompt = Some("Enter a secret value: ");
    let secret_value = getpass(max_len, prompt).expect("a multi-byte character was probably split");
    assert!(secret_value.len() <= max_len);
    println!("secret_value[{}] = {}", secret_value.len(), secret_value);

    // Without a prompt
    let max_len: usize = 50;
    let secret_value = getpass(max_len, <Option<String>>::None)
        .expect("a multi-byte character was probably split");
    assert!(secret_value.len() <= max_len);
    println!("secret_value[{}] = {}", secret_value.len(), secret_value);
}
```

## Disclaimer and word of caution

I created this library for fun, as an opportunity to experiment with the concept of [FFI bindings](https://doc.rust-lang.org/nomicon/ffi.html) in Rust. Before using this library, you should carefully read `man 3 getpass` on the target system, as well as review the source code in both [./src/lib.rs](https://github.com/BB-301/c-getpass-in-rust/blob/main/src/lib.rs) and [./c/my_lib.c](https://github.com/BB-301/c-getpass-in-rust/blob/main/c/my_lib.c).

## Contact

If you have any questions, if you find bugs, or if you have suggestions for this project, please feel free to contact me by opening an issue on the [repository](https://github.com/BB-301/c-getpass-in-rust/issues).

## License

This project is released under the [MIT License](https://github.com/BB-301/c-getpass-in-rust/blob/main/LICENSE).

## Copyright

Copyright (c) 2024 BB-301 (fw3dg3@gmail.com)
