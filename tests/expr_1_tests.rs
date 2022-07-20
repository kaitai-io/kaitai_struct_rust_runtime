#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use kaitai::*;

mod helpers_tests;
use helpers_tests::*;

mod expr_1;
use expr_1::*;

#[test]
fn basic_parse() {
    let bytes = get_file_as_byte_vec("formats/bin/str_encodings.bin");
    let mut reader = BytesReader::new(&bytes);

    let mut test = Expr1::default();
    {
        let res = test.read(&mut reader, None, KStructUnit::parent_stack());
        println!("{:?}", res);
        assert!(res.is_ok());
    }

    assert_eq!(10, test.len_of_1);

    let res = test.len_of_1_mod(&mut reader, None, KStructUnit::parent_stack());
    assert_eq!(8, *res.unwrap());

    assert_eq!("Some ASC", *test.str1());

    let res = test.str1_len(&mut reader, None, KStructUnit::parent_stack());
    assert_eq!(8, *res.unwrap());
}