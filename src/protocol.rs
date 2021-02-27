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
