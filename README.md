# rust_armv8_hello 
Realize a bare metal system for arm with print module.

## Setup Rust
You can go this [page](https://www.rust-lang.org/en-US/install.html) or just run:  
```
curl https://sh.rustup.rs -sSf | sh
```
First install with default options (you can also directly install the nightly version), this allows you to start with normal rust programming (remember to add *$HOME/.cargo/bin* to your *$PATH*). Then, type:
```
rustup toolchain install nightly
```
to install the nightly version of rust. The nightly version of rust allows us to use some features which are essential to write the system. Then switch your default rust toolchain to the nightly version:
```
rustup default nightly-x86_64-unknown-linux-gnu
```
Install the xargo and add rust-src for building the core:
```
cargo install xargo
rustup component add rust-src
```

## GCC Toolchain
I use the aarch64-none-elf-gcc toolchain for cross compiling. You can download [here](https://github.com/arter97/aarch64-none-elf-6.1) or just clone my whole project (they are in bin and libexec). Then change the path in the Makefile:
```
CROSS_COMPILE ?= path_to_the_toolchain/bin/aarch64-none-elf-
```

## QEMU
Install the qemu:
```
sudo apt-get install qemu
```
# Run the Bare Metal System
```
make qemu
```
You should see the output:
```
Press Ctrl-A and then X to exit QEMU

qemu-system-aarch64	-M virt -cpu cortex-a57	\
					-smp 1 -m 128 \
					-nographic \
					-kernel hello.elf
Hello world!
0x123456
```


## Using GDB
If you want to debug the system, you can use the aarch64-none-elf-gdb in the toolchain. Qemu implements a gdb connector using a TCP connection. To do so, you can run the qemu with options *-s* and *-S*.
```
 -s: shorthand for -gdb tcp::1234  
 -S: freeze the CPU at startup, you can connect your gdb at this time  
```

For convenience, you can just type:
> make dqemu

Now you are running the qemu without output, the command freezes the system before executing any code and waits for a connection on the TCP port 1234. Open another terminal and run aarch64-none-elf-gdb and enter:
```
target remote localhost:1234
file hello.elf
```

if you have problem like this:

go [here](https://www.mpfr.org/mpfr-current/) to download the GNU MPFR 4.0.1 package and install it.  
if you still get above problem after install MPFR, you should check your LD_LIBRARY_PATH,