#FILES
There are three rust files in the src up to now, lib.rs is the main file, reg.rs is for the register constants and print.rs is the print module. And two files for gcc compiler (hello.ld and startup.S)

##lib.rs
First is the rust_main funciton, it is what the system will do after booting up. Here it just print a string and an int64. You can add some funcitons here to realize the system's funtions. Then comes the reset funciton, you can put initialization functions here to initialize the system. Next is eh_personality and panic function, they are used by the failure mechanisms of the compiler. (By the way, panic_fmt is outdated, we should use panic instead.)

##print.rs
putchar is the based function, it delivers a char to the address of UART0 so that we can see the information in qemu. puts just sends multiple chars to UART0. Up to putu32, I encountered a problem that fmt::Write trait doesn't work here and I can't figure out what makes it invalid. So I have to manually print other types of variables like u32 and u64 byte by byte. At the end of print.rs I also define some macros for convenience of use.

##hello.ld
This file is used for link stage of gcc, it specificly define the entry and the base address of the final program.

##startup.S This file defines the 'frame' of the system. As the system bootups, it first comes to the contents in the startup.S. We can see in the file, it checks the cpu_id first than setup stack address and goes to the reset funciton which is defined in lib.rs.