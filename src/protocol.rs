#![allow(dead_code)]

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct HavePayload {
    index: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BitfieldPayload {
    bitfield: Vec<u8>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct RequestPayload {
    index: usize,
    begin: usize,
    length: usize,
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
