#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use kaitai::*;

mod helpers_tests;
use helpers_tests::*;

mod str_repeat;
use str_repeat::*;

#[test]
fn basic_parse() {
    let bytes = get_file_as_byte_vec("formats/bin/term_strz.bin");
    let mut reader = BytesReader::new(&bytes);
    let mut test = StrRepeat::default();
    {
        let res = test.read(&mut reader, None, KStructUnit::parent_stack());
        println!("{:?}", res);
        assert!(res.is_ok());
    }

    assert_eq!("foo|", test.entries[0]);
    assert_eq!("bar|", test.entries[1]);
    assert_eq!("baz@", test.entries[2]);
}
