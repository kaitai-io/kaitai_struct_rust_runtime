//! # Kaitai Struct: runtime library for Rust
//!
//! This library implements Kaitai Struct API for Rust.
//!
//! [Kaitai Struct] is a declarative language used for describe various binary
//! data structures, laid out in files or in memory: i.e. binary file
//! formats, network stream packet formats, etc.
//!
//! [Kaitai Struct]: https://github.com/kaitai-io/kaitai_struct/
#![deny(missing_docs)]
#![deny(unsafe_code)]

mod kaitai_stream;
mod kaitai_struct;
mod processors;

pub use crate::kaitai_stream::KaitaiStream;
pub use crate::kaitai_struct::KaitaiStruct;
pub use crate::processors::*;

#[cfg(test)]
mod tests {
    use crate::KaitaiStream;
    use pretty_assertions::assert_eq;
    use std::io::Cursor;

    #[test]
    fn test_seek() {
        let mut buf = Cursor::new([0, 0, 0, 0, 64, 226, 1, 0]);
        let _ = buf.seek(4);
        assert_eq!(buf.read_s4le().unwrap(), 123456);
    }

    #[test]
    fn test_pos() {
        let mut buf = Cursor::new([0, 0, 0, 0, 64, 226, 1, 0]);
        assert_eq!(buf.pos().unwrap(), 0);
        let _ = buf.seek(4);
        assert_eq!(buf.pos().unwrap(), 4);
    }

    #[test]
    fn test_multiple_reads() {
        let mut buf = Cursor::new([1, 2, 3, 4, 5, 6, 7, 8]);
        for x in 0..8 {
            assert_eq!(buf.pos().unwrap(), x as u64);
            assert_eq!(buf.read_s1().unwrap(), (x + 1) as i8);
        }
    }

    #[test]
    fn test_size() {
        let mut buf = Cursor::new([0, 0, 0, 0, 64, 226, 1, 0]);
        assert_eq!(buf.size().unwrap(), 8);
    }

    #[test]
    fn test_is_eof() {
        let mut buf = Cursor::new([0, 0, 0, 0]);
        assert_eq!(buf.is_eof().unwrap(), false);
        let _ = buf.read_s2le();
        assert_eq!(buf.is_eof().unwrap(), false);
        let _ = buf.read_s2le();
        assert_eq!(buf.is_eof().unwrap(), true);
    }

    macro_rules! test_read_integer {
        ($name:ident, $value:expr) => {
            #[test]
            fn $name() {
                let mut buf = Cursor::new([1, 2, 3, 4, 5, 6, 7, 8]);
                assert_eq!(buf.$name().unwrap(), $value);
            }
        };
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
        let mut buf = Cursor::new([0, 0, 128, 62]);
        assert_eq!(buf.read_f4le().unwrap(), 0.25);
    }

    #[test]
    fn read_f4be() {
        let mut buf = Cursor::new([62, 128, 0, 0]);
        assert_eq!(buf.read_f4be().unwrap(), 0.25);
    }

    #[test]
    fn read_f8le() {
        let mut buf = Cursor::new([0, 0, 0, 0, 0, 0, 208, 63]);
        assert_eq!(buf.read_f8le().unwrap(), 0.25);
    }

    #[test]
    fn read_f8be() {
        let mut buf = Cursor::new([63, 208, 0, 0, 0, 0, 0, 0]);
        assert_eq!(buf.read_f8be().unwrap(), 0.25);
    }

    #[test]
    fn read_bytes() {
        let mut buf = Cursor::new([1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(buf.read_bytes(4).unwrap(), [1, 2, 3, 4]);
    }

    #[test]
    fn read_bytes_full() {
        let mut buf = Cursor::new([1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(buf.read_bytes_full().unwrap(), [1, 2, 3, 4, 5, 6, 7, 8]);
    }

    mod read_bytes_term {
        use super::*;

        mod without_error {
            use super::*;
            use pretty_assertions::assert_eq;

            #[test]
            fn with_terminator() {
                let mut buf = Cursor::new([
                    1, 2, 3, 4, 0, // first chunk
                    5, 6, 0, // second chunk
                ]);
                // Read to 0, but not consume it and not include in the result
                assert_eq!(
                    buf.read_bytes_term(0, false, false, false).unwrap(),
                    [1, 2, 3, 4]
                );
                // Read to 0 and consume it, but do not include in the result.
                // Because we already at 0, an empty array is returned
                assert_eq!(buf.read_bytes_term(0, false, true, false).unwrap(), []);
                // Read to second 0, include it to the result, but not consume
                assert_eq!(
                    buf.read_bytes_term(0, true, false, false).unwrap(),
                    [5, 6, 0]
                );
                // Read to second 0 and consume it, and include in the result.
                // Because we already at second 0, only it is returned
                assert_eq!(buf.read_bytes_term(0, true, true, false).unwrap(), [0]);
            }

            #[test]
            fn without_terminator() {
                let mut buf = Cursor::new([1, 2, 3, 4]);
                // Read to missing 0, do not try consume it or include it in the result
                assert_eq!(
                    buf.read_bytes_term(0, false, false, false).unwrap(),
                    [1, 2, 3, 4]
                );
                // Read to missing 0, try to consume it but do not try to include it in the result
                // Because we already at the end, an empty array is returned
                assert_eq!(buf.read_bytes_term(0, false, true, false).unwrap(), []);

                let mut buf = Cursor::new([5, 6]);
                // Read to missing 0, do not try to consume, but try to include it in the result
                assert_eq!(buf.read_bytes_term(0, true, false, false).unwrap(), [5, 6]);
                // Read to missing 0, try to consume and include it in the result
                // Because we already at the end, an empty array is returned
                assert_eq!(buf.read_bytes_term(0, true, true, false).unwrap(), []);
            }
        }

        mod with_eos_error {
            use super::*;
            use pretty_assertions::assert_eq;

            #[test]
            fn with_terminator() {
                let mut buf = Cursor::new([
                    1, 2, 3, 4, 0, // first chunk
                    5, 6, 0, // second chunk
                ]);
                // Read to 0, but not consume it and not include in the result
                assert_eq!(
                    buf.read_bytes_term(0, false, false, true).unwrap(),
                    [1, 2, 3, 4]
                );
                // Read to 0 and consume it, but do not include in the result.
                // Because we already at 0, an empty array is returned
                assert_eq!(buf.read_bytes_term(0, false, true, true).unwrap(), []);
                // Read to second 0, include it to the result, but not consume
                assert_eq!(
                    buf.read_bytes_term(0, true, false, true).unwrap(),
                    [5, 6, 0]
                );
                // Read to second 0 and consume it, and include in the result.
                // Because we already at second 0, only it is returned
                assert_eq!(buf.read_bytes_term(0, true, true, true).unwrap(), [0]);
            }

            #[test]
            fn without_terminator() {
                // Read to missing 0 lead to error
                assert!(matches!(
                    Cursor::new([1, 2]).read_bytes_term(0, false, false, true),
                    Err(_)
                ));
                assert!(matches!(
                    Cursor::new([3, 4]).read_bytes_term(0, false, true, true),
                    Err(_)
                ));
                assert!(matches!(
                    Cursor::new([5, 6]).read_bytes_term(0, true, false, true),
                    Err(_)
                ));
                assert!(matches!(
                    Cursor::new([7, 8]).read_bytes_term(0, true, true, true),
                    Err(_)
                ));
            }
        }
    }

    #[test]
    fn process_xor_one() {
        assert_eq!(crate::process_xor_one(&[0, 0, 0, 0], 1), [1, 1, 1, 1]);
    }

    #[test]
    fn process_xor_many() {
        assert_eq!(
            crate::process_xor_many(&[0, 0, 0, 0], &[1, 2, 3, 4]),
            [1, 2, 3, 4]
        );
    }

    #[test]
    fn process_rotate_left() {
        assert_eq!(
            crate::process_rotate_left(&[0b1111_0000, 0b0110_0110], 2, 1),
            [0b1100_0011, 0b1001_1001]
        );
        assert_eq!(
            crate::process_rotate_left(&[0b1111_0000, 0b0110_0110], -6, 1),
            [0b1100_0011, 0b1001_1001]
        );
    }

    #[test]
    fn process_zlib() {
        let arr = [
            120, 156, 75, 84, 40, 44, 205, 76, 206, 86, 72, 42, 202, 47, 207, 83, 72, 203, 175, 80,
            200, 42, 205, 45, 40, 86, 200, 47, 75, 45, 2, 0, 148, 189, 10, 127,
        ];
        let deflate = crate::process_zlib(&arr).unwrap();
        assert_eq!(deflate, "a quick brown fox jumps over".as_bytes())
    }
}
