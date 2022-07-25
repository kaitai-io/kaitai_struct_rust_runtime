#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use kaitai::*;

mod helpers_tests;
use helpers_tests::*;

mod process_coerce_bytes;
use process_coerce_bytes::*;

#[test]
fn basic_parse() {
    let bytes = get_file_as_byte_vec("formats/bin/process_coerce_bytes.bin");
    let reader = BytesReader::new(&bytes);
    let mut test = ProcessCoerceBytes::default();
    {
        let res = test.read(&reader, None, KStructUnit::parent_stack());
        println!("{:?}", res);
        assert!(res.is_ok());
    }

    assert_eq!(0, test.records[0].flag());

    let mut i = test.records.iter_mut();

    let buf : Vec<u8> = vec![0x41, 0x41, 0x41, 0x41];
    let x = i.next().unwrap();
    assert_eq!(&buf, x.buf(&reader).unwrap());

    let buf : Vec<u8> = vec![0x42, 0x42, 0x42, 0x42];
    let x = i.next().unwrap();
    assert_eq!(&buf, x.buf(&reader).unwrap());
}
