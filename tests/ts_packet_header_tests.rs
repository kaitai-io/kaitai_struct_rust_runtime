#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use kaitai::*;

mod helpers_tests;
use helpers_tests::*;

mod ts_packet_header;
use ts_packet_header::*;

#[test]
fn basic_parse() {
    let bytes = get_file_as_byte_vec("formats/bin/ts_packet.bin");
    let mut reader = BytesReader::new(&bytes);
    let mut test = TsPacketHeader::default();
    {
        let res = test.read(&mut reader, None, KStructUnit::parent_stack());
        println!("{:?}", res);
        assert!(res.is_ok());
    }

    assert_eq!(71, test.sync_byte());
    assert_eq!(false, test.transport_error_indicator());
    assert_eq!(false, test.payload_unit_start_indicator());
    assert_eq!(true, test.transport_priority());
    assert_eq!(33, test.pid());
    assert_eq!(0, test.transport_scrambling_control());
    assert_eq!(TsPacketHeader_AdaptationFieldControlEnum::PayloadOnly, *test.adaptation_field_control());
}
