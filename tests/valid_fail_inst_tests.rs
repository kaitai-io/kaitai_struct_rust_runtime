#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use kaitai::*;

mod helpers_tests;
use helpers_tests::*;

mod valid_fail_inst;
use valid_fail_inst::*;

#[test]
fn basic_parse() {
    let bytes = get_file_as_byte_vec("formats/bin/fixed_struct.bin");
    let reader = BytesReader::new(&bytes);
    let mut test = ValidFailInst::default();
    {
        let res = test.read(&reader, None, KStructUnit::parent_stack());
        println!("{:?}", res);
        if let KError::ValidationNotEqual(desc) = res.unwrap_err() {
            println!("Validation not equal: {}", desc);
        }
    }
}