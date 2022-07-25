#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(irrefutable_let_patterns)]

use kaitai::*;

mod helpers_tests;
use helpers_tests::*;

mod switch_else_only;
use switch_else_only::*;

#[test]
fn basic_parse() {
    let bytes = get_file_as_byte_vec("formats/bin/switch_opcodes.bin");
    let reader = BytesReader::new(&bytes);

    let mut test = SwitchElseOnly::default();
    {
        let res = test.read(&reader, None, KStructUnit::parent_stack());
        println!("{:?}", res);
        assert!(res.is_ok());
    }

    assert_eq!(83, test.opcode);
    let SwitchElseOnly_PrimByte::S1(v) = test.prim_byte.as_ref().unwrap();
    assert_eq!(102, *v);

    let SwitchElseOnly_Ut::SwitchElseOnly_Data(d) = test.ut.as_ref().unwrap();
    assert_eq!(vec![0x72, 0x00, 0x49, 0x42], d.value);
}