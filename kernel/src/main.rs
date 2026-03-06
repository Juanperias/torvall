#![no_std]
#![no_main]

use core::fmt::Write;
use torvall_fmt::{println, Level};

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    println!(Level::Info, "hello {}", "from torvall");

    loop {}
}

#[panic_handler]
pub fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
