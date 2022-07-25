#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use kaitai::*;

mod helpers_tests;
use helpers_tests::*;

mod process_to_user;
use process_to_user::*;

#[test]
fn basic_parse() {
    let bytes = get_file_as_byte_vec("formats/bin/process_rotate.bin");
    let mut reader = BytesReader::new(&bytes);
    let mut test = ProcessToUser::default();
    {
        let res = test.read(&mut reader, None, KStructUnit::parent_stack());
        println!("{:?}", res);
        assert!(res.is_ok());
    }

    assert_eq!("Hello", *test.buf1().str());
}