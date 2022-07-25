#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use kaitai::*;

mod helpers_tests;
use helpers_tests::*;

mod process_rotate;
use process_rotate::*;

#[test]
fn basic_parse() {
    let bytes = get_file_as_byte_vec("formats/bin/process_rotate.bin");
    let reader = BytesReader::new(&bytes);
    let mut test = ProcessRotate::default();
    {
        let res = test.read(&reader, None, KStructUnit::parent_stack());
        println!("{:?}", res);
        assert!(res.is_ok());
    }

    let buf1 : Vec<u8> = vec![0x48, 0x65, 0x6C, 0x6C, 0x6F];
    assert_eq!(buf1, *test.buf1());
    let buf2 : Vec<u8> = vec![0x57, 0x6F, 0x72, 0x6C, 0x64];
    assert_eq!(buf2, *test.buf2());
    let buf3 : Vec<u8> = vec![0x54, 0x68, 0x65, 0x72, 0x65];
    assert_eq!(buf3, *test.buf3());
}