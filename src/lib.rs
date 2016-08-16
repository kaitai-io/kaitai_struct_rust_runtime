
extern crate byteorder;
extern crate encoding;
extern crate flate2;

use std::io::{Cursor, Seek, SeekFrom, Read, Result};
use flate2::read::ZlibDecoder;

use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use encoding::DecoderTrap;
use encoding::label::encoding_from_whatwg_label;

pub trait Stream: ReadBytesExt + Seek {
    // ------------------ //
    // Stream positioning //
    // ------------------ //

    fn is_eof(&mut self) -> Result<bool> {
        // TODO: I'm positive there's a better way to do this!
        // See also the `size` implementation
        let pos = try!(self.pos());
        let size = try!(Seek::seek(self, SeekFrom::End(0)));
        try!(Seek::seek(self, SeekFrom::Start(pos)));
        return Ok(pos >= size);
    }

    fn seek(&mut self, position: u64) -> Result<()> {
        match Seek::seek(self, SeekFrom::Start(position)) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    fn pos(&mut self) -> Result<u64> {
        match Seek::seek(self, SeekFrom::Current(0)) {
            Ok(i) => Ok(i as u64),
            Err(e) => Err(e),
        }
    }

    fn size(&mut self) -> Result<u64> {
        // TODO: I'm positive there's a better way to do this!
        let pos = try!(self.pos());
        let size = try!(Seek::seek(self, SeekFrom::End(0)));
        try!(Seek::seek(self, SeekFrom::Start(pos)));
        return Ok(size);
    }

    // ---------------------- //
    // Integer types - signed //
    // ---------------------- //

    fn read_s1(&mut self) -> Result<i8> {
        self.read_i8()
    }

    // Big endian

    fn read_s2be(&mut self) -> Result<i16> {
        self.read_i16::<BigEndian>()
    }

    fn read_s4be(&mut self) -> Result<i32> {
        self.read_i32::<BigEndian>()
    }

    fn read_s8be(&mut self) -> Result<i64> {
        self.read_i64::<BigEndian>()
    }

    // Little endian

    fn read_s2le(&mut self) -> Result<i16> {
        self.read_i16::<LittleEndian>()
    }

    fn read_s4le(&mut self) -> Result<i32> {
        self.read_i32::<LittleEndian>()
    }

    fn read_s8le(&mut self) -> Result<i64> {
        self.read_i64::<LittleEndian>()
    }

    // ------------------------ //
    // Integer types - unsigned //
    // ------------------------ //

    fn read_u1(&mut self) -> Result<u8> {
        self.read_u8()
    }

    // Big endian

    fn read_u2be(&mut self) -> Result<u16> {
        self.read_u16::<BigEndian>()
    }

    fn read_u4be(&mut self) -> Result<u32> {
        self.read_u32::<BigEndian>()
    }

    fn read_u8be(&mut self) -> Result<u64> {
        self.read_u64::<BigEndian>()
    }

    // Little endian

    fn read_u2le(&mut self) -> Result<u16> {
        self.read_u16::<LittleEndian>()
    }

    fn read_u4le(&mut self) -> Result<u32> {
        self.read_u32::<LittleEndian>()
    }

    fn read_u8le(&mut self) -> Result<u64> {
        self.read_u64::<LittleEndian>()
    }

    // -------------------- //
    // Floating point types //
    // -------------------- //

    // Big endian

    fn read_f4be(&mut self) -> Result<f32> {
        self.read_f32::<BigEndian>()
    }

    fn read_f8be(&mut self) -> Result<f64> {
        self.read_f64::<BigEndian>()
    }

    // Little endian

    fn read_f4le(&mut self) -> Result<f32> {
        self.read_f32::<LittleEndian>()
    }

    fn read_f8le(&mut self) -> Result<f64> {
        self.read_f64::<LittleEndian>()
    }

    // ----------- //
    // Byte arrays //
    // ----------- //

    fn read_bytes(&mut self, count: usize) -> Result<Vec<u8>> {
        let mut buffer = vec![0; count];
        match self.read_exact(&mut buffer[..]) {
            Ok(_) => Ok(buffer),
            Err(e) => Err(e),
        }
    }

    fn read_bytes_full(&mut self) -> Result<Vec<u8>> {
        let mut buffer = vec![0; 0];
        match self.read_to_end(&mut buffer) {
            Ok(_) => Ok(buffer),
            Err(e) => Err(e),
        }
    }

    fn ensure_fixed_contents(&mut self, count: usize, expected: Vec<u8>) -> Result<Vec<u8>> {
        let mut buffer = vec![0; count];
        match self.read_exact(&mut buffer[..]) {
            Ok(_) => {
                assert_eq!(buffer, expected);
                Ok(buffer)
            }
            Err(e) => Err(e),
        }
    }

    // ------- //
    // Strings //
    // ------- //

    fn read_str_eos(&mut self, encoding: &str) -> Result<String> {
        match encoding_from_whatwg_label(encoding) {
            Some(enc) => {
                let buffer = try!(self.read_bytes_full());
                match enc.decode(&buffer, DecoderTrap::Strict) {
                    Ok(s) => Ok(s),
                    Err(e) => panic!("Error decoding string: {}", e),
                }
            }
            None => panic!("Unknown encoding: {}", encoding),
        }
    }

    fn read_str_byte_limit(&mut self, length: usize, encoding: &str) -> Result<String> {
        match encoding_from_whatwg_label(encoding) {
            Some(enc) => {
                let buffer = try!(self.read_bytes(length));
                match enc.decode(&buffer, DecoderTrap::Strict) {
                    Ok(s) => Ok(s),
                    Err(e) => panic!("Error decoding string: {}", e),
                }
            }
            None => panic!("Unknown encoding: {}", encoding),
        }
    }

    fn read_strz(&mut self,
                 encoding: &str,
                 terminator: u8,
                 include_terminator: bool,
                 consume_terminator: bool,
                 eos_error: bool)
                 -> Result<String> {
        let enc = match encoding_from_whatwg_label(encoding) {
            Some(enc) => enc,
            None => return panic!("Unknown encoding: {}", encoding),
        };
        let mut buffer = vec![];
        let mut c = vec![0; 1];
        loop {
            match self.read_exact(&mut c[..]) {
                Ok(_) => {}
                Err(e) => {
                    if eos_error {
                        return Err(e);
                    }
                    break;
                }
            };
            if c[0] == terminator {
                if include_terminator {
                    buffer.push(c[0])
                }
                if !consume_terminator {
                    let pos = try!(self.pos());
                    try!(Seek::seek(self, SeekFrom::Start((pos - 1) as u64)));
                }
                break;
            }
            buffer.push(c[0])
        }
        match enc.decode(&buffer, DecoderTrap::Strict) {
            Ok(s) => Ok(s),
            Err(e) => panic!("Error decoding string: {}", e),
        }
    }

    // --------------------- //
    // Byte array processing //
    // --------------------- //

    fn process_xor_one(&mut self, value: Vec<u8>, key: u8) -> Vec<u8> {
        let mut result = vec![0; value.len()];
        for i in 0..value.len() {
            result[i] = (value[i] ^ key) as u8;
        }
        return result;
    }

    fn process_xor_many(&mut self, value: Vec<u8>, key: Vec<u8>) -> Vec<u8> {
        let mut result = vec![0; value.len()];
        let mut j = 0;
        for i in 0..value.len() {
            result[i] = (value[i] ^ key[j]) as u8;
            j = (j + 1) % key.len();
        }
        return result;
    }

    fn process_rotate_left(&mut self, data: Vec<u8>, amount: i32, group_size: i32) -> Vec<u8> {
        if amount < -7 || amount > 7 {
            panic!("Rotation of more than 7 cannot be performed.");
        }

        let mut rot_amount = amount;
        if rot_amount < 0 {
            rot_amount += 8;
        }

        let mut result = vec![0; data.len()];
        match group_size {
            1 => {
                for i in 0..data.len() {
                    result[i] = data[i].rotate_left(rot_amount as u32);
                }
            }
            _ => return panic!("Unable to rotate a group of {} bytes yet", group_size),
        }
        return result;
    }

    fn process_zlib(&mut self, data: Vec<u8>) -> Result<Vec<u8>> {
        let mut decoder = ZlibDecoder::new(Cursor::new(data));
        let mut result = Vec::new();
        match decoder.read_to_end(&mut result) {
            Ok(_) => Ok(result),
            Err(e) => Err(e),
        }
    }
}

impl<T: Read + Seek> Stream for T {}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use Stream;

    #[test]
    fn test_seek() {
        let mut buf = Cursor::new(vec![0, 0, 0, 0, 64, 226, 1, 0]);
        let _ = buf.seek(4);
        assert_eq!(buf.read_s4le().unwrap(), 123456);
    }

    #[test]
    fn test_pos() {
        let mut buf = Cursor::new(vec![0, 0, 0, 0, 64, 226, 1, 0]);
        assert_eq!(buf.pos().unwrap(), 0);
        let _ = buf.seek(4);
        assert_eq!(buf.pos().unwrap(), 4);
    }

    #[test]
    fn test_multiple_reads() {
        let mut buf = Cursor::new(vec![1, 2, 3, 4, 5, 6, 7, 8]);
        for x in 0..8 {
            assert_eq!(buf.pos().unwrap(), x as u64);
            assert_eq!(buf.read_s1().unwrap(), (x + 1) as i8);
        }
    }

    #[test]
    fn test_size() {
        let mut buf = Cursor::new(vec![0, 0, 0, 0, 64, 226, 1, 0]);
        assert_eq!(buf.size().unwrap(), 8);
    }

    #[test]
    fn test_is_eof() {
        let mut buf = Cursor::new(vec![0, 0, 0, 0]);
        assert_eq!(buf.is_eof().unwrap(), false);
        let _ = buf.read_s2le();
        assert_eq!(buf.is_eof().unwrap(), false);
        let _ = buf.read_s2le();
        assert_eq!(buf.is_eof().unwrap(), true);
    }

    macro_rules! test_read_integer {
        ($name:ident, $value:expr) => (
            #[test]
            fn $name() {
                let mut buf = Cursor::new(vec![1, 2, 3, 4, 5, 6, 7, 8]);
                assert_eq!(buf.$name().unwrap(), $value);
            }
        );
    }

    test_read_integer!(read_u1, 1);
    test_read_integer!(read_s1, 1);

    test_read_integer!(read_s2le, 513);
    test_read_integer!(read_s2be, 258);
    test_read_integer!(read_u2le, 513);
    test_read_integer!(read_u2be, 258);

    test_read_integer!(read_s4le, 67305985);
    test_read_integer!(read_s4be, 16909060);
    test_read_integer!(read_u4le, 67305985);
    test_read_integer!(read_u4be, 16909060);

    test_read_integer!(read_s8le, 578437695752307201);
    test_read_integer!(read_s8be, 72623859790382856);
    test_read_integer!(read_u8le, 578437695752307201);
    test_read_integer!(read_u8be, 72623859790382856);

    #[test]
    fn read_f4le() {
        let mut buf = Cursor::new(vec![0, 0, 128, 62]);
        assert_eq!(buf.read_f4le().unwrap(), 0.25);
    }

    #[test]
    fn read_f4be() {
        let mut buf = Cursor::new(vec![62, 128, 0, 0]);
        assert_eq!(buf.read_f4be().unwrap(), 0.25);
    }

    #[test]
    fn read_f8le() {
        let mut buf = Cursor::new(vec![0, 0, 0, 0, 0, 0, 208, 63]);
        assert_eq!(buf.read_f8le().unwrap(), 0.25);
    }

    #[test]
    fn read_f8be() {
        let mut buf = Cursor::new(vec![63, 208, 0, 0, 0, 0, 0, 0]);
        assert_eq!(buf.read_f8be().unwrap(), 0.25);
    }

    #[test]
    fn read_bytes() {
        let mut buf = Cursor::new(vec![1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(buf.read_bytes(4).unwrap(), vec![1, 2, 3, 4]);
    }

    #[test]
    fn read_bytes_full() {
        let mut buf = Cursor::new(vec![1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(buf.read_bytes_full().unwrap(), vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn ensure_fixed_contents() {
        let mut buf = Cursor::new(vec![1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(buf.ensure_fixed_contents(4, vec![1, 2, 3, 4]).unwrap(),
                   vec![1, 2, 3, 4]);
    }

    #[test]
    #[should_panic]
    fn ensure_fixed_contents_panic() {
        let mut buf = Cursor::new(vec![1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(buf.ensure_fixed_contents(4, vec![5, 6, 7, 8]).unwrap(),
                   vec![1, 2, 3, 4]);
    }

    #[test]
    fn read_str_byte_limit() {
        let mut buf = Cursor::new(vec![
            230, 151, 165, 230, 156, 172, 232, 170, 158, // utf-8
            147, 250, 150, 123, 140, 234, // shift_jis
        ]);
        assert_eq!(buf.read_str_byte_limit(9, "utf-8").unwrap(), "日本語");
        assert_eq!(buf.read_str_byte_limit(6, "shift_jis").unwrap(),
                   "日本語");
    }

    #[test]
    fn read_str_eos() {
        let mut buf = Cursor::new(vec![49, 50, 51]);
        assert_eq!(buf.read_str_eos("ascii").unwrap(), "123");
        assert_eq!(buf.pos().unwrap(), 3);
    }

    #[test]
    fn read_strz() {
        let mut buf = Cursor::new(vec![
            230, 151, 165, 230, 156, 172, 232, 170, 158, 0, // utf-8
            147, 250, 150, 123, 140, 234, 0, // shift_jis
        ]);
        assert_eq!(buf.read_strz("utf-8", 0, false, true, false).unwrap(),
                   "日本語");
        assert_eq!(buf.read_strz("shift_jis", 0, false, true, false).unwrap(),
                   "日本語");
    }

    #[test]
    #[should_panic]
    fn read_strz_panic() {
        let mut buf = Cursor::new(vec![49, 50, 51]); // no terminator
        assert_eq!(buf.read_strz("utf-8", 0, false, true, true).unwrap(), "123");
    }

    #[test]
    fn process_xor_one() {
        let mut buf = Cursor::new(vec![]);
        assert_eq!(buf.process_xor_one(vec![0, 0, 0, 0], 1), vec![1, 1, 1, 1]);
    }

    #[test]
    fn process_xor_one_many() {
        let mut buf = Cursor::new(vec![]);
        assert_eq!(buf.process_xor_many(vec![0, 0, 0, 0], vec![1, 2, 3, 4]),
                   vec![1, 2, 3, 4]);
    }

    #[test]
    fn process_rotate_left() {
        let mut buf = Cursor::new(vec![]);
        assert_eq!(buf.process_rotate_left(vec![0b1111_0000, 0b0110_0110], 2, 1),
                   vec![0b1100_0011, 0b1001_1001]);
        assert_eq!(buf.process_rotate_left(vec![0b1111_0000, 0b0110_0110], -6, 1),
                   vec![0b1100_0011, 0b1001_1001]);
    }

    #[test]
    fn process_zlib() {
        let mut buf = Cursor::new(vec![]);
        let arr = vec![120, 156, 75, 84, 40, 44, 205, 76, 206, 86, 72, 42, 202, 47, 207, 83, 72,
                       203, 175, 80, 200, 42, 205, 45, 40, 86, 200, 47, 75, 45, 2, 0, 148, 189,
                       10, 127];
        let deflate = buf.process_zlib(arr).unwrap();
        assert_eq!(deflate, "a quick brown fox jumps over".as_bytes())
    }
}
