//! Module with built-in routines for `process` key.

use flate2::read::ZlibDecoder;
use std::io::{Cursor, Read, Result};

/// Performs a XOR processing with given data, XORing every byte of input with a single given value.
/// Returns processed data
///
/// # Examples
///
/// ```
/// # use kaitai_struct::process_xor_one;
/// # use pretty_assertions::assert_eq;
/// assert_eq!(
///     process_xor_one(b"Hello world", 0x10),
///     &[88, 117, 124, 124, 127, 48, 103, 127, 98, 124, 116]
/// );
/// ```
///
/// # Parameters
/// - `data`: data to process
/// - `key`: value to XOR with
// TODO: inplace processing
pub fn process_xor_one(value: &[u8], key: u8) -> Vec<u8> {
    let mut result = vec![0; value.len()];
    for i in 0..value.len() {
        result[i] = (value[i] ^ key) as u8;
    }
    return result;
}

/// Performs a XOR processing with given data, XORing every byte of input with a key
/// array, repeating key array many times, if necessary (i.e. if data array is longer
/// than key array).
///
/// # Examples
///
/// ```
/// # use kaitai_struct::process_xor_many;
/// # use pretty_assertions::assert_eq;
/// assert_eq!(
///     process_xor_many(b"Hello world", b"secret"),
///     &[59, 0, 15, 30, 10, 84, 4, 10, 17, 30, 1]
/// );
/// ```
///
/// # Parameters
/// - `data`: data to process
/// - `key`: array of bytes to XOR with
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

/// Performs a circular left rotation shift for a given buffer by a given amount of bits,
/// using groups of `group_size` bytes each time. Right circular rotation should be performed
/// using this procedure with corrected amount.
///
/// Returns copy of source array with requested shift applied.
///
/// # Panics
/// If `abs(amount) > 7` then a panic is generated.
///
/// NOTE: also panic is generated when `group_size` is not 1, because it is not implemented yet.
///
/// # Parameters
/// - `data`: source data to process
/// - `amount`: number of bits to shift by
/// - `group_size`: number of bytes per group to shift
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

/// Performs an unpacking ("inflation") of zlib-compressed data with usual zlib headers.
///
/// # Parameters
/// - `data`: compressed data
pub fn process_zlib(data: &[u8]) -> Result<Vec<u8>> {
    let mut decoder = ZlibDecoder::new(Cursor::new(data));
    let mut result = Vec::new();
    match decoder.read_to_end(&mut result) {
        Ok(_) => Ok(result),
        Err(e) => Err(e),
    }
}
