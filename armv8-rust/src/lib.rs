#![no_std]
#![feature(panic_handler)]
#![feature(panic_implementation)]
#![feature(lang_items)]
#![feature(asm)] // for insert assembly code
#![no_main]

extern crate rlibc;

#[macro_use]
mod print;

mod reg;

// system main
#[no_mangle]
pub extern "C" fn rust_main() -> ! {
	puts!("Hello world!\n");
    printu64!(0x123456);
    loop {}
}

// system reset entry
#[no_mangle]
pub unsafe extern "C" fn reset() -> ! {
    // initialization
    // ...
    // main
    rust_main();
}

#[lang = "eh_personality"] extern fn eh_personality() {}

// panic handler
use core::panic::PanicInfo;
#[panic_handler]
#[no_mangle]
pub extern fn panic(_info: &PanicInfo) -> ! {
	if let Some(location) = _info.location() {
		print!("\nPanic in ");
	    print!(location.file());
	    print!(" at line ");
	    printu32!(location.line());
	    print!("");
	}
    loop {}
 }