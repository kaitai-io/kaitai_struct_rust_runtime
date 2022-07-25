#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use kaitai::*;

mod helpers_tests;
use helpers_tests::*;

mod term_strz;
use term_strz::*;

#[test]
fn basic_parse() {
    let bytes = get_file_as_byte_vec("formats/bin/term_strz.bin");
    let reader = BytesReader::new(&bytes);
    let mut test = TermStrz::default();
    {
        let res = test.read(&reader, None, KStructUnit::parent_stack());
        println!("{:?}", res);
        assert!(res.is_ok());
    }

    assert_eq!("foo", test.s1);
    assert_eq!("bar", test.s2);
    assert_eq!("|baz@", test.s3);
}