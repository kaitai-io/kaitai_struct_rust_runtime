use std::io::{Read, Result, Seek, SeekFrom};

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
}

impl<T: Read + Seek> KaitaiStream for T {}
