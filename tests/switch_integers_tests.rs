#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use kaitai::*;

mod helpers_tests;
use helpers_tests::*;

mod switch_integers;
use switch_integers::*;

#[test]
fn basic_parse() {
    let bytes = get_file_as_byte_vec("formats/bin/switch_integers.bin");
    let reader = BytesReader::new(&bytes);

    let mut test = SwitchIntegers::default();
    {
        let res = test.read(&reader, None, KStructUnit::parent_stack());
        println!("{:?}", res);
        assert!(res.is_ok());
    }

    assert_eq!(4, test.opcodes.len());

    assert_eq!(1, test.opcodes[0].code);
    let i : u8 = test.opcodes[0].body.as_ref().unwrap().into();
    assert_eq!(7, i);

    assert_eq!(2, test.opcodes[1].code);
    let i : u16 = test.opcodes[1].body.as_ref().unwrap().into();
    assert_eq!(16448, i);

    assert_eq!(4, test.opcodes[2].code);
    let i : u32 = test.opcodes[2].body.as_ref().unwrap().into();
    assert_eq!(4919, i);

    assert_eq!(8, test.opcodes[3].code);
    let i : u64 = test.opcodes[3].body.as_ref().unwrap().into();
    assert_eq!(4919, i);
}