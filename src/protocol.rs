#![allow(dead_code)]

pub struct HavePayload {
    index: usize,
}

pub struct BitfieldPayload {
    bitfield: Vec<u8>,
}

pub struct RequestPayload {
    index: usize,
    begin: usize,
    length: usize,
}

pub struct PiecePayload {
    index: usize,
    begin: usize,
    piece: Vec<u8>,
}
