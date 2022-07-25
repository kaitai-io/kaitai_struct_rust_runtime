#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use kaitai::*;

mod helpers_tests;
use helpers_tests::*;

mod str_pad_term_empty;
use str_pad_term_empty::*;

#[test]
fn basic_parse() {
    let bytes = get_file_as_byte_vec("formats/bin/str_pad_term_empty.bin");
    let reader = BytesReader::new(&bytes);

    let mut test = StrPadTermEmpty::default();
    {
        let res = test.read(&reader, None, KStructUnit::parent_stack());
        println!("{:?}", res);
        assert!(res.is_ok());
    }

    assert_eq!("", test.str_pad);
    assert_eq!("", test.str_term);
    assert_eq!("", test.str_term_and_pad);
    assert_eq!("@", test.str_term_include);
}