pub mod fb;
pub mod qemu_debug_port;

pub trait InternalOutputDevice {
    const NAME: &str;

    fn write_char(&self, ch: char);
    fn write_str(&self, string: &str) {
        for ch in string.chars() {
            self.write_char(ch);
        }
    }
}
