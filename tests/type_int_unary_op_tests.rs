#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use kaitai::*;

mod helpers_tests;
use helpers_tests::*;

mod type_int_unary_op;
use type_int_unary_op::*;

#[test]
fn basic_parse() {
    let bytes = get_file_as_byte_vec("formats/bin/fixed_struct.bin");
    let mut reader = BytesReader::new(&bytes);
    let mut test = TypeIntUnaryOp::default();
    {
        let res = test.read(&mut reader, None, KStructUnit::parent_stack());
        println!("{:?}", res);
        assert!(res.is_ok());
    }

    assert_eq!(16720, test.value_s2);
    assert_eq!(4706543082108963651i64, test.value_s8);
    assert_eq!(-16720, *test.unary_s2(&mut reader, None, KStructUnit::parent_stack()).unwrap());
    assert_eq!(-4706543082108963651i64, *test.unary_s8(&mut reader, None, KStructUnit::parent_stack()).unwrap());
}