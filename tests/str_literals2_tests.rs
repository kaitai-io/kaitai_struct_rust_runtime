#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use kaitai::*;

mod helpers_tests;
use helpers_tests::*;

mod str_literals2;
use str_literals2::*;

#[test]
fn basic_parse() {
    let bytes = get_file_as_byte_vec("formats/bin/fixed_struct.bin");
    let reader = BytesReader::new(&bytes);

    let mut test = StrLiterals2::default();
    {
        let res = test.read(&reader, None, KStructUnit::parent_stack());
        println!("{:?}", res);
        assert!(res.is_ok());
    }

    let dollar1 = test.dollar1(&reader).unwrap();
    assert_eq!("$foo", *dollar1);
    
    let dollar2 = test.dollar2(&reader).unwrap();
    assert_eq!("${foo}", *dollar2);

    let hash = test.hash(&reader).unwrap();
    assert_eq!("#{foo}", *hash);

    let at_sign = test.at_sign(&reader).unwrap();
    assert_eq!("@foo", *at_sign);
}