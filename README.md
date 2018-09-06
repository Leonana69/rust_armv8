# rust_armv8_hello 
Realize a bare metal system for arm with print module.

# GCC Toolchain
I use the aarch64-none-elf-gcc toolchain for cross compiling. You can download [here](https://github.com/arter97/aarch64-none-elf-6.1)

Then change the path in the Makefile:
> CROSS_COMPILE ?= path_to_the_toolchain/bin/aarch64-none-elf-

# QEMU
Install the qemu.
