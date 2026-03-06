#![no_std]

use spin::Mutex;
use torvall_unsafe::{
    arch::amd64::irq::{IrqGuard, cli, restore_eflags, save_eflags},
    ds::ring_buffer::RingBuffer,
};

#[cfg(debug_assertions)]
use crate::writers::{InternalOutputDevice, qemu_debug_port::QemuDebugPortOutput};

pub mod writers;

const LOG_BUFFER_SIZE: usize = 1 << 17;

pub static LOG_BUFFER: Mutex<RingBuffer<u8, LOG_BUFFER_SIZE>> =
    Mutex::new(RingBuffer::new([0_u8; LOG_BUFFER_SIZE]));

#[cfg(debug_assertions)]
pub const QEMU_WRITER: QemuDebugPortOutput = QemuDebugPortOutput;

pub static mut LOG_WRITER: LogWriter = LogWriter;

pub struct LogWriter;

impl core::fmt::Write for LogWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        #[cfg(debug_assertions)]
        QEMU_WRITER.write_str(s);

        let _guard = IrqGuard::new();

        {
            let bytes = s.as_bytes();

            let mut log_buffer = LOG_BUFFER.lock();

            log_buffer.write_slice(bytes);
        }

        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        write!($crate::LogWriter, "{}", format_args!($($arg)*)).expect("Cannot format args");
    }
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n");
    };
    ($($arg:tt)*) => {
        $crate::print!($($arg)*);
        $crate::print!("\n");
    }
}
