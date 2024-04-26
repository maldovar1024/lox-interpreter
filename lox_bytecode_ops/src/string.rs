use crate::codec::{Decode, DecodeResult, Encode, Write};

#[derive(Debug)]
pub struct StringSymbol(pub(crate) u32);

impl From<StringSymbol> for u32 {
    #[inline(always)]
    fn from(value: StringSymbol) -> Self {
        value.0
    }
}

#[derive(Debug, Default)]
pub struct StringIntern {
    strings: indexmap::IndexSet<Box<str>>,
}

impl StringIntern {
    pub fn intern(&mut self, s: &str) -> StringSymbol {
        StringSymbol(match self.strings.get_index_of(s) {
            Some(idx) => idx,
            None => self.strings.insert_full(s.to_string().into_boxed_str()).0,
        } as u32)
    }
}

impl<Writer: Write> Encode<Writer> for StringSymbol {
    fn encode(&self, writer: &mut Writer) {
        writer.write(&self.0.to_le_bytes());
    }
}

impl Decode for StringSymbol {
    fn decode(buf: &[u8]) -> DecodeResult<Self> {
        let (v, size) = u32::decode(buf)?;
        Ok((Self(v), size))
    }
}
