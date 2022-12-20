#![allow(unused)]

use byteorder::{BigEndian, ByteOrder, LittleEndian};
use unicode_segmentation::UnicodeSegmentation;
use std::{  {rc::{Rc, Weak}, 
            cell::RefCell, string::FromUtf16Error},
            io::Read,
            ops::{Deref, DerefMut}};
use flate2::read::ZlibDecoder;
use once_cell::unsync::OnceCell;

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
    IoError{ desc: String },
    CastError,
    UndecidedEndiannessError(String),
}
pub type KResult<T> = Result<T, KError>;

pub trait CustomDecoder {
    fn decode(&self, bytes: &[u8]) -> Vec<u8>;
}

#[derive(Debug)]
enum Shared<T> {
    Empty,
    Main(Rc<T>),
    Slave(Weak<T>),
}

impl<T> Default for Shared<T> {
    fn default() -> Self {
        Shared::Empty
    }
}

#[derive(Default, Debug)]
pub struct SharedType<T>(RefCell<Shared<T>>);

impl<T: Default + Clone> SharedType<T> {
    pub fn new(t:& T) -> Self {
        Self(RefCell::new(Shared::Main(Rc::new(t.clone()))))
    }

    pub fn get(&self) -> Rc<T> {
        match *self.0.borrow()  {
            Shared::Main(ref rc) => 
                Rc::clone(rc),
            Shared::Slave(ref weak) => {
                let x = weak.upgrade().unwrap();
                Rc::clone(&x)
            },
            Shared::Empty => 
                panic!("attempt to get Shared::Empty"),
        }
    }

    pub fn set(&self, rc: Rc<T>) {
        let mut x = self.0.borrow_mut();
        match *x {
            Shared::Main(ref rc) => 
                *x = Shared::Main(rc.clone()),
            Shared::Slave(ref weak) => {
                let y = weak.upgrade().unwrap().clone();
                *x = Shared::Slave(Rc::downgrade(&y));
            },
            Shared::Empty => 
                panic!("attempt to set Shared::Empty"),
        }
    }
}

//overflow evaluating the requirement `SharedType<RootStruct>: PartialEq`
// impl<T, U: PartialEq> PartialEq<U> for SharedType<T> {
//     fn eq(&self, other: &U) -> bool {
//         std::unimplemented!()//self.get().eq(other)
//     }
// }

// impl<T, U: PartialOrd> PartialOrd<U> for SharedType<T> {
//     fn partial_cmp(&self, other: &U) -> Option<std::cmp::Ordering> {
//         std::unimplemented!()
//     }
// }

impl<T: Clone> Clone for SharedType<T> {
    fn clone(&self) -> Self {
        unimplemented!();
        SharedType::<T>::clone(self)
    }
}

pub trait KStruct<'r, 's: 'r>: Default + Clone {
    type Root: KStruct<'r, 's>;
    type Parent: KStruct<'r, 's>;

    /// Parse this struct (and any children) from the supplied stream
    fn read<S: KStream>(
        &self,
        _io: &'s S,
        _root: SharedType<Self::Root>,
        _parent: SharedType<Self::Parent>,
    ) -> KResult<()>;

    /// helper function to read struct
    fn read_into<S: KStream, T: KStruct<'r, 's> + Default>(
        _io: &'s S,
        _root: Option<SharedType<T::Root>>,
        _parent: Option<SharedType<T::Parent>>,
    ) -> KResult<T> {
        let mut root: SharedType<T::Root>;
        if let Some(rc) = _root {
            root = rc;
        } else {
            let r = T::Root::default();
            root = SharedType::<T::Root>::new(&r);
        }

        let mut parent: SharedType<T::Parent>;
        if let Some(par) = _parent {
            parent = par;
        } else {
            let p = T::Parent::default();
            parent = SharedType::<T::Parent>::new(&p);
        }
        
        let t = T::default();
        t.read(_io, root, parent)?;
        Ok(t)
    }
}

use std::{fs, path::Path};

impl From<std::io::Error> for KError {
    fn from(err: std::io::Error) -> Self {
        Self::IoError{ desc: err.to_string() }            
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
    fn read_bits_int_be(&self, n: usize) -> KResult<u64>;
    fn read_bits_int_le(&self, n: usize) -> KResult<u64>;

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

    fn process_zlib(bytes: &[u8]) -> Vec<u8> {
        let mut dec = ZlibDecoder::new(bytes);
        let mut dec_bytes = Vec::new();
        dec.read_to_end(&mut dec_bytes);
        dec_bytes
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
        if self.state.borrow().bits_left > 0 {
            return false;
        }
        self.pos() == self.size()
    }

    fn seek(&self, position: usize) -> KResult<()> {
        if position > self.bytes.len() {
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

    fn read_bits_int_be(&self, n: usize) -> KResult<u64> {
        let mut res : u64 = 0;

        if n > 64 {
            return Err(KError::ReadBitsTooLarge { requested: n });
        }

        let n = n as i64;
        let bits_needed = n - self.state.borrow().bits_left;
        self.state.borrow_mut().bits_left = -bits_needed & 7;

        if bits_needed > 0 {
            let bytes_needed = ((bits_needed - 1) / 8) + 1;
            let buf = self.read_bytes(bytes_needed as usize)?;
            for b in buf {
                res = res << 8 | (*b as u64);
            }
            let mut inner = self.state.borrow_mut();
            let new_bits = res;
            res >>= inner.bits_left;
            if bits_needed < 64 {
                res |= inner.bits << bits_needed;
            }
            inner.bits = new_bits;
        } else {
            res = self.state.borrow().bits >> -bits_needed;
        }

        let mut inner = self.state.borrow_mut();
        let mut mask = (1u64 << inner.bits_left) - 1;
        inner.bits &= mask;

        Ok(res)
    }

    fn read_bits_int_le(&self, n: usize) -> KResult<u64> {
        let mut res : u64 = 0;

        if n > 64 {
            return Err(KError::ReadBitsTooLarge { requested: n });
        }

        let n = n as i64;
        let bits_needed = n - self.state.borrow().bits_left;

        if bits_needed > 0 {
            let bytes_needed = ((bits_needed - 1) / 8) + 1;
            let buf = self.read_bytes(bytes_needed as usize)?;
            for i in 0..bytes_needed {
                res |= (buf[i as usize] as u64) << (i * 8);
            }
            let mut inner = self.state.borrow_mut();
            let new_bits;
            if bits_needed < 64 {
                new_bits = res >> bits_needed;
            } else {
                new_bits = 0;
            }
            res = res << inner.bits_left | inner.bits;
            inner.bits = new_bits;
        } else {
            let mut inner = self.state.borrow_mut();
            res = inner.bits;
            inner.bits >>= n;

        }

        let mut inner = self.state.borrow_mut();
        inner.bits_left = -bits_needed & 7;

        if n < 64 {
            let mut mask = (1u64 << n) - 1;
            res &= mask;
        }

        Ok(res)
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

    let enc = label.to_lowercase();
    if enc == "cp437"
    {
        use std::io::BufReader;
        let reader = BufReader::new(bytes);
        let mut buffer = reader.bytes();
        let mut r = cp437::Reader::new(&mut buffer);
        return Ok(r.consume(bytes.len()));
    }

    Err(KError::Encoding{ desc: format!("decode_string: unknown WHATWG Encoding standard: {}", label)})
}

pub fn reverse_string<S: AsRef<str>>(s: S) -> KResult<String> {
    Ok(s.as_ref().to_string().graphemes(true).rev().collect())
}

pub fn modulo(a: i64, b: i64) -> i64 {
    let mut r = a % b;
    if r < 0 {
        r += b;
    }
    r
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
    use std::{borrow::BorrowMut, clone};

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

        assert_eq!(reader.read_bits_int_be(1).unwrap(), 1);
    }

    #[test]
    fn read_bits_multiple() {
        // 0xA0
        let b = vec![0b10100000];
        let reader = BytesReader::new(&b[..]);

        assert_eq!(reader.read_bits_int_be(1).unwrap(), 1);
        assert_eq!(reader.read_bits_int_be(1).unwrap(), 0);
        assert_eq!(reader.read_bits_int_be(1).unwrap(), 1);
    }

    #[test]
    fn read_bits_large() {
        let b = vec![0b10100000];
        let reader = BytesReader::new(&b[..]);

        assert_eq!(reader.read_bits_int_be(3).unwrap(), 5);
    }

    #[test]
    fn read_bits_span() {
        let b = vec![0x01, 0x80];
        let reader = BytesReader::new(&b[..]);

        assert_eq!(reader.read_bits_int_be(9).unwrap(), 3);
    }

    #[test]
    fn read_bits_too_large() {
        let b: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let reader = BytesReader::new(&b[..]);

        assert_eq!(
            reader.read_bits_int_be(65).unwrap_err(),
            KError::ReadBitsTooLarge { requested: 65 }
        )
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

    #[derive(Default, Debug, Clone)]
    struct RootStruct {
        child: SharedType<ChildStruct>,
    }

    // impl PartialEq for SharedType<ChildStruct> {
    //     fn eq(&self, other: &Self) -> bool {
    //         unimplemented!()
    //     }
    // }

    impl<'r, 's: 'r> KStruct<'r, 's> for RootStruct {
        type Root = Self;
        type Parent = Self;

        fn read<S: KStream>(
                &self,
                io: &'s S,
                root: SharedType<Self::Root>,
                parent: SharedType<Self::Parent>,
            ) -> KResult<()> {
                let x = ChildStruct::read_into(io, Some(root), Some(parent))?;
                self.child.set(Rc::new(x));
                Ok(())
        }
    }

    #[test]
    fn empty_parent() {
        let b = [];
        let reader = BytesReader::new(&b[..]);
        let root_struct: RootStruct = RootStruct::read_into(&reader, None, None).unwrap();
    }

    #[derive(Default, Debug, Clone, PartialEq)]
    struct ChildStruct {
        parent: RefCell<SharedType<RootStruct>>,
    }

    impl PartialEq for SharedType<RootStruct> {
        fn eq(&self, other: &Self) -> bool {
            unimplemented!()
        }
    }

    impl<'r, 's: 'r> KStruct<'r, 's> for ChildStruct {
        type Root = RootStruct;
        type Parent = RootStruct;

        fn read<S: KStream>(
                &self,
                _io: &'s S,
                _root: SharedType<Self::Root>,
                _parent: SharedType<Self::Parent>,
            ) -> KResult<()> {
                *self.parent.borrow_mut() = _parent;

                Ok(())
        }
    }
/*
    #[test]
    fn root_is_parent() {
        let b = [];
        let reader = BytesReader::new(&b[..]);
        let root_struct = Rc::<RootStruct>::new(RootStruct::read_into(&reader, None, None).unwrap());
        let ors = Some(root_struct.clone());
        let child_struct: ChildStruct = ChildStruct::read_into(&reader, ors, Some(&*root_struct.clone())).unwrap();

        dbg!(&child_struct);
        assert_eq!(*child_struct.parent.borrow(), *root_struct);
        assert_eq!(**child_struct.parent.borrow().child.borrow().as_ref().unwrap(), child_struct);
    }

    #[derive(Default, Debug, Clone)]
    struct GrandChildStruct {
        parent: RefCell<ChildStruct>,
    }

    impl<'r, 's: 'r> KStruct<'r, 's> for GrandChildStruct {
        type Root = RootStruct;
        type Parent = ChildStruct;

        fn read<S: KStream>(
                &self,
                _io: &'s S,
                _root: Option<SharedType<Self::Root>>,
                _parent: Option<SharedType<Self::Parent>>,
            ) -> KResult<()> {
                if let Some(parent) = _parent {
                    *self.parent.borrow_mut() = parent.clone();
                }

                Ok(())
        }
    }

    #[test]
    fn child_is_parent() {
        let b = [];
        let reader = BytesReader::new(&b[..]);
        let root_struct = Rc::<RootStruct>::new(RootStruct::read_into(&reader, None, None).unwrap());
        let child_struct: ChildStruct = ChildStruct::read_into(&reader,Some(root_struct.clone ()), Some(&*root_struct.clone())).unwrap();
        let grand_child_struct: GrandChildStruct = GrandChildStruct::read_into(&reader, Some(root_struct.clone()), Some(&child_struct.clone())).unwrap();

        assert_eq!(*child_struct.parent.borrow(), *root_struct);
    }
*/
}
