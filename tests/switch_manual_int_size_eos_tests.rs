#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use kaitai::*;

mod helpers_tests;
use helpers_tests::*;

mod switch_manual_int_size_eos;
use switch_manual_int_size_eos::*;

#[test]
fn basic_parse() {
    let bytes = get_file_as_byte_vec("formats/bin/switch_tlv.bin");
    let reader = BytesReader::new(&bytes);

    let mut test = SwitchManualIntSizeEos::default();
    {
        let res = test.read(&reader, None, KStructUnit::parent_stack());
        println!("{:?}", res);
        assert!(res.is_ok());
    }

    assert_eq!(4, test.chunks().len());

    assert_eq!(17, test.chunks()[0].code);
    if let SwitchManualIntSizeEos_Chunk_ChunkBody_Body::SwitchManualIntSizeEos_Chunk_ChunkBody_ChunkMeta(s) =  test.chunks[0].body.as_ref().unwrap().body() {
        assert_eq!("Stuff", s.title);
        assert_eq!("Me", s.author);
    } else {
        panic!("expected enum SwitchManualIntSizeEos_Chunk_ChunkBody_ChunkMeta");
    }

    assert_eq!(34, test.chunks()[1].code);
    if let SwitchManualIntSizeEos_Chunk_ChunkBody_Body::SwitchManualIntSizeEos_Chunk_ChunkBody_ChunkDir(s) =  test.chunks[1].body.as_ref().unwrap().body() {
        let strings : Vec<String> = vec!["AAAA", "BBBB", "CCCC"].iter().map(|&s| s.to_string() ).collect();
        assert_eq!(strings, *s.entries());
    } else {
        panic!("expected enum SwitchManualIntSizeEos_Chunk_ChunkBody_ChunkDir");
    }
 
    assert_eq!(51, test.chunks()[2].code);
    if let SwitchManualIntSizeEos_Chunk_ChunkBody_Body::Bytes(s) =  test.chunks[2].body.as_ref().unwrap().body() {
        let raw : Vec<u8> = vec![0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80];
        assert_eq!(raw, *s);
    } else {
        panic!("expected enum Bytes");
    }
 
    assert_eq!(255, test.chunks()[3].code);
    if let SwitchManualIntSizeEos_Chunk_ChunkBody_Body::Bytes(s) =  test.chunks[3].body.as_ref().unwrap().body() {
        let raw : Vec<u8> = vec![];
        assert_eq!(raw, *s);
    } else {
        panic!("expected enum Bytes");
    }
}