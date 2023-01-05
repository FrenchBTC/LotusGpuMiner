use std::convert::TryInto;

use serde::Deserialize;

pub struct Block {
    pub header: [u8; 160],
    pub body: Vec<u8>,
    pub target: [u8; 32],
    pub bignonce: u64
}

#[derive(Deserialize, Debug, Clone)]
pub struct GetRawUnsolvedBlockResponse {
    pub result: Option<RawUnsolvedBlockAndTarget>,
    pub error: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RawUnsolvedBlockAndTarget {
    pub blockhex: String,
    pub target: String,
    pub bignonce: String
}

pub fn as_u32_le(array: &[u8; 8]) -> u64 {
    ((array[0] as u64) <<  0) +
    ((array[1] as u64) <<  8) +
    ((array[2] as u64) << 16) +
    ((array[3] as u64) << 24) +

    ((array[4] as u64) << 32) +
    ((array[5] as u64) << 38) +
    ((array[6] as u64) << 46) +
    ((array[7] as u64) << 54)
}

pub fn create_block(unsolved_block_and_target: &RawUnsolvedBlockAndTarget) -> Block {
    let block = hex::decode(&unsolved_block_and_target.blockhex).unwrap();
    let bignonce_byte_arr: [u8; 8] = hex::decode(&unsolved_block_and_target.bignonce)
        .unwrap()
        .try_into()
        .unwrap();
    let bignonce: u64 = as_u32_le(&bignonce_byte_arr);
    // nBits (4 bytes)
    let mut target: [u8; 32] = hex::decode(&unsolved_block_and_target.target)
        .unwrap()
        .try_into()
        .unwrap();
    target.reverse();
    Block {
        header: block[0..160].try_into().unwrap(),
        body: block[160..].try_into().unwrap(),
        target,
        bignonce
    }
}

impl Block {
    pub fn prev_hash(&self) -> &[u8] {
        &self.header[..32]
    }
}
