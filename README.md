# rust_armv8_hello 
Realize a bare metal system for arm with print module.

# GCC Toolchain
I use the aarch64-none-elf-gcc toolchain for cross compiling. You can download [here](https://github.com/arter97/aarch64-none-elf-6.1).

Then change the path in the Makefile:
> CROSS_COMPILE ?= path_to_the_toolchain/bin/aarch64-none-elf-

# QEMU
Install the qemu.

# Using GDB
If you want to debug the system, you can use the aarch64-none-elf-gdb in the toolchain. Qemu implements a gdb connector using a TCP connection. To do so, you can run the qemu with options *-s* and *-S*. (or you can just run with *make dqemu*)
> -s: shorthand for -gdb tcp::1234
> -S: freeze the CPU at startup, you can connect your gdb at this time

Then