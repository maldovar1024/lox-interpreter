pub trait Write {
    fn write(&mut self, buf: &[u8]);
}

pub trait Encode<Writer: Write> {
    fn encode(&self, writer: &mut Writer);
}

impl<Writer: Write> Encode<Writer> for bool {
    fn encode(&self, writer: &mut Writer) {
        writer.write(&[u8::from(*self)]);
    }
}

impl<Writer: Write> Encode<Writer> for f64 {
    fn encode(&self, writer: &mut Writer) {
        writer.write(&self.to_le_bytes());
    }
}
