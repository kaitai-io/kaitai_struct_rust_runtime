#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use kaitai::*;

mod helpers_tests;
use helpers_tests::*;

mod switch_manual_int_size;
use switch_manual_int_size::*;

#[test]
fn basic_parse() {
    let bytes = get_file_as_byte_vec("formats/bin/switch_tlv.bin");
    let reader = BytesReader::new(&bytes);

    let mut test = SwitchManualIntSize::default();
    {
        let res = test.read(&reader, None, KStructUnit::parent_stack());
        println!("{:?}", res);
        assert!(res.is_ok());
    }

    assert_eq!(4, test.chunks().len());

    assert_eq!(17, test.chunks()[0].code);
    if let SwitchManualIntSize_Chunk_Body::SwitchManualIntSize_Chunk_ChunkMeta(s) =  test.chunks[0].body.as_ref().unwrap() {
        assert_eq!("Stuff", s.title);
        assert_eq!("Me", s.author);
    } else {
        panic!("expected enum SwitchManualIntSize_Chunk_ChunkMeta");
    }

    assert_eq!(34, test.chunks()[1].code);
    if let SwitchManualIntSize_Chunk_Body::SwitchManualIntSize_Chunk_ChunkDir(s) =  test.chunks[1].body.as_ref().unwrap() {
        let strings : Vec<String> = vec!["AAAA", "BBBB", "CCCC"].iter().map(|&s| s.to_string() ).collect();
        assert_eq!(strings, *s.entries());
    } else {
        panic!("expected enum SwitchManualIntSize_Chunk_ChunkDir");
    }
 
    assert_eq!(51, test.chunks()[2].code);
    if let SwitchManualIntSize_Chunk_Body::Bytes(s) =  test.chunks[2].body.as_ref().unwrap() {
        let raw : Vec<u8> = vec![0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80];
        assert_eq!(raw, *s);
    } else {
        panic!("expected enum Bytes");
    }
 
    assert_eq!(255, test.chunks()[3].code);
    if let SwitchManualIntSize_Chunk_Body::Bytes(s) =  test.chunks[3].body.as_ref().unwrap() {
        let raw : Vec<u8> = vec![];
        assert_eq!(raw, *s);
    } else {
        panic!("expected enum Bytes");
    }
}