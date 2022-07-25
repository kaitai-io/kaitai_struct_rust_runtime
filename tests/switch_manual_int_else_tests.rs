#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use kaitai::*;

mod helpers_tests;
use helpers_tests::*;

mod switch_manual_int_else;
use switch_manual_int_else::*;

#[test]
fn basic_parse() {
    let bytes = get_file_as_byte_vec("formats/bin/switch_opcodes2.bin");
    let reader = BytesReader::new(&bytes);

    let mut test = SwitchManualIntElse::default();
    {
        let res = test.read(&reader, None, KStructUnit::parent_stack());
        println!("{:?}", res);
        assert!(res.is_ok());
    }

    assert_eq!(4, test.opcodes().len());

    assert_eq!(83, test.opcodes()[0].code);
    if let SwitchManualIntElse_Opcode_Body::SwitchManualIntElse_Opcode_Strval(s) =  test.opcodes[0].body.as_ref().unwrap() {
        assert_eq!("foo", s.value);
    } else {
        panic!("expected enum SwitchManualIntElse_Opcode_Strval");
    }

    assert_eq!(88, test.opcodes()[1].code);
    if let SwitchManualIntElse_Opcode_Body::SwitchManualIntElse_Opcode_Noneval(s) =  test.opcodes[1].body.as_ref().unwrap() {
        assert_eq!(66, s.filler);
    } else {
        panic!("expected enum SwitchManualIntElse_Opcode_Noneval");
    }

    assert_eq!(89, test.opcodes()[2].code);
    if let SwitchManualIntElse_Opcode_Body::SwitchManualIntElse_Opcode_Noneval(s) =  test.opcodes[2].body.as_ref().unwrap() {
    assert_eq!(51966, s.filler);
    } else {
        panic!("expected enum SwitchManualIntElse_Opcode_Noneval");
    }

    assert_eq!(73, test.opcodes()[3].code);
    if let SwitchManualIntElse_Opcode_Body::SwitchManualIntElse_Opcode_Intval(s) =  test.opcodes[3].body.as_ref().unwrap() {
        assert_eq!(7, s.value);
    } else {
        panic!("expected enum SwitchManualIntElse_Opcode_Intval");
    }
}