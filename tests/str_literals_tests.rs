#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use kaitai::*;

mod helpers_tests;
use helpers_tests::*;

mod str_literals;
use str_literals::*;

#[test]
fn basic_parse() {
    let bytes = get_file_as_byte_vec("formats/bin/fixed_struct.bin");
    let reader = BytesReader::new(&bytes);

    let mut test = StrLiterals::default();
    {
        let res = test.read(&reader, None, KStructUnit::parent_stack());
        println!("{:?}", res);
        assert!(res.is_ok());
    }

    let backslashes = test.backslashes(&reader).unwrap();
    assert_eq!("\u{005c}\u{005c}\u{005c}", *backslashes);
    
    let octal_eatup = test.octal_eatup(&reader).unwrap();
    assert_eq!("\u{0}\u{0032}\u{0032}", *octal_eatup);

    let octal_eatup2 = test.octal_eatup2(&reader).unwrap();
    assert_eq!("\u{2}\u{32}", *octal_eatup2);

    let double_quotes = test.double_quotes(&reader).unwrap();
    assert_eq!("\u{22}\u{22}\u{22}", *double_quotes);

    let complex_str = test.complex_str(&reader).unwrap();
    assert_eq!("\u{0}\u{1}\u{2}\u{7}\u{8}\u{0a}\u{0d}\u{09}\u{0b}\u{c}\u{1b}\u{3d}\u{7}\u{a}\u{24}\u{263b}", *complex_str);
}