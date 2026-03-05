#![no_std]
#![no_main]

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    loop {}
}

#[panic_handler]
pub fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
