mod kaitai_stream;
mod kaitai_struct;

pub use crate::kaitai_stream::KaitaiStream;
pub use crate::kaitai_struct::KaitaiStruct;

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

    #[test]
    fn ensure_fixed_contents() {
        let mut buf = Cursor::new([1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(
            buf.ensure_fixed_contents(4, vec![1, 2, 3, 4]).unwrap(),
            [1, 2, 3, 4]
        );
    }

    #[test]
    #[should_panic]
    fn ensure_fixed_contents_panic() {
        let mut buf = Cursor::new([1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(
            buf.ensure_fixed_contents(4, vec![5, 6, 7, 8]).unwrap(),
            [1, 2, 3, 4]
        );
    }

    #[test]
    fn read_str_byte_limit() {
        let mut buf = Cursor::new([
            230, 151, 165, 230, 156, 172, 232, 170, 158, // utf-8
            147, 250, 150, 123, 140, 234, // shift_jis
        ]);
        assert_eq!(buf.read_str_byte_limit(9, "utf-8").unwrap(), "日本語");
        assert_eq!(buf.read_str_byte_limit(6, "shift_jis").unwrap(), "日本語");
    }

    #[test]
    fn read_str_eos() {
        let mut buf = Cursor::new([49, 50, 51]);
        assert_eq!(buf.read_str_eos("ascii").unwrap(), "123");
        assert_eq!(buf.pos().unwrap(), 3);
    }

    #[test]
    fn read_strz() {
        let mut buf = Cursor::new([
            230, 151, 165, 230, 156, 172, 232, 170, 158, 0, // utf-8
            147, 250, 150, 123, 140, 234, 0, // shift_jis
        ]);
        assert_eq!(
            buf.read_strz("utf-8", 0, false, true, false).unwrap(),
            "日本語"
        );
        assert_eq!(
            buf.read_strz("shift_jis", 0, false, true, false).unwrap(),
            "日本語"
        );
    }

    #[test]
    #[should_panic]
    fn read_strz_panic() {
        let mut buf = Cursor::new([49, 50, 51]); // no terminator
        assert_eq!(buf.read_strz("utf-8", 0, false, true, true).unwrap(), "123");
    }

    #[test]
    fn process_xor_one() {
        let mut buf = Cursor::new([]);
        assert_eq!(buf.process_xor_one(vec![0, 0, 0, 0], 1), [1, 1, 1, 1]);
    }

    #[test]
    fn process_xor_one_many() {
        let mut buf = Cursor::new([]);
        assert_eq!(
            buf.process_xor_many(vec![0, 0, 0, 0], vec![1, 2, 3, 4]),
            [1, 2, 3, 4]
        );
    }

    #[test]
    fn process_rotate_left() {
        let mut buf = Cursor::new([]);
        assert_eq!(
            buf.process_rotate_left(vec![0b1111_0000, 0b0110_0110], 2, 1),
            [0b1100_0011, 0b1001_1001]
        );
        assert_eq!(
            buf.process_rotate_left(vec![0b1111_0000, 0b0110_0110], -6, 1),
            [0b1100_0011, 0b1001_1001]
        );
    }

    #[test]
    fn process_zlib() {
        let mut buf = Cursor::new([]);
        let arr = vec![
            120, 156, 75, 84, 40, 44, 205, 76, 206, 86, 72, 42, 202, 47, 207, 83, 72, 203, 175, 80,
            200, 42, 205, 45, 40, 86, 200, 47, 75, 45, 2, 0, 148, 189, 10, 127,
        ];
        let deflate = buf.process_zlib(arr).unwrap();
        assert_eq!(deflate, "a quick brown fox jumps over".as_bytes())
    }
}
