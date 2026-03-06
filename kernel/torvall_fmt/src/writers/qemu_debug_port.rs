use torvall_unsafe::arch::amd64::io_port::outb;

use crate::writers::InternalOutputDevice;

pub struct QemuDebugPortOutput;

impl InternalOutputDevice for QemuDebugPortOutput {
    const NAME: &str = "QEMU_DEBUG_PORT";

    fn write_char(&self, ch: char) {
        // SAFETY: this is safe as long as you're running this in qemu which is totally true is
        // you're in debug mode
        unsafe {
            outb(0xe9, ch as u8);
        }
    }
}
