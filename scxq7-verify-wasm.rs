// scxq7-verify-wasm.rs
// SCXQ7 Verifier — WASM Projection
// Authority: NONE (projection-only)

#![no_std]
extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use sha2::{Digest, Sha256};

const MAGIC: [u8; 8] = [0x53, 0x43, 0x58, 0x51, 0x37, 0x00, 0xAA, 0x55];

const KERNEL_HASH: &str =
    "7f9d8c6e4b1a2e7c9a43d2f6c5e89b1a4f7c0e6d2b9a4c1f8e3a6b9d0f12c4e";

#[no_mangle]
pub extern "C" fn verify(ptr: *const u8, len: usize) -> i32 {
    let bytes = unsafe { core::slice::from_raw_parts(ptr, len) };

    if len < 256 {
        return 1;
    }
    if bytes[0..8] != MAGIC {
        return 2;
    }

    let kernel = &bytes[64..256];
    let mut hasher = Sha256::new();
    hasher.update(kernel);
    let hash = hasher.finalize();

    if format!("{:x}", hash) != KERNEL_HASH {
        return 3;
    }

    let ctx = u32::from_le_bytes([bytes[60], bytes[61], bytes[62], bytes[63]]);
    if ctx == 0 {
        return 4;
    }

    let pm = u32::from_le_bytes([bytes[56], bytes[57], bytes[58], bytes[59]]);
    if pm == 0 {
        return 5;
    }

    0
}
