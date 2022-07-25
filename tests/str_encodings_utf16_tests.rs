#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use kaitai::*;

mod helpers_tests;
use helpers_tests::*;

mod str_encodings_utf16;
use str_encodings_utf16::*;

#[test]
fn basic_parse() {
    let bytes = get_file_as_byte_vec("formats/bin/str_encodings_utf16.bin");
    let reader = BytesReader::new(&bytes);

    let mut test = StrEncodingsUtf16::default();
    {
        let res = test.read(&reader, None, KStructUnit::parent_stack());
        println!("{:?}", res);
        assert!(res.is_ok());
    }

    assert_eq!(12, test.len_be);
    assert!(test.be_bom_removed.is_some());
    let be_bom_removed = test.be_bom_removed.as_ref().unwrap();
    assert_eq!(65279, be_bom_removed.bom);
    assert_eq!("\u{3053}\u{3093}\u{306b}\u{3061}\u{306f}", be_bom_removed.str);

    assert_eq!(12, test.len_le);
    assert!(test.le_bom_removed.is_some());
    let le_bom_removed = test.le_bom_removed.as_ref().unwrap();
    assert_eq!(65279, le_bom_removed.bom);
    assert_eq!("\u{3053}\u{3093}\u{306b}\u{3061}\u{306f}", le_bom_removed.str);
}