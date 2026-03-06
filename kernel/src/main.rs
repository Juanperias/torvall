#![no_std]
#![no_main]

use core::fmt::Write;
use torvall_fmt::print;

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    print!("hello! {}", 2);

    loop {}
}

#[panic_handler]
pub fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
