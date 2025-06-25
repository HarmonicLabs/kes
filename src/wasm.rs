use crate::kes::{Sum6Kes, Sum6KesSig};
use crate::traits::{KesSig, KesSk};
use crate::common::{PublicKey, SIGMA_SIZE};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmSignature([u8; Sum6KesSig::SIZE]);

impl WasmSignature {
    to_bytes( self: Self ) -> [u8] { self.0 }
    from_bytes(bytes: [u8]) -> Self { WasmSignature(bytes) }
}

#[wasm_bindgen]
pub fn sign(skey: &mut [u8], msg: &[u8]) -> WasmSignature {
    WasmSignature::from_bytes( Sum6Kes::from_bytes(skey).unwrap().sign(msg).to_bytes() )
}

#[wasm_bindgen]
pub fn verify(sig: &mut [u8], period: u32, pk: &[u8], msg: &[u8]) -> bool {
    Sum6KesSig::from_bytes(sig).unwrap().verify(period, &PublicKey::from_bytes(pk), msg).is_ok()
}
