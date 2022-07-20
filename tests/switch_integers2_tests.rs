#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use kaitai::*;

mod helpers_tests;
use helpers_tests::*;

mod switch_integers2;
use switch_integers2::*;

#[test]
fn basic_parse() {
    let bytes = get_file_as_byte_vec("formats/bin/switch_integers.bin");
    let mut reader = BytesReader::new(&bytes);

    let mut test = SwitchIntegers2::default();
    {
        let res = test.read(&mut reader, None, KStructUnit::parent_stack());
        println!("{:?}", res);
        assert!(res.is_ok());
    }

    assert_eq!(1, test.code);
    assert_eq!(7, test.len());
    assert_eq!(vec![0x02u8, 0x40u8, 0x40u8, 0x04u8, 0x37u8, 0x13u8, 0x00u8], *test.ham());
    assert_eq!(0, test.padding);

    test.len_mod_str(&mut reader, None, KStructUnit::parent_stack()).unwrap();
    assert_eq!("13", test.len_mod_str.unwrap());
}