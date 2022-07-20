#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use kaitai::*;

mod helpers_tests;
use helpers_tests::*;

mod str_eos;
use str_eos::*;

#[test]
fn basic_parse() {
    let bytes = get_file_as_byte_vec("formats/bin/term_strz.bin");
    let mut reader = BytesReader::new(&bytes);
    let mut test = StrEos::default();
    {
        let res = test.read(&mut reader, None, KStructUnit::parent_stack());
        println!("{:?}", res);
        assert!(res.is_ok());
    }

    assert_eq!("foo|bar|baz@", test.str);
}