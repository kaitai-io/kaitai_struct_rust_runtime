#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use kaitai::*;

mod helpers_tests;
use helpers_tests::*;

mod term_u1_val;
use term_u1_val::*;

#[test]
fn basic_parse() {
    let bytes = get_file_as_byte_vec("formats/bin/str_encodings.bin");
    let mut reader = BytesReader::new(&bytes);
    let mut test = TermU1Val::default();
    {
        let res = test.read(&mut reader, None, KStructUnit::parent_stack());
        println!("{:?}", res);
        assert!(res.is_ok());
    }

    let b1 : Vec<u8> = vec![0x0A, 0x00, 0x53, 0x6F, 0x6D, 0x65, 0x20, 0x41, 0x53, 0x43, 0x49, 0x49, 0x0F, 0x00];
    assert_eq!(b1, test.foo);
    assert_eq!("\u{3053}\u{3093}\u{306b}", test.bar);
}