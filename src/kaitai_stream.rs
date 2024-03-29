use std::io::{Read, Result, Seek, SeekFrom};

use byteorder::{BigEndian, ByteOrder, LittleEndian};

macro_rules! read_endian {
    ($this:ident, $size:expr, $end:ident, $method:ident) => {{
        let mut buf = [0; $size];
        $this.read_exact(&mut buf)?;
        Ok($end::$method(&buf))
    }};
}

/// `KaitaiStream` trait provides implementation of [Kaitai Stream API] for Rust.
///
/// It provides a wide variety of simple methods to read (parse) binary
/// representations of primitive types, such as integer and floating
/// point numbers, byte arrays and strings, and also provides stream
/// positioning / navigation methods with unified cross-language and
/// cross-toolkit semantics.
///
/// Methods of this trait are called from the generated code to parse bynary stream
/// into generated structures.
///
/// This trait is automatically implemented for all sources that is `Read + Seek`.
///
/// Typically, end users won't access any of these Kaitai Stream methods
/// manually, but would describe a binary structure format using .ksy language
/// and then would use Kaitai Struct compiler to generate source code in
/// desired target language.  That code, in turn, would use this trait
/// and API to do the actual parsing job.
///
/// [Kaitai Stream API]: https://doc.kaitai.io/stream_api.html
pub trait KaitaiStream: Read + Seek {
    // ------------------ //
    // Stream positioning //
    // ------------------ //

    /// Check if stream pointer is at the end of stream.
    fn is_eof(&mut self) -> Result<bool> {
        // TODO: I'm positive there's a better way to do this!
        // See also the `size` implementation
        let pos = self.pos()?;
        let size = Seek::seek(self, SeekFrom::End(0))?;
        Seek::seek(self, SeekFrom::Start(pos))?;
        return Ok(pos >= size);
    }

    /// Set stream pointer to designated position in bytes from the beginning of the stream.
    fn seek(&mut self, position: u64) -> Result<()> {
        match Seek::seek(self, SeekFrom::Start(position)) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    /// Get current position of a stream pointer in number of bytes from the beginning of the stream.
    fn pos(&mut self) -> Result<u64> {
        match Seek::seek(self, SeekFrom::Current(0)) {
            Ok(i) => Ok(i as u64),
            Err(e) => Err(e),
        }
    }

    /// Get total size of the stream in bytes.
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

    /// Reads one signed 1-byte integer, used to parse `s1` Kaitai type.
    fn read_s1(&mut self) -> Result<i8> {
        let mut buf = [0; 1];
        self.read_exact(&mut buf)?;
        Ok(buf[0] as i8)
    }

    // Big endian

    /// Reads one signed 2-byte integer, used to parse `s2` Kaitai type in Big Endian.
    fn read_s2be(&mut self) -> Result<i16> {
        read_endian!(self, 2, BigEndian, read_i16)
    }

    /// Reads one signed 4-byte integer, used to parse `s4` Kaitai type in Big Endian.
    fn read_s4be(&mut self) -> Result<i32> {
        read_endian!(self, 4, BigEndian, read_i32)
    }

    /// Reads one signed 8-byte integer, used to parse `s8` Kaitai type in Big Endian.
    fn read_s8be(&mut self) -> Result<i64> {
        read_endian!(self, 8, BigEndian, read_i64)
    }

    // Little endian

    /// Reads one signed 2-byte integer, used to parse `s2` Kaitai type in Little Endian.
    fn read_s2le(&mut self) -> Result<i16> {
        read_endian!(self, 2, LittleEndian, read_i16)
    }

    /// Reads one signed 4-byte integer, used to parse `s4` Kaitai type in Little Endian.
    fn read_s4le(&mut self) -> Result<i32> {
        read_endian!(self, 4, LittleEndian, read_i32)
    }

    /// Reads one signed 8-byte integer, used to parse `s8` Kaitai type in Little Endian.
    fn read_s8le(&mut self) -> Result<i64> {
        read_endian!(self, 8, LittleEndian, read_i64)
    }

    // ------------------------ //
    // Integer types - unsigned //
    // ------------------------ //

    /// Reads one unsigned 1-byte integer, used to parse `u1` Kaitai type.
    fn read_u1(&mut self) -> Result<u8> {
        let mut buf = [0; 1];
        self.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    // Big endian

    /// Reads one unsigned 2-byte integer, used to parse `u2` Kaitai type in Big Endian.
    fn read_u2be(&mut self) -> Result<u16> {
        read_endian!(self, 2, BigEndian, read_u16)
    }

    /// Reads one unsigned 4-byte integer, used to parse `u4` Kaitai type in Big Endian.
    fn read_u4be(&mut self) -> Result<u32> {
        read_endian!(self, 4, BigEndian, read_u32)
    }

    /// Reads one unsigned 8-byte integer, used to parse `u8` Kaitai type in Big Endian.
    fn read_u8be(&mut self) -> Result<u64> {
        read_endian!(self, 8, BigEndian, read_u64)
    }

    // Little endian

    /// Reads one unsigned 2-byte integer, used to parse `u2` Kaitai type in Little Endian.
    fn read_u2le(&mut self) -> Result<u16> {
        read_endian!(self, 2, LittleEndian, read_u16)
    }

    /// Reads one unsigned 4-byte integer, used to parse `u4` Kaitai type in Little Endian.
    fn read_u4le(&mut self) -> Result<u32> {
        read_endian!(self, 4, LittleEndian, read_u32)
    }

    /// Reads one unsigned 8-byte integer, used to parse `u8` Kaitai type in Little Endian.
    fn read_u8le(&mut self) -> Result<u64> {
        read_endian!(self, 8, LittleEndian, read_u64)
    }

    // -------------------- //
    // Floating point types //
    // -------------------- //

    // Big endian

    /// Reads one floating point number with single precision, used to parse `f4` Kaitai type in Big Endian.
    fn read_f4be(&mut self) -> Result<f32> {
        read_endian!(self, 4, BigEndian, read_f32)
    }

    /// Reads one floating point number with double precision, used to parse `f8` Kaitai type in Big Endian.
    fn read_f8be(&mut self) -> Result<f64> {
        read_endian!(self, 8, BigEndian, read_f64)
    }

    // Little endian

    /// Reads one floating point number with single precision, used to parse `f4` Kaitai type in Little Endian.
    fn read_f4le(&mut self) -> Result<f32> {
        read_endian!(self, 4, LittleEndian, read_f32)
    }

    /// Reads one floating point number with double precision, used to parse `f8` Kaitai type in Little Endian.
    fn read_f8le(&mut self) -> Result<f64> {
        read_endian!(self, 8, LittleEndian, read_f64)
    }

    // ----------- //
    // Byte arrays //
    // ----------- //

    /// Reads designated number of bytes from the stream and returns them in a newly allocated buffer.
    fn read_bytes(&mut self, count: usize) -> Result<Vec<u8>> {
        let mut buffer = vec![0; count];
        match self.read_exact(&mut buffer[..]) {
            Ok(_) => Ok(buffer),
            Err(e) => Err(e),
        }
    }

    /// Reads all the remaining bytes in a stream and returns them in a newly allocated buffer.
    fn read_bytes_full(&mut self) -> Result<Vec<u8>> {
        let mut buffer = vec![0; 0];
        match self.read_to_end(&mut buffer) {
            Ok(_) => Ok(buffer),
            Err(e) => Err(e),
        }
    }

    /// Reads bytes until the `terminator` byte is reached.
    ///
    /// # Parameters
    /// - `terminator`: the byte that terminates search
    /// - `include_terminator`: `true` to include the terminator in the returned array.
    ///   If `eos_error` is `false` and no terminator found, does nothing
    /// - `consume_terminator`: `true` to consume the terminator byte before returning
    /// - `eos_error`: `true` to return an error when the EOS was reached before the terminator,
    ///   otherwise EOF is treated as a terminator
    fn read_bytes_term(
        &mut self,
        terminator: u8,
        include_terminator: bool,
        consume_terminator: bool,
        eos_error: bool,
    ) -> Result<Vec<u8>> {
        let mut buffer = vec![];
        let mut c = [0; 1];
        loop {
            // TODO: Very non-optimal, optimize!
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
        Ok(buffer)
    }
}

impl<T: Read + Seek> KaitaiStream for T {}
