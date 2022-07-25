#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use kaitai::*;

mod helpers_tests;
use helpers_tests::*;

mod term_bytes;
use term_bytes::*;

#[test]
fn basic_parse() {
    let bytes = get_file_as_byte_vec("formats/bin/term_strz.bin");
    let reader = BytesReader::new(&bytes);
    let mut test = TermBytes::default();
    {
        let res = test.read(&reader, None, KStructUnit::parent_stack());
        println!("{:?}", res);
        assert!(res.is_ok());
    }

    let s1 : Vec<u8> = vec![0x66, 0x6F, 0x6F];
    assert_eq!(s1, test.s1);

    let s2 : Vec<u8> = vec![0x62, 0x61, 0x72];
    assert_eq!(s2, test.s2);

    let s3 : Vec<u8> = vec![0x7C, 0x62, 0x61, 0x7A, 0x40];
    assert_eq!(s3, test.s3);
}