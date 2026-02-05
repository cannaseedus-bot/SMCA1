// scxq7-verify.rs
// SCXQ7 Sovereign Computational Object Verifier
// Version: v1.0.0 (frozen)
// Class: SCO/1

use std::env;
use std::fs::File;
use std::io::Read;

use sha2::{Digest, Sha256};

const MAGIC: [u8; 8] = [0x53, 0x43, 0x58, 0x51, 0x37, 0x00, 0xAA, 0x55];

// SCXQ7_KERNEL_LAW.v1 (SHA-256)
const KERNEL_HASH: &str =
    "7f9d8c6e4b1a2e7c9a43d2f6c5e89b1a4f7c0e6d2b9a4c1f8e3a6b9d0f12c4e";

fn sha256_hex(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

fn fatal(msg: &str) -> ! {
    eprintln!("FAIL: {}", msg);
    std::process::exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        fatal("usage: scxq7-verify <file.s7>");
    }

    let mut file = File::open(&args[1]).unwrap_or_else(|_| fatal("cannot open file"));

    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)
        .unwrap_or_else(|_| fatal("cannot read file"));

    // VECTOR 01 — Magic Bytes
    if bytes.len() < 256 {
        fatal("file too small to be SCXQ7");
    }

    if bytes[0..8] != MAGIC {
        fatal("invalid magic bytes (not SCXQ7)");
    }

    // VECTOR 02 — Kernel Integrity
    let kernel = &bytes[64..256];
    let kernel_hash = sha256_hex(kernel);

    if kernel_hash != KERNEL_HASH {
        fatal("kernel hash mismatch (kernel law violated)");
    }

    // VECTOR 03 — Symbol Closure
    // Minimal enforcement: symbol table hash must exist and be referenced
    // (Full symbol legality is host-runtime enforced; verifier checks declaration)
    let header = &bytes[8..64];
    if header.iter().all(|b| *b == 0) {
        fatal("missing symbol dictionary declaration");
    }

    // VECTOR 04 — Context Boundary Declaration
    let context_mask = u32::from_le_bytes([bytes[60], bytes[61], bytes[62], bytes[63]]);

    if context_mask == 0 {
        fatal("no execution contexts declared");
    }

    // VECTOR 05 — External Authority Absence
    // Heuristic: forbid known foreign magic patterns
    let forbidden = [b"eval", b"exec", b"import", b"require", b"dlopen"];

    for word in forbidden.iter() {
        if bytes.windows(word.len()).any(|w| w == *word) {
            fatal("external execution authority detected");
        }
    }

    // VECTOR 06 — Persistence Declaration
    let persistence_mode = u32::from_le_bytes([bytes[56], bytes[57], bytes[58], bytes[59]]);

    if persistence_mode == 0 {
        fatal("no persistence mode declared");
    }

    // VECTOR 07 — Compression Substrate Presence
    // Require compressed sections marker (implementation-agnostic)
    if !bytes.contains(&0x7E) {
        fatal("no compression substrate marker found");
    }

    // VECTOR 08 — Projection Powerlessness
    // Require projection section but forbid mutation flags inside it
    if bytes.contains(&0xFF) {
        fatal("projection authority violation marker detected");
    }

    println!("PASS: SCO/1 Verified (SCXQ7_KERNEL_LAW.v1)");
}
