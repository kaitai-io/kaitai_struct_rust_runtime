#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use kaitai::*;

mod helpers_tests;
use helpers_tests::*;

mod str_pad_term;
use str_pad_term::*;

#[test]
fn basic_parse() {
    let bytes = get_file_as_byte_vec("formats/bin/str_pad_term.bin");
    let mut reader = BytesReader::new(&bytes);

    let mut test = StrPadTerm::default();
    {
        let res = test.read(&mut reader, None, KStructUnit::parent_stack());
        println!("{:?}", res);
        assert!(res.is_ok());
    }

    assert_eq!("str1", test.str_pad);
    assert_eq!("str2foo", test.str_term);
    assert_eq!("str+++3bar+++", test.str_term_and_pad);
    assert_eq!("str4baz@", test.str_term_include);
}
