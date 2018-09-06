To figure out the data layout for architecture configure file (.json file), just run:  
```
rustup run nightly-x86_64-unknown-linux-gnu rustc --target aarch64-unknown-none --emit=llvm-ir empty.rs
```
the data layout will be in the empty.ll