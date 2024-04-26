use std::mem;
use thiserror::Error;

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

#[derive(Debug, Error)]
pub enum DecoderErrorDetail {
    #[error("invalid bool value `{0:#b}`")]
    InvalidBool(u8),
    #[error("no enough data, expected {expected} byte(s), remaining {rem} byte(s)")]
    NoEnoughData { expected: usize, rem: usize },
}

#[derive(Debug, Error)]
#[error("{pos}: {error}")]
pub struct DecoderError {
    pos: usize,
    error: DecoderErrorDetail,
}

impl DecoderError {
    pub fn from_detail(pos: usize, detail: DecoderErrorDetail) -> Self {
        Self { pos, error: detail }
    }
}

pub type DecodeResult<T> = Result<(T, usize), Box<DecoderErrorDetail>>;

pub trait Decode: Sized {
    fn decode(buf: &[u8]) -> DecodeResult<Self>;
}

#[inline(always)]
pub fn get_bytes<const N: usize>(buf: &[u8]) -> Result<[u8; N], Box<DecoderErrorDetail>> {
    if buf.len() < N {
        Err(Box::new(DecoderErrorDetail::NoEnoughData {
            expected: N,
            rem: buf.len(),
        }))
    } else {
        let mut res = [0; N];
        res.copy_from_slice(&buf[..N]);
        Ok(res)
    }
}

impl Decode for bool {
    fn decode(buf: &[u8]) -> DecodeResult<Self> {
        const SIZE: usize = mem::size_of::<bool>();
        Ok((
            match get_bytes::<SIZE>(buf)?[0] {
                0 => false,
                1 => true,
                _ => return Err(Box::new(DecoderErrorDetail::InvalidBool(buf[0]))),
            },
            SIZE,
        ))
    }
}

macro_rules! impl_decode {
    ($($ty: ty),*) => {
        $(impl Decode for $ty {
            fn decode(buf: &[u8]) -> DecodeResult<Self> {
                const SIZE: usize = mem::size_of::<$ty>();
                Ok((<$ty>::from_le_bytes(get_bytes::<SIZE>(buf)?), SIZE))
            }
        })*
    };
}

impl_decode! {u32, f64}
