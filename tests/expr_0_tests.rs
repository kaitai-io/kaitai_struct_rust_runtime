#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use kaitai::*;

mod helpers_tests;
use helpers_tests::*;

mod expr_0;
use expr_0::*;

#[test]
fn basic_parse() {
    let bytes = get_file_as_byte_vec("formats/bin/str_encodings.bin");
    let mut reader = BytesReader::new(&bytes);

    let mut test = Expr0::default();
    {
        let res = test.read(&mut reader, None, KStructUnit::parent_stack());
        println!("{:?}", res);
        assert!(res.is_ok());
    }

    assert_eq!(10, test.len_of_1);

    let res = test.must_be_f7(&mut reader, None, KStructUnit::parent_stack());
    assert_eq!(0xf7, *res.unwrap());

    let res = test.must_be_abc123(&mut reader, None, KStructUnit::parent_stack());
    assert_eq!("abc123", res.unwrap());
}