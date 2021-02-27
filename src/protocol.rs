#![allow(dead_code)]
use std::convert::TryFrom;
use std::fmt::Display;
use std::mem::size_of;

use thiserror::Error;

type Bytes = Vec<u8>;

#[derive(Debug, Display, PartialEq, Eq, Error)]
pub enum DecodeError {
    TooLong,
    TooShort,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct HavePayload {
    index: u32,
}

impl TryFrom<Bytes> for HavePayload {
    type Error = DecodeError;

    fn try_from(value: Bytes) -> Result<Self, Self::Error> {
        if value.len() > size_of::<u32>() {
            return Err(Self::Error::TooLong);
        }

        if value.len() < size_of::<u32>() {
            return Err(Self::Error::TooShort);
        }

        let bytes_array: [u8; 4] = [value[0], value[1], value[2], value[3]];
        Ok(Self {
            index: u32::from_be_bytes(bytes_array),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BitfieldPayload {
    bitfield: Vec<u8>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct RequestPayload {
    index: u32,
    begin: u32,
    length: u32,
}

impl TryFrom<Bytes> for RequestPayload {
    type Error = DecodeError;

    fn try_from(value: Bytes) -> Result<Self, Self::Error> {
        if value.len() > 3 * size_of::<u32>() {
            return Err(Self::Error::TooLong);
        }

        if value.len() < 3 * size_of::<u32>() {
            return Err(Self::Error::TooShort);
        }

        let index_bytes: [u8; 4] = [value[0], value[1], value[2], value[3]];
        let begin_bytes: [u8; 4] = [value[4], value[5], value[6], value[7]];
        let length_bytes: [u8; 4] = [value[8], value[9], value[10], value[11]];

        Ok(Self {
            index: u32::from_be_bytes(index_bytes),
            begin: u32::from_be_bytes(begin_bytes),
            length: u32::from_be_bytes(length_bytes),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PiecePayload {
    index: usize,
    begin: usize,
    piece: Vec<u8>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct CancelPayload {
    index: usize,
    begin: usize,
    length: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum PeerMessage {
    Choke,
    Unchoke,
    Interested,
    NotInterested,
    Have(HavePayload),
    Bitfield(BitfieldPayload),
    Request(RequestPayload),
    Piece(PiecePayload),
    Cancel(CancelPayload),
}
