#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use kaitai::*;

mod helpers_tests;
use helpers_tests::*;

mod str_encodings_default;
use str_encodings_default::*;

#[test]
fn basic_parse() {
    let bytes = get_file_as_byte_vec("formats/bin/str_encodings.bin");
    let mut reader = BytesReader::new(&bytes);

    let mut test = StrEncodingsDefault::default();
    {
        let res = test.read(&mut reader, None, KStructUnit::parent_stack());
        println!("{:?}", res);
        assert!(res.is_ok());
    }

    assert_eq!(10, test.len_of_1);
    assert_eq!("Some ASCII", test.str1);

    assert_eq!(15, test.rest().len_of_2);
    assert_eq!("\u{3053}\u{3093}\u{306b}\u{3061}\u{306f}", test.rest().str2);

    assert_eq!(10, test.rest().len_of_3);
    assert_eq!("\u{3053}\u{3093}\u{306b}\u{3061}\u{306f}", test.rest().str3);

    assert_eq!(3, test.rest().len_of_4);
    assert_eq!("\u{2591}\u{2592}\u{2593}", test.rest().str4);
}