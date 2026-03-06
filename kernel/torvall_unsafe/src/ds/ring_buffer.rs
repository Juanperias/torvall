pub struct RingBuffer<T, const SIZE: usize> {
    pub buffer: [T; SIZE],
    pub idx: usize,
}

impl<T, const SIZE: usize> RingBuffer<T, SIZE> {
    pub const fn new(buffer: [T; SIZE]) -> Self {
        Self { buffer, idx: 0 }
    }
    pub fn push(&mut self, item: T) {
        self.buffer[self.idx] = item;
        self.idx = (self.idx + 1) % self.buffer.len();
    }
    pub fn write_slice(&mut self, mut items: &[T]) {
        let buffer_len = self.buffer.len();

        if items.len() > buffer_len {
            items = &items[items.len() - buffer_len..];
        }

        let items_len = items.len();

        if (self.idx + items_len) <= buffer_len {
            unsafe {
                core::ptr::copy_nonoverlapping(
                    items.as_ptr(),
                    self.buffer.as_mut_ptr().add(self.idx),
                    items_len,
                );
            }
            self.idx = (self.idx + items_len) % buffer_len;

            return;
        }

        let space_at_end = buffer_len - self.idx;
        let remaining = items_len - space_at_end;

        unsafe {
            core::ptr::copy_nonoverlapping(
                items.as_ptr(),
                self.buffer.as_mut_ptr().add(self.idx),
                space_at_end,
            );

            core::ptr::copy_nonoverlapping(
                items.as_ptr().add(space_at_end),
                self.buffer.as_mut_ptr(),
                remaining,
            );
        }
        self.idx = remaining;
    }
}
