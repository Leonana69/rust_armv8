#![no_std]
#![feature(panic_handler)]
#![feature(panic_implementation)]
#![feature(lang_items)]
#![no_main]

// #[cfg(target_arch = "arm")]
// #[path = "arm.rs"]
// mod platform;
// pub use self::platform::*;
// extern crate volatile;
// #[macro_use]
// extern crate lazy_static;
// extern crate spin;
extern crate rlibc;
#[macro_use]
mod print;
mod reg;


#[no_mangle]
pub extern "C" fn rust_main() -> !{
	puts!("leonana\n");
    printu64!(0x13456);
    
    // let mut frame_allocator = memory::area_frame_allocator::new(
    // kernel_start as usize, kernel_end as usize, multiboot_start,
    // multiboot_end, memory_map_tag.memory_areas());
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn reset() -> ! {
    // asm!("nop");
    // extern "C" {
        
    //     static mut __bss_start: u64;
    //     static mut __bss_end: u64;
    // }

    // Zeroes the .bss
    // r0::zero_bss(&mut __bss_start, &mut __bss_end);

    // extern "Rust" {
    //     fn main() -> !;
    // }

    rust_main();
}

#[lang = "eh_personality"] extern fn eh_personality() {}
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