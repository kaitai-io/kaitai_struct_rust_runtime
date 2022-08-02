#![allow(unused)]

use byteorder::{BigEndian, ByteOrder, LittleEndian};
use std::{cell::RefCell, string::FromUtf16Error};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Needed {
    Size(usize),
    Unknown,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum KError {
    Incomplete(Needed),
    EmptyIterator,
    Encoding { desc: String },
    MissingInstanceValue,
    MissingRoot,
    MissingParent,
    ReadBitsTooLarge { requested: usize },
    UnexpectedContents { actual: Vec<u8> },
    ValidationNotEqual(String),
    UnknownVariant(i64),
    EncounteredEOF,
}
pub type KResult<T> = Result<T, KError>;

pub trait KStruct<'r, 's: 'r>: Default {
    type Root: KStruct<'r, 's>;
    type ParentStack;

    /// Parse this struct (and any children) from the supplied stream
    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>,
    ) -> KResult<()>;

    /// helper function to read struct
    fn read_into<S: KStream, T: KStruct<'r, 's> + Default>(
        _io: &'s S,
        _root: Option<&'r T::Root>,
        _parent: Option<TypedStack<T::ParentStack>>,
    ) -> KResult<T> {
        let mut t = T::default();
        t.read(_io, _root, _parent)?;
        Ok(t)
    }
}

/// Dummy struct used to indicate an absence of value; needed for
/// root structs to satisfy the associated type bounds in the
/// `KStruct` trait.
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct KStructUnit;
impl KStructUnit {
    pub fn parent_stack() -> TypedStack<KStructUnit> {
        TypedStack { current: (KStructUnit) }
    }
}
impl<'r, 's: 'r> KStruct<'r, 's> for KStructUnit {
    type Root = KStructUnit;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>,
    ) -> KResult<()> {
        Ok(())
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct TypedStack<C> {
    current: C,
}
impl<C> TypedStack<C>
where
    C: Clone,
{
    fn clone(&self) -> Self {
        TypedStack {
            current: self.current.clone(),
        }
    }
    pub fn push<N>(&self, next: N) -> TypedStack<(N, C)> {
        TypedStack {
            current: (next, self.current.clone()),
        }
    }
}
impl<C, P> TypedStack<(C, P)>
where
    C: Clone,
    P: Clone,
{
    pub fn peek(&self) -> &C {
        &self.current.0
    }

    pub fn pop(&self) -> TypedStack<P> {
        TypedStack {
            current: (self.current.clone().1),
        }
    }
}

pub trait KStream {
    fn is_eof(&self) -> bool;
    fn seek(&self, position: usize) -> KResult<()>;
    fn pos(&self) -> usize;
    fn size(&self) -> usize;

    fn read_s1(&self) -> KResult<i8> {
        Ok(self.read_bytes(1)?[0] as i8)
    }
    fn read_s2be(&self) -> KResult<i16> {
        Ok(BigEndian::read_i16(self.read_bytes(2)?))
    }
    fn read_s4be(&self) -> KResult<i32> {
        Ok(BigEndian::read_i32(self.read_bytes(4)?))
    }
    fn read_s8be(&self) -> KResult<i64> {
        Ok(BigEndian::read_i64(self.read_bytes(8)?))
    }
    fn read_s2le(&self) -> KResult<i16> {
        Ok(LittleEndian::read_i16(self.read_bytes(2)?))
    }
    fn read_s4le(&self) -> KResult<i32> {
        Ok(LittleEndian::read_i32(self.read_bytes(4)?))
    }
    fn read_s8le(&self) -> KResult<i64> {
        Ok(LittleEndian::read_i64(self.read_bytes(8)?))
    }

    fn read_u1(&self) -> KResult<u8> {
        Ok(self.read_bytes(1)?[0] as u8)
    }
    fn read_u2be(&self) -> KResult<u16> {
        Ok(BigEndian::read_u16(self.read_bytes(2)?))
    }
    fn read_u4be(&self) -> KResult<u32> {
        Ok(BigEndian::read_u32(self.read_bytes(4)?))
    }
    fn read_u8be(&self) -> KResult<u64> {
        Ok(BigEndian::read_u64(self.read_bytes(8)?))
    }
    fn read_u2le(&self) -> KResult<u16> {
        Ok(LittleEndian::read_u16(self.read_bytes(2)?))
    }
    fn read_u4le(&self) -> KResult<u32> {
        Ok(LittleEndian::read_u32(self.read_bytes(4)?))
    }
    fn read_u8le(&self) -> KResult<u64> {
        Ok(LittleEndian::read_u64(self.read_bytes(8)?))
    }

    fn read_f4be(&self) -> KResult<f32> {
        Ok(BigEndian::read_f32(self.read_bytes(4)?))
    }
    fn read_f8be(&self) -> KResult<f64> {
        Ok(BigEndian::read_f64(self.read_bytes(8)?))
    }
    fn read_f4le(&self) -> KResult<f32> {
        Ok(LittleEndian::read_f32(self.read_bytes(4)?))
    }
    fn read_f8le(&self) -> KResult<f64> {
        Ok(LittleEndian::read_f64(self.read_bytes(8)?))
    }

    fn align_to_byte(&self) -> KResult<()>;
    fn read_bits_int(&self, n: usize) -> KResult<u64>;

    fn read_bytes(&self, len: usize) -> KResult<&[u8]>;
    fn read_bytes_full(&self) -> KResult<&[u8]>;
    fn read_bytes_term(
        &self,
        term: u8,
        include: bool,
        consume: bool,
        eos_error: bool,
    ) -> KResult<&[u8]>;

    fn ensure_fixed_contents(&self, expected: &[u8]) -> KResult<&[u8]> {
        let actual = self.read_bytes(expected.len())?;
        if actual == expected {
            Ok(actual)
        } else {
            // Return what the actual contents were; our caller provided us
            // what was expected so we don't need to return it, and it makes
            // the lifetimes way easier
            Err(KError::UnexpectedContents { actual: actual.to_vec() })
        }
    }

    /// Return a byte array that is sized to exclude all trailing instances of the
    /// padding character.
    fn bytes_strip_right<'a>(&'a self, bytes: &'a [u8], pad: u8) -> &'a [u8] {
        let mut new_len = bytes.len();
        while new_len > 0 && bytes[new_len - 1] == pad {
            new_len -= 1;
        }
        &bytes[..new_len]
    }

    /// Return a byte array that contains all bytes up until the
    /// termination byte. Can optionally include the termination byte as well.
    fn bytes_terminate<'a>(&'a self, bytes: &'a [u8], term: u8, include_term: bool) -> &'a [u8] {
        let mut new_len = 0;
        while bytes[new_len] != term && new_len < bytes.len() {
            new_len += 1;
        }

        if include_term && new_len < bytes.len() {
            new_len += 1;
        }

        &bytes[..new_len]
    }

    fn process_xor_one(bytes: &[u8], key: u8) -> Vec<u8> {
        let mut res = bytes.to_vec();
        for i in res.iter_mut() {
            *i = *i ^ key;
        }
        return res;
    }

    fn process_xor_many(bytes: &[u8], key: &[u8]) -> Vec<u8> {
        let mut res = bytes.to_vec();
        let mut ki = 0;
        for i in res.iter_mut() {
            *i = *i ^ key[ki];
            ki = ki + 1;
            if (ki >= key.len()) {
                ki = 0;
            }
        }
        return res;
    }

    fn process_rotate_left(bytes: &[u8], amount: u8) -> Vec<u8> {
        let mut res = bytes.to_vec();
        for i in res.iter_mut() {
            *i = (*i << amount) | (*i >> (8 - amount));
        }
        return res;
    }
}

#[derive(Default)]
struct BytesReaderState {
    pos: usize,
    bits: u64,
    bits_left: i64,
}
pub struct BytesReader<'a> {
    state: RefCell<BytesReaderState>,
    bytes: &'a [u8],
}
impl<'a> BytesReader<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        BytesReader {
            state: RefCell::new(BytesReaderState::default()),
            bytes,
        }
    }
}
impl<'a> KStream for BytesReader<'a> {
    fn is_eof(&self) -> bool {
        self.pos() == self.size()
    }

    fn seek(&self, position: usize) -> KResult<()> {
        if position >= self.bytes.len() {
            return Err(KError::Incomplete(Needed::Size(position - self.pos())));
        }
        self.state.borrow_mut().pos = position;
        Ok(())
    }

    fn pos(&self) -> usize {
        self.state.borrow().pos
    }

    fn size(&self) -> usize {
        self.bytes.len()
    }

    fn align_to_byte(&self) -> KResult<()> {
        let mut inner = self.state.borrow_mut();
        inner.bits = 0;
        inner.bits_left = 0;

        Ok(())
    }

    fn read_bits_int(&self, n: usize) -> KResult<u64> {
        if n > 64 {
            return Err(KError::ReadBitsTooLarge { requested: n });
        }

        let n = n as i64;
        let bits_needed = n - self.state.borrow().bits_left;
        if bits_needed > 0 {
            // 1 bit => 1 byte
            // 8 bits => 1 byte
            // 9 bits => 2 bytes
            let bytes_needed = ((bits_needed - 1) / 8) + 1;
            // Need to be careful here, because `read_bytes` will borrow our state as mutable,
            // which panics if we're currently holding a borrow
            let buf = self.read_bytes(bytes_needed as usize)?;
            let mut inner = self.state.borrow_mut();
            for b in buf {
                inner.bits <<= 8;
                inner.bits |= *b as u64;
                inner.bits_left += 8;
            }
        }

        let mut inner = self.state.borrow_mut();
        let mut mask = (1u64 << n) - 1;
        let shift_bits = inner.bits_left - n;
        mask <<= shift_bits;

        let result: u64 = (inner.bits & mask) >> shift_bits;

        inner.bits_left -= n;
        mask = (1u64 << inner.bits_left) - 1;
        inner.bits &= mask;

        Ok(result)
    }

    fn read_bytes(&self, len: usize) -> KResult<&[u8]> {
        let cur_pos = self.state.borrow().pos;
        if len + cur_pos > self.size() {
            return Err(KError::Incomplete(Needed::Size(
                len + cur_pos - self.size(),
            )));
        }

        self.state.borrow_mut().pos += len;
        Ok(&self.bytes[cur_pos..cur_pos + len])
    }

    fn read_bytes_full(&self) -> KResult<&[u8]> {
        let cur_pos = self.state.borrow().pos;
        self.state.borrow_mut().pos = self.size();
        Ok(&self.bytes[cur_pos..self.size()])

    }

    fn read_bytes_term(&self, term: u8, include: bool, consume: bool, eos_error: bool)
        -> KResult<&[u8]> {
        let pos = self.state.borrow().pos;
        let mut new_len = pos;
        while new_len < self.bytes.len() && self.bytes[new_len] != term {
            new_len += 1;
        }

        if new_len == self.bytes.len() {
            if eos_error {
                return Err(KError::EncounteredEOF);
            }
            Ok(&self.bytes[pos..])
        } else {
            // consume terminator?
            self.state.borrow_mut().pos = new_len + consume as usize;
            // but return or not 'term' symbol depend on 'include' flag
            Ok(&self.bytes[pos..new_len + include as usize])
        }
    }
}

use encoding::{Encoding, DecoderTrap};
use encoding::label::encoding_from_whatwg_label;

pub fn decode_string<'a>(
     bytes: &'a [u8],
     label: &'a str
) -> KResult<String> {

    if let Some(enc) = encoding_from_whatwg_label(label) {
        return enc.decode(bytes, DecoderTrap::Replace).map_err(|e| KError::Encoding { desc: e.to_string() });
    }

    Err(KError::Encoding{ desc: format!("decode_string: unknown WHATWG Encoding standard: {}", label)})
}

macro_rules! kf_max {
    ($i: ident, $t: ty) => {
        pub fn $i<'a>(first: Option<&'a $t>, second: &'a $t) -> Option<&'a $t> {
            if second.is_nan() {
                first
            } else if first.is_none() {
                Some(second)
            } else {
                if first.unwrap() < second {
                    Some(second)
                } else {
                    first
                }
            }
        }
    };
}
kf_max!(kf32_max, f32);
kf_max!(kf64_max, f64);

macro_rules! kf_min {
    ($i: ident, $t: ty) => {
        pub fn $i<'a>(first: Option<&'a $t>, second: &'a $t) -> Option<&'a $t> {
            if second.is_nan() {
                first
            } else if first.is_none() {
                Some(second)
            } else {
                if first.unwrap() < second {
                    first
                } else {
                    Some(second)
                }
            }
        }
    };
}
kf_min!(kf32_min, f32);
kf_min!(kf64_min, f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_strip_right() {
        let b = [1, 2, 3, 4, 5, 5, 5, 5];
        let reader = BytesReader::new(&b[..]);
        let c = reader.bytes_strip_right(&b, 5);

        assert_eq!([1, 2, 3, 4], c);
    }

    #[test]
    fn basic_read_bytes() {
        let b = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let reader = BytesReader::new(&b[..]);

        assert_eq!(reader.read_bytes(4).unwrap(), &[1, 2, 3, 4]);
        assert_eq!(reader.read_bytes(3).unwrap(), &[5, 6, 7]);
        assert_eq!(
            reader.read_bytes(4).unwrap_err(),
            KError::Incomplete(Needed::Size(3))
        );
        assert_eq!(reader.read_bytes(1).unwrap(), &[8]);
    }

    #[test]
    fn read_bits_single() {
        let b = vec![0x80];
        let reader = BytesReader::new(&b[..]);

        assert_eq!(reader.read_bits_int(1).unwrap(), 1);
    }

    #[test]
    fn read_bits_multiple() {
        // 0xA0
        let b = vec![0b10100000];
        let reader = BytesReader::new(&b[..]);

        assert_eq!(reader.read_bits_int(1).unwrap(), 1);
        assert_eq!(reader.read_bits_int(1).unwrap(), 0);
        assert_eq!(reader.read_bits_int(1).unwrap(), 1);
    }

    #[test]
    fn read_bits_large() {
        let b = vec![0b10100000];
        let reader = BytesReader::new(&b[..]);

        assert_eq!(reader.read_bits_int(3).unwrap(), 5);
    }

    #[test]
    fn read_bits_span() {
        let b = vec![0x01, 0x80];
        let reader = BytesReader::new(&b[..]);

        assert_eq!(reader.read_bits_int(9).unwrap(), 3);
    }

    #[test]
    fn read_bits_too_large() {
        let b: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let reader = BytesReader::new(&b[..]);

        assert_eq!(
            reader.read_bits_int(65).unwrap_err(),
            KError::ReadBitsTooLarge { requested: 65 }
        )
    }

    #[test]
    fn stack_clone() {
        let t = TypedStack { current: () };
        let t2: TypedStack<(u8, ())> = t.push(12);
        let t3: TypedStack<(u16, (u8, ()))> = t2.push(14);

        assert_eq!(*t3.peek(), 14);
        assert_eq!(*t3.pop().peek(), *t2.peek());
    }

    #[test]
    fn read_bytes_term() {
        let b = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let reader = BytesReader::new(&b[..]);

        assert_eq!(reader.read_bytes_term(3, false, false, false).unwrap(), &[1, 2]);
        assert_eq!(reader.read_bytes_term(3, true, false, true).unwrap(), &[3]);
        assert_eq!(reader.read_bytes_term(3, false, true, true).unwrap(), &[]);
        assert_eq!(reader.read_bytes_term(5, true, true, true).unwrap(), &[4, 5]);
        assert_eq!(reader.read_bytes_term(8, false, false, true).unwrap(), &[6, 7]);
        assert_eq!(reader.read_bytes_term(11, false, true, true).unwrap_err(), KError::EncounteredEOF);
        assert_eq!(reader.read_bytes_term(9, true, true, false).unwrap(), &[8, 9]);
        assert_eq!(reader.read_bytes_term(10, true, false, false).unwrap(), &[10]);
    }

    #[test]
    fn process_xor_one() {
        let b = vec![0x66];
        let reader = BytesReader::new(&b[..]);
        fn as_stream_trait<S: KStream>(_io: &S) {
            let res = S::process_xor_one(_io.read_bytes(1).unwrap(), 3);
            assert_eq!(0x65, res[0]);
        }
        as_stream_trait(&reader);
    }

    #[test]
    fn process_xor_many() {
        let b = vec![0x66, 0x6F];
        let reader = BytesReader::new(&b[..]);
        fn as_stream_trait<S: KStream>(_io: &S) {
            let key : Vec<u8> = vec![3, 3];
            let res = S::process_xor_many(_io.read_bytes(2).unwrap(), &key);
            assert_eq!(vec![0x65, 0x6C], res);
        }
        as_stream_trait(&reader);
    }

    #[test]
    fn process_rotate_left() {
        let b = vec![0x09, 0xAC];
        let reader = BytesReader::new(&b[..]);
        fn as_stream_trait<S: KStream>(_io: &S) {
            let res = S::process_rotate_left(_io.read_bytes(2).unwrap(), 3);
            let expected : Vec<u8> = vec![0x48, 0x65];
            assert_eq!(expected, res);
        }
        as_stream_trait(&reader);
    }

    #[test]
    fn basic_seek() {
        let b = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let reader = BytesReader::new(&b[..]);

        assert_eq!(reader.read_bytes(4).unwrap(), &[1, 2, 3, 4]);
        let pos = reader.pos();
        reader.seek(1).unwrap();
        assert_eq!(reader.read_bytes(4).unwrap(), &[2, 3, 4, 5]);
        reader.seek(pos).unwrap();
        assert_eq!(reader.read_bytes(4).unwrap(), &[5, 6, 7, 8]);
        assert_eq!(reader.seek(9).unwrap_err(),
            KError::Incomplete(Needed::Size(1)));
    }
}
