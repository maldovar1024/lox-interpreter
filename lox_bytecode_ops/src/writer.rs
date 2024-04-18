use std::mem;

use crate::codec::Write;

#[derive(Default)]
pub struct OpWriter {
    buf: Vec<u8>,
}

impl OpWriter {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn flush(&mut self) -> Vec<u8> {
        mem::take(&mut self.buf)
    }
}

impl Write for OpWriter {
    fn write(&mut self, buf: &[u8]) {
        self.buf.extend_from_slice(buf);
    }
}
