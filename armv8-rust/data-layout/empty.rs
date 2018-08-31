#![feature(no_core)]
#![crate_type = "rlib"]
#![no_core]
// rustup run nightly-x86_64-unknown-linux-gnu rustc --target aarch64-unknown-none --emit=llvm-ir empty.rs
// you can get data layout in empty.ll