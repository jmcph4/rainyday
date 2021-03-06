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

impl From<BitfieldPayload> for Bytes {
    fn from(value: BitfieldPayload) -> Self {
        value.bitfield
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct RequestPayload {
    index: u32,
    begin: u32,
    length: u32,
}

impl From<RequestPayload> for Bytes {
    fn from(value: RequestPayload) -> Self {
        let mut bytes: Bytes = vec![];

        bytes.extend_from_slice(&value.index.to_be_bytes());
        bytes.extend_from_slice(&value.begin.to_be_bytes());
        bytes.extend_from_slice(&value.length.to_be_bytes());

        bytes
    }
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

impl From<PiecePayload> for Bytes {
    fn from(value: PiecePayload) -> Self {
        let mut bytes: Bytes = vec![];

        bytes.extend_from_slice(&value.index.to_be_bytes());
        bytes.extend_from_slice(&value.begin.to_be_bytes());
        bytes.extend_from_slice(&value.piece);

        bytes
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

impl From<CancelPayload> for Bytes {
    fn from(value: CancelPayload) -> Self {
        let mut bytes: Bytes = vec![];

        bytes.extend_from_slice(&value.index.to_be_bytes());
        bytes.extend_from_slice(&value.begin.to_be_bytes());
        bytes.extend_from_slice(&value.length.to_be_bytes());

        bytes
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
        if value.len() < size_of::<u32>() + 1 {
            return Err(DecodeError::TooShort);
        }

        let length_bytes: [u8; 4] = [value[0], value[1], value[2], value[3]];
        let length: u32 = u32::from_be_bytes(length_bytes);

        if value.len() != (length as usize) + size_of::<u32>() {
            return Err(DecodeError::WrongLength);
        }

        let id: u8 = value[4];

        /* length check for non-payload peer messages */
        if id <= 3 && value.len() > size_of::<u32>() + 1 {
            return Err(DecodeError::TooLong);
        }

        match id {
            0 => Ok(Self::Choke),
            1 => Ok(Self::Unchoke),
            2 => Ok(Self::Interested),
            3 => Ok(Self::NotInterested),
            4 => Ok(Self::Have(HavePayload::try_from(value[5..].to_vec())?)),
            5 => Ok(Self::Bitfield(BitfieldPayload::try_from(
                value[5..].to_vec(),
            )?)),
            6 => Ok(Self::Request(RequestPayload::try_from(
                value[5..].to_vec(),
            )?)),
            7 => Ok(Self::Piece(PiecePayload::try_from(value[5..].to_vec())?)),
            8 => {
                Ok(Self::Cancel(CancelPayload::try_from(value[5..].to_vec())?))
            }
            _ => Err(DecodeError::InvalidMessageType),
        }
    }
}

impl From<PeerMessage> for Bytes {
    fn from(value: PeerMessage) -> Self {
        /* fields we'll be mutating along the way */
        let mut length: u32 = 1;
        let id: u8;
        let mut payload: Bytes = vec![];

        /* handle each message case */
        match value {
            PeerMessage::Choke => {
                id = 0;
            }
            PeerMessage::Unchoke => {
                id = 1;
            }
            PeerMessage::Interested => {
                id = 2;
            }
            PeerMessage::NotInterested => {
                id = 3;
            }
            PeerMessage::Have(p) => {
                length = 5;
                id = 4;
                payload = p.into();
            }
            PeerMessage::Bitfield(p) => {
                length = 1 + p.bitfield.len() as u32;
                id = 5;
                payload = p.into();
            }
            PeerMessage::Request(p) => {
                length = 13;
                id = 6;
                payload = {
                    let tmp: Vec<Bytes> = vec![
                        p.index.to_be_bytes().to_vec(),
                        p.begin.to_be_bytes().to_vec(),
                        p.length.to_be_bytes().to_vec(),
                    ];
                    tmp.iter().flatten().cloned().collect()
                };
            }
            PeerMessage::Piece(p) => {
                length = 9 + p.piece.len() as u32;
                id = 7;
                payload = {
                    let tmp: Vec<Bytes> = vec![
                        p.index.to_be_bytes().to_vec(),
                        p.begin.to_be_bytes().to_vec(),
                        p.piece,
                    ];
                    tmp.iter().flatten().cloned().collect()
                };
            }
            PeerMessage::Cancel(p) => {
                length = 12;
                id = 8;
                payload = {
                    let tmp: Vec<Bytes> = vec![
                        p.index.to_be_bytes().to_vec(),
                        p.begin.to_be_bytes().to_vec(),
                        p.length.to_be_bytes().to_vec(),
                    ];
                    tmp.iter().flatten().cloned().collect()
                };
            }
        }

        /* marshal everything into bytes */
        let length_bytes: Bytes = length.to_be_bytes().to_vec();
        let bytes: Vec<Bytes> = vec![length_bytes, vec![id], payload];

        bytes.iter().flatten().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_choke_normal() {
        let bytes: Bytes = vec![0x00, 0x00, 0x00, 0x01, 0x00];

        let result: Result<PeerMessage, DecodeError> =
            PeerMessage::try_from(bytes);

        assert!(result.is_ok());

        let actual_message: PeerMessage = result.unwrap();
        let expected_message: PeerMessage = PeerMessage::Choke;

        assert_eq!(actual_message, expected_message);
    }

    #[test]
    fn test_decode_choke_abnormal_bad_id() {
        let bytes: Bytes = vec![0x00, 0x00, 0x00, 0x01, 0xff];

        let result: Result<PeerMessage, DecodeError> =
            PeerMessage::try_from(bytes);

        assert!(result.is_err());

        let actual_error: DecodeError = result.unwrap_err();
        let expected_error: DecodeError = DecodeError::InvalidMessageType;

        assert_eq!(actual_error, expected_error);
    }

    #[test]
    fn test_decode_choke_abnormal_bad_length() {
        let bytes: Bytes = vec![0x00, 0x00, 0x00, 0xff, 0x00];

        let result: Result<PeerMessage, DecodeError> =
            PeerMessage::try_from(bytes);

        assert!(result.is_err());

        let actual_error: DecodeError = result.unwrap_err();
        let expected_error: DecodeError = DecodeError::WrongLength;

        assert_eq!(actual_error, expected_error);
    }

    #[test]
    fn test_decode_choke_abnormal_surplus_data() {
        let bytes: Bytes = vec![
            0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00,
        ];

        let result: Result<PeerMessage, DecodeError> =
            PeerMessage::try_from(bytes);

        assert!(result.is_err());

        let actual_error: DecodeError = result.unwrap_err();
        let expected_error: DecodeError = DecodeError::TooLong;

        assert_eq!(actual_error, expected_error);
    }

    #[test]
    fn test_decode_choke_abnormal_deficit_data() {
        let bytes: Bytes = vec![0x00, 0x00, 0x00, 0x00];

        let result: Result<PeerMessage, DecodeError> =
            PeerMessage::try_from(bytes);

        assert!(result.is_err());

        let actual_error: DecodeError = result.unwrap_err();
        let expected_error: DecodeError = DecodeError::TooShort;

        assert_eq!(actual_error, expected_error);
    }

    #[test]
    fn test_encode_choke_normal() {
        let message: PeerMessage = PeerMessage::Choke;

        let actual_bytes: Bytes = message.into();
        let expected_bytes: Bytes = vec![0x00, 0x00, 0x00, 0x01, 0x00];

        assert_eq!(actual_bytes, expected_bytes);
    }

    #[test]
    fn test_decode_unchoke_normal() {
        let bytes: Bytes = vec![0x00, 0x00, 0x00, 0x01, 0x01];

        let result: Result<PeerMessage, DecodeError> =
            PeerMessage::try_from(bytes);

        assert!(result.is_ok());

        let actual_message: PeerMessage = result.unwrap();
        let expected_message: PeerMessage = PeerMessage::Unchoke;

        assert_eq!(actual_message, expected_message);
    }

    #[test]
    fn test_decode_unchoke_abnormal_bad_id() {
        let bytes: Bytes = vec![0x00, 0x00, 0x00, 0x01, 0xff];

        let result: Result<PeerMessage, DecodeError> =
            PeerMessage::try_from(bytes);

        assert!(result.is_err());

        let actual_error: DecodeError = result.unwrap_err();
        let expected_error: DecodeError = DecodeError::InvalidMessageType;

        assert_eq!(actual_error, expected_error);
    }

    #[test]
    fn test_decode_unchoke_abnormal_bad_length() {
        let bytes: Bytes = vec![0x00, 0x00, 0x00, 0xff, 0x01];

        let result: Result<PeerMessage, DecodeError> =
            PeerMessage::try_from(bytes);

        assert!(result.is_err());

        let actual_error: DecodeError = result.unwrap_err();
        let expected_error: DecodeError = DecodeError::WrongLength;

        assert_eq!(actual_error, expected_error);
    }

    #[test]
    fn test_decode_unchoke_abnormal_surplus_data() {
        let bytes: Bytes = vec![
            0x00, 0x00, 0x00, 0x08, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00,
        ];

        let result: Result<PeerMessage, DecodeError> =
            PeerMessage::try_from(bytes);

        assert!(result.is_err());

        let actual_error: DecodeError = result.unwrap_err();
        let expected_error: DecodeError = DecodeError::TooLong;

        assert_eq!(actual_error, expected_error);
    }

    #[test]
    fn test_decode_unchoke_abnormal_deficit_data() {
        let bytes: Bytes = vec![0x00, 0x00, 0x00, 0x00];

        let result: Result<PeerMessage, DecodeError> =
            PeerMessage::try_from(bytes);

        assert!(result.is_err());

        let actual_error: DecodeError = result.unwrap_err();
        let expected_error: DecodeError = DecodeError::TooShort;

        assert_eq!(actual_error, expected_error);
    }

    #[test]
    fn test_encode_unchoke_normal() {
        let message: PeerMessage = PeerMessage::Unchoke;

        let actual_bytes: Bytes = message.into();
        let expected_bytes: Bytes = vec![0x00, 0x00, 0x00, 0x01, 0x01];

        assert_eq!(actual_bytes, expected_bytes);
    }

    #[test]
    fn test_decode_interested_normal() {
        let bytes: Bytes = vec![0x00, 0x00, 0x00, 0x01, 0x02];

        let result: Result<PeerMessage, DecodeError> =
            PeerMessage::try_from(bytes);

        assert!(result.is_ok());

        let actual_message: PeerMessage = result.unwrap();
        let expected_message: PeerMessage = PeerMessage::Interested;

        assert_eq!(actual_message, expected_message);
    }

    #[test]
    fn test_decode_interested_abnormal_bad_id() {
        let bytes: Bytes = vec![0x00, 0x00, 0x00, 0x01, 0xff];

        let result: Result<PeerMessage, DecodeError> =
            PeerMessage::try_from(bytes);

        assert!(result.is_err());

        let actual_error: DecodeError = result.unwrap_err();
        let expected_error: DecodeError = DecodeError::InvalidMessageType;

        assert_eq!(actual_error, expected_error);
    }

    #[test]
    fn test_decode_interested_abnormal_bad_length() {
        let bytes: Bytes = vec![0x00, 0x00, 0x00, 0xff, 0x02];

        let result: Result<PeerMessage, DecodeError> =
            PeerMessage::try_from(bytes);

        assert!(result.is_err());

        let actual_error: DecodeError = result.unwrap_err();
        let expected_error: DecodeError = DecodeError::WrongLength;

        assert_eq!(actual_error, expected_error);
    }

    #[test]
    fn test_decode_interested_abnormal_surplus_data() {
        let bytes: Bytes = vec![
            0x00, 0x00, 0x00, 0x08, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00,
        ];

        let result: Result<PeerMessage, DecodeError> =
            PeerMessage::try_from(bytes);

        assert!(result.is_err());

        let actual_error: DecodeError = result.unwrap_err();
        let expected_error: DecodeError = DecodeError::TooLong;

        assert_eq!(actual_error, expected_error);
    }

    #[test]
    fn test_decode_interested_abnormal_deficit_data() {
        let bytes: Bytes = vec![0x00, 0x00, 0x00, 0x00];

        let result: Result<PeerMessage, DecodeError> =
            PeerMessage::try_from(bytes);

        assert!(result.is_err());

        let actual_error: DecodeError = result.unwrap_err();
        let expected_error: DecodeError = DecodeError::TooShort;

        assert_eq!(actual_error, expected_error);
    }

    #[test]
    fn test_encode_interested_normal() {
        let message: PeerMessage = PeerMessage::Interested;

        let actual_bytes: Bytes = message.into();
        let expected_bytes: Bytes = vec![0x00, 0x00, 0x00, 0x01, 0x02];

        assert_eq!(actual_bytes, expected_bytes);
    }

    #[test]
    fn test_decode_not_interested_normal() {
        let bytes: Bytes = vec![0x00, 0x00, 0x00, 0x01, 0x03];

        let result: Result<PeerMessage, DecodeError> =
            PeerMessage::try_from(bytes);

        assert!(result.is_ok());

        let actual_message: PeerMessage = result.unwrap();
        let expected_message: PeerMessage = PeerMessage::NotInterested;

        assert_eq!(actual_message, expected_message);
    }

    #[test]
    fn test_decode_not_interested_abnormal_bad_id() {
        let bytes: Bytes = vec![0x00, 0x00, 0x00, 0x01, 0xff];

        let result: Result<PeerMessage, DecodeError> =
            PeerMessage::try_from(bytes);

        assert!(result.is_err());

        let actual_error: DecodeError = result.unwrap_err();
        let expected_error: DecodeError = DecodeError::InvalidMessageType;

        assert_eq!(actual_error, expected_error);
    }

    #[test]
    fn test_decode_not_interested_abnormal_bad_length() {
        let bytes: Bytes = vec![0x00, 0x00, 0x00, 0xff, 0x03];

        let result: Result<PeerMessage, DecodeError> =
            PeerMessage::try_from(bytes);

        assert!(result.is_err());

        let actual_error: DecodeError = result.unwrap_err();
        let expected_error: DecodeError = DecodeError::WrongLength;

        assert_eq!(actual_error, expected_error);
    }

    #[test]
    fn test_decode_not_interested_abnormal_surplus_data() {
        let bytes: Bytes = vec![
            0x00, 0x00, 0x00, 0x08, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00,
        ];

        let result: Result<PeerMessage, DecodeError> =
            PeerMessage::try_from(bytes);

        assert!(result.is_err());

        let actual_error: DecodeError = result.unwrap_err();
        let expected_error: DecodeError = DecodeError::TooLong;

        assert_eq!(actual_error, expected_error);
    }

    #[test]
    fn test_decode_not_interested_abnormal_deficit_data() {
        let bytes: Bytes = vec![0x00, 0x00, 0x00, 0x00];

        let result: Result<PeerMessage, DecodeError> =
            PeerMessage::try_from(bytes);

        assert!(result.is_err());

        let actual_error: DecodeError = result.unwrap_err();
        let expected_error: DecodeError = DecodeError::TooShort;

        assert_eq!(actual_error, expected_error);
    }

    #[test]
    fn test_encode_not_interested_normal() {
        let message: PeerMessage = PeerMessage::NotInterested;

        let actual_bytes: Bytes = message.into();
        let expected_bytes: Bytes = vec![0x00, 0x00, 0x00, 0x01, 0x03];

        assert_eq!(actual_bytes, expected_bytes);
    }

    #[test]
    fn test_decode_have_normal() {
        let bytes: Bytes =
            vec![0x00, 0x00, 0x00, 0x05, 0x04, 0x00, 0x00, 0x00, 0xff];

        let result: Result<PeerMessage, DecodeError> =
            PeerMessage::try_from(bytes);

        assert!(result.is_ok());

        let actual_message: PeerMessage = result.unwrap();
        let expected_message: PeerMessage =
            PeerMessage::Have(HavePayload { index: 255 });

        assert_eq!(actual_message, expected_message);
    }

    #[test]
    fn test_decode_have_abnormal_bad_id() {
        let bytes: Bytes =
            vec![0x00, 0x00, 0x00, 0x05, 0xff, 0x00, 0x00, 0x00, 0xff];

        let result: Result<PeerMessage, DecodeError> =
            PeerMessage::try_from(bytes);

        assert!(result.is_err());

        let actual_error: DecodeError = result.unwrap_err();
        let expected_error: DecodeError = DecodeError::InvalidMessageType;

        assert_eq!(actual_error, expected_error);
    }

    #[test]
    fn test_decode_have_abnormal_bad_length() {
        let bytes: Bytes =
            vec![0x00, 0x00, 0x00, 0xff, 0x04, 0x00, 0x00, 0x00, 0xff];

        let result: Result<PeerMessage, DecodeError> =
            PeerMessage::try_from(bytes);

        assert!(result.is_err());

        let actual_error: DecodeError = result.unwrap_err();
        let expected_error: DecodeError = DecodeError::WrongLength;

        assert_eq!(actual_error, expected_error);
    }

    #[test]
    fn test_decode_have_abnormal_surplus_data() {
        let bytes: Bytes = vec![
            0x00, 0x00, 0x00, 0x08, 0x04, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00,
            0x00,
        ];

        let result: Result<PeerMessage, DecodeError> =
            PeerMessage::try_from(bytes);

        assert!(result.is_err());

        let actual_error: DecodeError = result.unwrap_err();
        let expected_error: DecodeError = DecodeError::TooLong;

        assert_eq!(actual_error, expected_error);
    }

    #[test]
    fn test_decode_have_abnormal_deficit_data() {
        let bytes: Bytes = vec![0x00, 0x00, 0x00, 0x00];

        let result: Result<PeerMessage, DecodeError> =
            PeerMessage::try_from(bytes);

        assert!(result.is_err());

        let actual_error: DecodeError = result.unwrap_err();
        let expected_error: DecodeError = DecodeError::TooShort;

        assert_eq!(actual_error, expected_error);
    }

    #[test]
    fn test_encode_have_normal() {
        let message: PeerMessage =
            PeerMessage::Have(HavePayload { index: 255 });

        let actual_bytes: Bytes = message.into();
        let expected_bytes: Bytes =
            vec![0x00, 0x00, 0x00, 0x05, 0x04, 0x00, 0x00, 0x00, 0xff];

        assert_eq!(actual_bytes, expected_bytes);
    }

    #[test]
    fn test_decode_bitfield_normal() {
        let bytes: Bytes = vec![
            0x00, 0x00, 0x00, 0x09, 0x05, 0xca, 0xfe, 0xbe, 0xef, 0xff, 0xff,
            0xff, 0xf0,
        ];

        let result: Result<PeerMessage, DecodeError> =
            PeerMessage::try_from(bytes);

        assert!(result.is_ok());

        let actual_message: PeerMessage = result.unwrap();
        let expected_message: PeerMessage =
            PeerMessage::Bitfield(BitfieldPayload {
                bitfield: vec![0xca, 0xfe, 0xbe, 0xef, 0xff, 0xff, 0xff, 0xf0],
            });

        assert_eq!(actual_message, expected_message);
    }

    #[test]
    fn test_decode_bitfield_abnormal_bad_id() {
        let bytes: Bytes =
            vec![0x00, 0x00, 0x00, 0x05, 0xff, 0x00, 0x00, 0x00, 0xff];

        let result: Result<PeerMessage, DecodeError> =
            PeerMessage::try_from(bytes);

        assert!(result.is_err());

        let actual_error: DecodeError = result.unwrap_err();
        let expected_error: DecodeError = DecodeError::InvalidMessageType;

        assert_eq!(actual_error, expected_error);
    }

    #[test]
    fn test_decode_bitfield_abnormal_bad_length() {
        let bytes: Bytes =
            vec![0x00, 0x00, 0x00, 0xff, 0x05, 0x00, 0x00, 0x00, 0xff];

        let result: Result<PeerMessage, DecodeError> =
            PeerMessage::try_from(bytes);

        assert!(result.is_err());

        let actual_error: DecodeError = result.unwrap_err();
        let expected_error: DecodeError = DecodeError::WrongLength;

        assert_eq!(actual_error, expected_error);
    }

    #[test]
    fn test_decode_bitfield_abnormal_deficit_data() {
        let bytes: Bytes = vec![0x00, 0x00, 0x00, 0x00];

        let result: Result<PeerMessage, DecodeError> =
            PeerMessage::try_from(bytes);

        assert!(result.is_err());

        let actual_error: DecodeError = result.unwrap_err();
        let expected_error: DecodeError = DecodeError::TooShort;

        assert_eq!(actual_error, expected_error);
    }

    #[test]
    fn test_encode_bitfield_normal() {
        let message: PeerMessage = PeerMessage::Bitfield(BitfieldPayload {
            bitfield: vec![0xca, 0xfe, 0xbe, 0xef, 0xff, 0xff, 0xff, 0xf0],
        });

        let actual_bytes: Bytes = message.into();
        let expected_bytes: Bytes = vec![
            0x00, 0x00, 0x00, 0x09, 0x05, 0xca, 0xfe, 0xbe, 0xef, 0xff, 0xff,
            0xff, 0xf0,
        ];

        assert_eq!(actual_bytes, expected_bytes);
    }

    #[test]
    fn test_decode_request_normal() {
        let bytes: Bytes = vec![
            0x00, 0x00, 0x00, 0x0d, 0x06, 0x00, 0x00, 0x00, 0x21, 0x00, 0x00,
            0x08, 0x00, 0x00, 0x00, 0x01, 0x00,
        ];

        let result: Result<PeerMessage, DecodeError> =
            PeerMessage::try_from(bytes);

        assert!(result.is_ok());

        let actual_message: PeerMessage = result.unwrap();
        let expected_message: PeerMessage =
            PeerMessage::Request(RequestPayload {
                index: 33,
                begin: 2048,
                length: 256,
            });

        assert_eq!(actual_message, expected_message);
    }

    #[test]
    fn test_decode_request_abnormal_bad_id() {
        let bytes: Bytes = vec![
            0x00, 0x00, 0x00, 0x0d, 0xff, 0x00, 0x00, 0x00, 0x21, 0x00, 0x00,
            0x08, 0x00, 0x00, 0x00, 0x01, 0x00,
        ];

        let result: Result<PeerMessage, DecodeError> =
            PeerMessage::try_from(bytes);

        assert!(result.is_err());

        let actual_error: DecodeError = result.unwrap_err();
        let expected_error: DecodeError = DecodeError::InvalidMessageType;

        assert_eq!(actual_error, expected_error);
    }

    #[test]
    fn test_decode_request_abnormal_bad_length() {
        let bytes: Bytes = vec![
            0x00, 0x00, 0x00, 0xff, 0x06, 0x00, 0x00, 0x00, 0x21, 0x00, 0x00,
            0x08, 0x00, 0x00, 0x00, 0x01, 0x00,
        ];

        let result: Result<PeerMessage, DecodeError> =
            PeerMessage::try_from(bytes);

        assert!(result.is_err());

        let actual_error: DecodeError = result.unwrap_err();
        let expected_error: DecodeError = DecodeError::WrongLength;

        assert_eq!(actual_error, expected_error);
    }

    #[test]
    fn test_decode_request_abnormal_deficit_data() {
        let bytes: Bytes = vec![0x00, 0x00, 0x00, 0x00];

        let result: Result<PeerMessage, DecodeError> =
            PeerMessage::try_from(bytes);

        assert!(result.is_err());

        let actual_error: DecodeError = result.unwrap_err();
        let expected_error: DecodeError = DecodeError::TooShort;

        assert_eq!(actual_error, expected_error);
    }

    #[test]
    fn test_encode_request_normal() {
        let message: PeerMessage = PeerMessage::Request(RequestPayload {
            index: 33,
            begin: 2048,
            length: 256,
        });

        let actual_bytes: Bytes = message.into();
        let expected_bytes: Bytes = vec![
            0x00, 0x00, 0x00, 0x0d, 0x06, 0x00, 0x00, 0x00, 0x21, 0x00, 0x00,
            0x08, 0x00, 0x00, 0x00, 0x01, 0x00,
        ];

        assert_eq!(actual_bytes, expected_bytes);
    }
}
