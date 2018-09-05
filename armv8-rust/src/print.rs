#![allow(dead_code)]
#![allow(unused_macros)]
// use volatile::Volatile;
// use core::fmt;
// use spin::Mutex;
use reg;

pub fn puts(s: &str) {
	for c in s.as_bytes() {
    	unsafe { *(reg::UART_BASE as *mut u32) = *c as u32; };
    }
}

pub fn putchar(c: char) {
	unsafe { *(reg::UART_BASE as *mut u32) = c as u32; };
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

macro_rules! printu32 {
	() => (puts!("\n"));
    ($fmt:expr) => (print::putu32($fmt));
}

macro_rules! printu64 {
	() => (puts!("\n"));
    ($fmt:expr) => (print::putu64($fmt));
}
// pub struct Buffer {
// 	pub chars: Volatile<u8>,
// }

// pub struct Writer {
//     pub out_id: u32,
//     // pub buffer: &'static mut Buffer,
// }

// impl Writer {
//     pub fn write_byte(&mut self, byte: u8) {
//         unsafe { *(reg::UART_BASE as *mut u32) = byte as u32; };
//     }
//     pub fn write_string(&mut self, s: &str) {
//         for byte in s.bytes() {
//         	match byte {
//                 // printable ASCII byte or newline
//                 0x20...0x7e | b'\n' => self.write_byte(byte),
//                 // not part of printable ASCII range
//                 _ => self.write_byte(0xfe),
//             }
//         }
//     }
// }

// impl fmt::Write for Writer {

//     fn write_str(&mut self, s: &str) -> fmt::Result {
//         self.write_string(s);
//         Ok(())
//     }

//     fn write_char(&mut self, c: char) -> fmt::Result {
//     	self.write_byte(c as u8);
//     	Ok(())
//     }
// }

// lazy_static! {
//     pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
//         out_id: 1,
//         buffer: unsafe { &mut *(reg::UART_BASE as *mut Buffer) },
//     });
// }

// macro_rules! print {
//     ($($arg:tt)*) => ($crate::print::print(format_args!($($arg)*)));
// }

// pub fn print(args: fmt::Arguments) {
//     use core::fmt::Write;
//     WRITER.lock().write_fmt(args).unwrap();
// }

// macro_rules! writee {
// 	($dst: expr, $($arg: tt)*) => ($dst.write_fmt(format_args!($($arg)*)))
// }
// macro_rules! print {
//     ($($arg:tt)*) => ({
//     	use core::fmt::Write;
//     	let mut writer = print::Writer{out_id: 0,};
//     	format!($($arg)*);
//     });
// }

// macro_rules! println {
//     ($fmt:expr) => (print!(concat!($fmt, "\n")));
//     ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
// }

