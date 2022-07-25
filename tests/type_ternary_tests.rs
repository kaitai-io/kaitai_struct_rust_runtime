#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use kaitai::*;

mod helpers_tests;
use helpers_tests::*;

mod type_ternary;
use type_ternary::*;

#[test]
fn basic_parse() {
    let bytes = get_file_as_byte_vec("formats/bin/term_strz.bin");
    let reader = BytesReader::new(&bytes);
    let mut test = TypeTernary::default();
    {
        let res = test.read(&reader, None, KStructUnit::parent_stack());
        println!("{:?}", res);
        assert!(res.is_ok());
    }

    assert_eq!(101, test.dif(&reader).unwrap().value)
}