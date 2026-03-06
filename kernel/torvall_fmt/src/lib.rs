#![no_std]

use spin::Mutex;
use torvall_unsafe::{
    arch::amd64::irq::IrqGuard,
    ds::ring_buffer::RingBuffer,
};

#[cfg(debug_assertions)]
use crate::writers::{InternalOutputDevice, qemu_debug_port::QemuDebugPortOutput};

pub mod writers;

pub enum Level {
    Err,
    Warn,
    Info,
    Debug
}

impl core::fmt::Display for Level {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Err => f.write_str("[err]"),
            Self::Warn => f.write_str("[warn]"),
            Self::Info => f.write_str("[info]"),
            Self::Debug => f.write_str("[debug]"),
        }
    }
}

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
    ($level: expr, $($arg:tt)*) => {
        write!($crate::LogWriter, "{} {}", $level, format_args!($($arg)*)).expect("Cannot format args");
    }
}

#[macro_export]
macro_rules! println {
    () => {
        write!($crate::LogWriter, "\n").expect("Cannot format args");
    };
    ($level: expr, $($arg:tt)*) => {
        $crate::print!($level, $($arg)*);
        write!($crate::LogWriter, "\n").expect("Cannot format args");

    }
}
