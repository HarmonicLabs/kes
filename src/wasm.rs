use crate::kes::Sum6Kes;
use crate::traits::{KesSig, KesSk};
use crate::common::PublicKey;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn sign(skey: &mut [u8], msg: &[u8]) -> &[u8] {
    Sum6Kes::from_bytes(skey).unwrap().sign(msg).as_bytes()
}

#[wasm_bindgen]
pub fn verify(skey: &mut [u8], period: u32, pk: &[u8], msg: &[u8]) -> bool {
    Sum6Kes::from_bytes(skey).unwrap().verifY(period, PublicKey::from_bytes(pk), msg).is_ok()
}
