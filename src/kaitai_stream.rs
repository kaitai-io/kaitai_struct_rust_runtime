use flate2::read::ZlibDecoder;
use std::io::{Cursor, Read, Result, Seek, SeekFrom};

use byteorder::{BigEndian, ByteOrder, LittleEndian};
use encoding::label::encoding_from_whatwg_label;
use encoding::DecoderTrap;

macro_rules! read_endian {
    ($this:ident, $size:expr, $end:ident, $method:ident) => {{
        let mut buf = [0; $size];
        $this.read_exact(&mut buf)?;
        Ok($end::$method(&buf))
    }};
}

pub trait KaitaiStream: Read + Seek {
    // ------------------ //
    // Stream positioning //
    // ------------------ //

    fn is_eof(&mut self) -> Result<bool> {
        // TODO: I'm positive there's a better way to do this!
        // See also the `size` implementation
        let pos = self.pos()?;
        let size = Seek::seek(self, SeekFrom::End(0))?;
        Seek::seek(self, SeekFrom::Start(pos))?;
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
        let pos = self.pos()?;
        let size = Seek::seek(self, SeekFrom::End(0))?;
        Seek::seek(self, SeekFrom::Start(pos))?;
        return Ok(size);
    }

    // ---------------------- //
    // Integer types - signed //
    // ---------------------- //

    fn read_s1(&mut self) -> Result<i8> {
        let mut buf = [0; 1];
        self.read_exact(&mut buf)?;
        Ok(buf[0] as i8)
    }

    // Big endian

    fn read_s2be(&mut self) -> Result<i16> {
        read_endian!(self, 2, BigEndian, read_i16)
    }

    fn read_s4be(&mut self) -> Result<i32> {
        read_endian!(self, 4, BigEndian, read_i32)
    }

    fn read_s8be(&mut self) -> Result<i64> {
        read_endian!(self, 8, BigEndian, read_i64)
    }

    // Little endian

    fn read_s2le(&mut self) -> Result<i16> {
        read_endian!(self, 2, LittleEndian, read_i16)
    }

    fn read_s4le(&mut self) -> Result<i32> {
        read_endian!(self, 4, LittleEndian, read_i32)
    }

    fn read_s8le(&mut self) -> Result<i64> {
        read_endian!(self, 8, LittleEndian, read_i64)
    }

    // ------------------------ //
    // Integer types - unsigned //
    // ------------------------ //

    fn read_u1(&mut self) -> Result<u8> {
        let mut buf = [0; 1];
        self.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    // Big endian

    fn read_u2be(&mut self) -> Result<u16> {
        read_endian!(self, 2, BigEndian, read_u16)
    }

    fn read_u4be(&mut self) -> Result<u32> {
        read_endian!(self, 4, BigEndian, read_u32)
    }

    fn read_u8be(&mut self) -> Result<u64> {
        read_endian!(self, 8, BigEndian, read_u64)
    }

    // Little endian

    fn read_u2le(&mut self) -> Result<u16> {
        read_endian!(self, 2, LittleEndian, read_u16)
    }

    fn read_u4le(&mut self) -> Result<u32> {
        read_endian!(self, 4, LittleEndian, read_u32)
    }

    fn read_u8le(&mut self) -> Result<u64> {
        read_endian!(self, 8, LittleEndian, read_u64)
    }

    // -------------------- //
    // Floating point types //
    // -------------------- //

    // Big endian

    fn read_f4be(&mut self) -> Result<f32> {
        read_endian!(self, 4, BigEndian, read_f32)
    }

    fn read_f8be(&mut self) -> Result<f64> {
        read_endian!(self, 8, BigEndian, read_f64)
    }

    // Little endian

    fn read_f4le(&mut self) -> Result<f32> {
        read_endian!(self, 4, LittleEndian, read_f32)
    }

    fn read_f8le(&mut self) -> Result<f64> {
        read_endian!(self, 8, LittleEndian, read_f64)
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
                let buffer = self.read_bytes_full()?;
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
                let buffer = self.read_bytes(length)?;
                match enc.decode(&buffer, DecoderTrap::Strict) {
                    Ok(s) => Ok(s),
                    Err(e) => panic!("Error decoding string: {}", e),
                }
            }
            None => panic!("Unknown encoding: {}", encoding),
        }
    }

    fn read_strz(
        &mut self,
        encoding: &str,
        terminator: u8,
        include_terminator: bool,
        consume_terminator: bool,
        eos_error: bool,
    ) -> Result<String> {
        let enc = match encoding_from_whatwg_label(encoding) {
            Some(enc) => enc,
            None => panic!("Unknown encoding: {}", encoding),
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
                    let pos = self.pos()?;
                    Seek::seek(self, SeekFrom::Start((pos - 1) as u64))?;
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
            _ => unimplemented!("Unable to rotate a group of {} bytes yet", group_size),
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

impl<T: Read + Seek> KaitaiStream for T {}
