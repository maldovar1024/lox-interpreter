#[derive(Debug)]
pub struct StringSymbol(u32);

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
