#![allow(dead_code)]
#![allow(unused_macros)]

use reg;

pub fn puts(s: &str) {
	for c in s.as_bytes() {
    	unsafe { *(reg::UART_BASE as *mut u32) = *c as u32; };
    }
}

pub fn putchar(c: char) {
	unsafe { *(reg::UART_BASE as *mut u32) = c as u32; };
}

pub fn putu32(u: u32) {
	if u == 0 {
		putchar('0');
		return;
	}
	puts("0x");
	let mut flag = false;
	let mut b = u;
	for _i in 0..8 {
		let mut p = (b>>28) & 0xf;
		match p {
			0...9 => p = p + 48,
			10...16 => p = p + 97 - 10,
			_ => (),
		}
		let up = p as u8 as char;
		if flag || p != 48 {
			flag = true;
			putchar(up);
		}

		b = b << 4;
	}
}

pub fn putu64(u: u64) {
	if u == 0 {
		putchar('0');
		return;
	}
	puts("0x");
	let mut flag = false;
	let mut b = u;
	for _i in 0..16 {
		let mut p = (b>>60) & 0xf;
		match p {
			0...9 => p = p + 48,
			10...16 => p = p + 97 - 10,
			_ => (),
		}
		let up = p as u8 as char;
		if flag || p != 48 {
			flag = true;
			putchar(up);
		}
		b = b << 4;
	}
}

macro_rules! puts {
	($($arg: tt)*) => (print::puts($($arg)*))
}

macro_rules! putchar {
	($($arg: tt)*) => (print::putchar($($arg)*))
}

macro_rules! print {
	() => (puts!("\n"));
    ($fmt:expr) => (puts!($fmt));
}

macro_rules! println {
	() => (puts!("\n"));
    ($fmt:expr) => (puts!(concat!($fmt, "\n")));
}


macro_rules! printu32 {
	() => (puts!("\n"));
    ($fmt:expr) => (print::putu32($fmt));
}

macro_rules! printu64 {
	() => (puts!("\n"));
    ($fmt:expr) => (print::putu64($fmt));
}
