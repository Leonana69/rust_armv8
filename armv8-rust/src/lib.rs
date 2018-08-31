#![no_std]
#![feature(panic_handler)]
#![feature(lang_items)]
extern crate multiboot2;
// extern crate volatile;
// #[macro_use]
// extern crate lazy_static;
// extern crate spin;
extern crate rlibc;
#[macro_use]
mod print;
mod reg;

pub fn test(a: u32, b: u32, c: u32, d: u32, e: u32) -> u32
{
	a + b + c + d + e
}


#[no_mangle]
pub extern fn rust_main(multiboot_information_address: usize) {
	// let boot_info = unsafe{ multiboot2::load(multiboot_information_address) };
	// let memory_map_tag = boot_info.memory_map_tag()
 //    .expect("Memory map tag required");
	puts!("leonana\n");
    printu64!(0x13456);
    loop {}
}


#[lang = "eh_personality"] extern fn eh_personality() {}
use core::panic::PanicInfo;
#[panic_handler]
pub extern fn panic_fmt(_info: &PanicInfo) -> ! { loop {} }