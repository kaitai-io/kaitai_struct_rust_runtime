//! Module with built-in routines for `process` key.

use flate2::read::ZlibDecoder;
use std::io::{Cursor, Read, Result};

// TODO: inplace processing
pub fn process_xor_one(value: &[u8], key: u8) -> Vec<u8> {
    let mut result = vec![0; value.len()];
    for i in 0..value.len() {
        result[i] = (value[i] ^ key) as u8;
    }
    return result;
}

// TODO: inplace processing
pub fn process_xor_many(value: &[u8], key: &[u8]) -> Vec<u8> {
    let mut result = vec![0; value.len()];
    let mut j = 0;
    for i in 0..value.len() {
        result[i] = (value[i] ^ key[j]) as u8;
        j = (j + 1) % key.len();
    }
    return result;
}

// TODO: inplace processing
pub fn process_rotate_left(data: &[u8], amount: i32, group_size: i32) -> Vec<u8> {
    if amount < -7 || amount > 7 {
        panic!("Rotation of more than 7 cannot be performed.");
    }

    let mut rot_amount = amount;
    if rot_amount < 0 {
        rot_amount += 8;
    }

    let mut result = vec![0; data.len()];
    match group_size {
        1 => {
            for i in 0..data.len() {
                result[i] = data[i].rotate_left(rot_amount as u32);
            }
        }
        _ => unimplemented!("Unable to rotate a group of {} bytes yet", group_size),
    }
    return result;
}

pub fn process_zlib(data: &[u8]) -> Result<Vec<u8>> {
    let mut decoder = ZlibDecoder::new(Cursor::new(data));
    let mut result = Vec::new();
    match decoder.read_to_end(&mut result) {
        Ok(_) => Ok(result),
        Err(e) => Err(e),
    }
}
