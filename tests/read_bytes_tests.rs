#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use kaitai::*;

mod helpers_tests;
use helpers_tests::*;

mod read_bytes;
use read_bytes::*;

#[test]
fn basic_parse() {
    let bytes = get_file_as_byte_vec("formats/bin/switch_integers.bin");
    let mut reader = BytesReader::new(&bytes);

    let mut test = ReadBytes::default();
    {
        let res = test.read(&mut reader, None, KStructUnit::parent_stack());
        println!("{:?}", res);
        assert!(res.is_ok());
    }

    assert_eq!(1, test.len());
    assert_eq!(vec![7], *test.val());
    assert_eq!(vec![0x2, 0x40], *test.padding());
}
