#[inline]
pub fn cli() {
    unsafe {
        core::arch::asm!("cli");
    }
}

#[inline]
pub fn sti() {
    unsafe {
        core::arch::asm!("sti");
    }
}

#[inline]
pub fn save_eflags() -> u64 {
    unsafe {
        let flags: u64;
        core::arch::asm!("pushfq; pop {}", out(reg) flags);

        flags
    }
}

#[inline]
pub fn restore_eflags(flags: u64) {
    unsafe {
        core::arch::asm!("push {}; popfq", in(reg) flags);
    }
}

pub struct IrqGuard(u64);

impl IrqGuard {
    pub fn new() -> Self {
        let ins = Self(save_eflags());
        cli();
        ins
    }
}

impl Drop for IrqGuard {
    fn drop(&mut self) {
        restore_eflags(self.0);
    }
}
