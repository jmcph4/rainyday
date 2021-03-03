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
    WrongLength,
    InvalidMessageType,
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

impl From<HavePayload> for Bytes {
    fn from(value: HavePayload) -> Self {
        value.index.to_be_bytes().to_vec()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BitfieldPayload {
    bitfield: Vec<u8>,
}

impl TryFrom<Bytes> for BitfieldPayload {
    type Error = DecodeError;

    fn try_from(value: Bytes) -> Result<Self, Self::Error> {
        Ok(Self { bitfield: value })
    }
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
    index: u32,
    begin: u32,
    piece: Vec<u8>,
}

impl TryFrom<Bytes> for PiecePayload {
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

        Ok(Self {
            index: u32::from_be_bytes(index_bytes),
            begin: u32::from_be_bytes(begin_bytes),
            piece: value[8..].to_vec(),
        })
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct CancelPayload {
    index: u32,
    begin: u32,
    length: u32,
}

impl TryFrom<Bytes> for CancelPayload {
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

impl TryFrom<Bytes> for PeerMessage {
    type Error = DecodeError;

    fn try_from(value: Bytes) -> Result<Self, Self::Error> {
        if value.len() < 2 * size_of::<u32>() {
            return Err(DecodeError::TooShort);
        }

        let length_bytes: [u8; 4] = [value[0], value[1], value[2], value[3]];
        let length: u32 = u32::from_be_bytes(length_bytes);

        if value.len() != length as usize {
            return Err(DecodeError::WrongLength);
        }

        let id_bytes: [u8; 4] = [value[4], value[5], value[6], value[7]];
        let id: u32 = u32::from_be_bytes(id_bytes);

        match id {
            0 => Ok(Self::Choke),
            1 => Ok(Self::Unchoke),
            2 => Ok(Self::Interested),
            3 => Ok(Self::NotInterested),
            4 => Ok(Self::Have(HavePayload::try_from(value[8..].to_vec())?)),
            5 => Ok(Self::Bitfield(BitfieldPayload::try_from(
                value[8..].to_vec(),
            )?)),
            6 => Ok(Self::Request(RequestPayload::try_from(
                value[8..].to_vec(),
            )?)),
            7 => Ok(Self::Piece(PiecePayload::try_from(value[8..].to_vec())?)),
            8 => {
                Ok(Self::Cancel(CancelPayload::try_from(value[8..].to_vec())?))
            }
            _ => Err(DecodeError::InvalidMessageType),
        }
    }
}
