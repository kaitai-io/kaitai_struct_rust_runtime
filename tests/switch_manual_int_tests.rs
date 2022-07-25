#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use kaitai::*;

mod helpers_tests;
use helpers_tests::*;

mod switch_manual_int;
use switch_manual_int::*;

#[test]
fn basic_parse() {
    let bytes = get_file_as_byte_vec("formats/bin/switch_opcodes.bin");
    let reader = BytesReader::new(&bytes);

    let mut test = SwitchManualInt::default();
    {
        let res = test.read(&reader, None, KStructUnit::parent_stack());
        println!("{:?}", res);
        assert!(res.is_ok());
    }

    assert_eq!(4, test.opcodes().len());

    assert_eq!(83, test.opcodes()[0].code);
    if let SwitchManualInt_Opcode_Body::SwitchManualInt_Opcode_Strval(s) =  test.opcodes[0].body.as_ref().unwrap() {
        assert_eq!("foobar", s.value);
    } else {
        panic!("expected enum SwitchManualInt_Opcode_Body");
    }

    assert_eq!(73, test.opcodes()[1].code);
    if let SwitchManualInt_Opcode_Body::SwitchManualInt_Opcode_Intval(s) =  test.opcodes[1].body.as_ref().unwrap() {
        assert_eq!(66, s.value);
    } else {
        panic!("expected enum SwitchManualInt_Opcode_Body");
    }

    assert_eq!(73, test.opcodes()[2].code);
    if let SwitchManualInt_Opcode_Body::SwitchManualInt_Opcode_Intval(s) =  test.opcodes[2].body.as_ref().unwrap() {
    assert_eq!(55, s.value);
    } else {
        panic!("expected enum SwitchManualInt_Opcode_Body");
    }

    assert_eq!(83, test.opcodes()[3].code);
    if let SwitchManualInt_Opcode_Body::SwitchManualInt_Opcode_Strval(s) =  test.opcodes[3].body.as_ref().unwrap() {
        assert_eq!("", s.value);
    } else {
        panic!("expected enum SwitchManualInt_Opcode_Body");
    }
}