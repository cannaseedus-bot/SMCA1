# WORMHOLE-SPEC v1 — Frozen Wire Formats and Conformance Law

This document locks the **canonical, normative v1** wire formats and conformance rules for Hybrid Wormholes. It is **additive** to prior architecture documents; no contradictions.

---

# 1. Wormhole Capability Schema v1

**(DNS TXT + HTTPS discovery, canonical)**

This schema defines **how a service declares what it can do**.
Discovery is *descriptive*, not imperative.

---

## 1.1 Canonical Capability Object

This is the **authoritative logical schema**, independent of transport:

```json
{
  "schema": "wormhole.capability.v1",
  "service": {
    "name": "string",
    "domain": "fqdn",
    "instance": "uuid"
  },
  "endpoints": {
    "http": "https://host/path",
    "ws": "wss://host/path",
    "proof": "https://host/proof"
  },
  "protocols": {
    "discovery": ["dns", "https"],
    "sync": ["http", "http+delta"],
    "runtime": ["ws", "ws+binary"],
    "proof": ["scxq2"]
  },
  "compression": ["scxq2", "brotli", "lz4"],
  "auth": ["jwt", "oauth2", "mtls", "anonymous"],
  "capabilities": [
    "object-server",
    "pi-adapter-v1",
    "pi-gccp-v1",
    "svg-tensor-index",
    "crdt"
  ],
  "limits": {
    "max_frame_bytes": 16777216,
    "max_state_bytes": 1073741824,
    "rate_per_sec": 1000
  },
  "hash": "sha256:…"
}
```

---

## 1.2 DNS TXT Record Layout (Authoritative)

**Record name**

```
_asxr._tcp.<domain>
```

**TXT payload (key=value, semicolon-delimited)**

```
v=1;
http=https://api.example.com/sync;
ws=wss://api.example.com/runtime;
proof=https://api.example.com/proof;
proto=ws,http,scxq2;
compress=scxq2,brotli;
auth=jwt;
caps=object-server,pi-adapter-v1,pi-gccp-v1;
hash=sha256:abcd…
```

### Rules

* TXT record MUST fit in DNS limits → **summary only**
* `hash` MUST correspond to full HTTPS descriptor
* DNSSEC REQUIRED for trust

---

## 1.3 HTTPS Capability Descriptor

**URL**

```
https://<domain>/.well-known/wormhole-capabilities.json
```

**Rules**

* JSON MUST exactly match the canonical schema
* `hash` MUST equal SHA-256 of canonicalized JSON
* HTTPS cert MUST match DNS name (DANE allowed)

---

## 1.4 Capability Negotiation Rule (LOCKED)

```text
SelectedCapabilities =
  ClientCapabilities ∩ ServerCapabilities
```

No defaults.
No assumptions.
No silent downgrade.

---

# 2. SCXQ2 Proof Envelope v1

**(What π / Object Server expect, exactly)**

This envelope travels **out-of-band** or **post-hoc**.
It never blocks runtime.

---

## 2.1 Canonical Proof Envelope

```json
{
  "schema": "scxq2.proof.v1",
  "subject": {
    "object": "object://state/path",
    "version": "monotonic-id",
    "byte_length": 12345678
  },
  "compression": {
    "algorithm": "scxq2",
    "dictionary_id": "sha256:…",
    "ratio": 87.4
  },
  "integrity": {
    "root_hash": "sha256:…",
    "chunk_hashes": ["sha256:…", "sha256:…"]
  },
  "proof": {
    "type": "zk-stark",
    "curve": "bls12-381",
    "proof_bytes": "base64…"
  },
  "signatures": {
    "server": "base64…",
    "algorithm": "sphincs+"
  },
  "timestamp": "iso-8601"
}
```

---

## 2.2 Verification Contract (π / Object Server)

π and Object Server MUST perform:

1. Verify signature
2. Verify proof against `root_hash`
3. Decompress payload
4. Hash decompressed payload
5. Compare hash == `root_hash`

If any step fails:

* **mark state invalid**
* **do not halt runtime**
* **trigger resync**

---

## 2.3 Proof Non-Blocking Rule (LOCKED)

```text
delivery ≠ acceptance
```

Runtime may proceed.
Proof governs **trust**, not **transport**.

---

# 3. Hybrid Wormhole Conformance Tests v1

These tests define **what makes an implementation valid**.

---

## 3.1 Discovery Conformance

**Test: DNS → HTTPS parity**

* TXT hash MUST match HTTPS descriptor hash
* Missing HTTPS descriptor = FAIL
* Hash mismatch = FAIL

---

## 3.2 Negotiation Conformance

**Test: Capability intersection**

* Server advertises `{A,B,C}`
* Client advertises `{B,C,D}`
* Selected MUST be `{B,C}` only

Implicit fallback = FAIL

---

## 3.3 Runtime Conformance

**Test: Runtime Ephemerality**

* Kill WebSocket
* Client MUST recover via HTTP sync
* No data loss allowed

---

## 3.4 Proof Conformance

**Test: Corruption Detection**

* Flip one bit in compressed payload
* Proof verification MUST fail
* Runtime MUST continue
* State MUST be invalidated

---

## 3.5 π Integration Conformance

**Test: Geometry Determinism**

* Same SVG-Tensor + same π-Profile
* MUST collapse to identical score
* CPU and GPU paths MUST match within ε

---

## 3.6 SCXQ2 Independence Test

* Disable SCXQ2
* System MUST still function (larger payloads)
* Proof layer is optional, never required

---

# 4. Minimal Reference Client

**(One real client proving the stack works)**

This is not a demo — it’s a **compliance anchor**.

---

## 4.1 Reference Client Responsibilities

The client MUST:

1. Perform DNS discovery
2. Fetch HTTPS capability descriptor
3. Negotiate protocol
4. Sync initial state via HTTP
5. Establish WebSocket runtime
6. Receive geometry deltas
7. Collapse via π-GCCP
8. Verify SCXQ2 proof asynchronously

---

## 4.2 Minimal Control Flow

```text
discover()
  ↓

negotiate()
  ↓

http_sync(full)
  ↓

ws_connect()
  ↓

on_delta → π_collapse()
  ↓

on_proof → verify_or_invalidate()
```

---

## 4.3 Minimal Client Pseudocode

```ts
const caps = await discoverDNS(domain);
const desc = await fetch(descUrl);

const negotiated = intersect(clientCaps, desc.capabilities);

await httpSync(desc.endpoints.http);

const ws = connect(desc.endpoints.ws);

ws.on("delta", d => {
  const geometry = decode(d);
  piCollapse(geometry);
});

ws.on("proof", p => {
  verifySCXQ2(p);
});
```

No framework assumptions.
No model assumptions.
No UI assumptions.

---

## 5. Final Lock

You now have:

* **Capability discovery law**
* **Proof envelope law**
* **Conformance criteria**
* **Executable reference path**

This means:

* Implementations can vary
* Behavior cannot

If you want next, we can:

* Freeze this as **WORMHOLE-SPEC v1**
* Emit **JSON Schemas** for all envelopes
* Produce a **single-file reference client** (Node, Browser, or PowerShell)
* Or bind this directly to **object:// semantics**
