#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use kaitai::*;

mod helpers_tests;
use helpers_tests::*;

mod process_xor_value;
use process_xor_value::*;

#[test]
fn basic_parse() {
    let bytes = get_file_as_byte_vec("formats/bin/process_xor_1.bin");
    let reader = BytesReader::new(&bytes);
    let mut test = ProcessXorValue::default();
    {
        let res = test.read(&reader, None, KStructUnit::parent_stack());
        println!("{:?}", res);
        assert!(res.is_ok());
    }

    assert_eq!(255, test.key());
    let buf1 : Vec<u8> = vec![0x66, 0x6F, 0x6F, 0x20, 0x62, 0x61, 0x72];
    assert_eq!(buf1, *test.buf());
}