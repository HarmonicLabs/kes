use crate::kes::{Sum6Kes, Sum6KesSig};
use crate::traits::{KesSig, KesSk};
use crate::common::PublicKey;

use wasm_bindgen::prelude::*;

/// cool struct
#[wasm_bindgen]
pub struct WasmSignature([u8; Sum6KesSig::SIZE]);

impl WasmSignature {
    /// cool function
    pub fn to_bytes(&self) -> [u8; Sum6KesSig::SIZE] {
        self.0.clone()
    }
    /// cool function
    pub fn new(bytes: &[u8]) -> Self {
        let mut arr = [0u8; Sum6KesSig::SIZE];
        arr.copy_from_slice(bytes);
        WasmSignature(arr)
    }
}

/// cool function
#[wasm_bindgen]
pub fn sign(skey: &mut [u8], msg: &[u8]) -> WasmSignature {
    WasmSignature(
        Sum6Kes::from_bytes(skey).unwrap().sign(msg).to_bytes()
    )
}

/// cool function
#[wasm_bindgen]
pub fn verify(sig: &mut [u8], period: u32, pk: &[u8], msg: &[u8]) -> bool {
    Sum6KesSig::from_bytes(sig).unwrap().verify(period, &PublicKey::from_bytes(pk).unwrap(), msg).is_ok()
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    Ok(())
}
