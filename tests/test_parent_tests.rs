#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use kaitai::*;

mod helpers_tests;
use helpers_tests::*;

mod test_parent;
use test_parent::*;

#[test]
fn basic_parse() {
    let bytes = get_file_as_byte_vec("formats/bin/switch_integers.bin");
    let mut reader = BytesReader::new(&bytes);
    let mut test = TestParent::default();
    {
        let res = test.read(&mut reader, None, KStructUnit::parent_stack());
        println!("{:?}", res);
        assert!(res.is_ok());
    }

    assert_eq!(1, test.root_byte);
    assert_eq!(7, test.child().child_byte);
    assert_eq!(1, test.child().child2().len());
    assert_eq!(vec![2, 64, 64, 4, 55, 19, 0], *test.child().child2()[0].child2_byte());
}
