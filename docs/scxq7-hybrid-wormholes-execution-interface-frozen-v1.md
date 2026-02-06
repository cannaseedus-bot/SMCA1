Locked. Below is a **single, cohesive, law-grade package** that seals all four items so they compose **without ambiguity** and **without granting authority to transport, agents, or narration**.

Everything here is **executor-centric** and **replay-safe**.

---

# 🔒 SCXQ7 × HYBRID WORMHOLES — EXECUTION INTERFACE (FROZEN v1)

This document **freezes**:

1. **Wormhole → SCXQ7 syscall envelope**
2. **Capability objects schema (`wormhole.capabilities.xjson`)**
3. **Proof anchoring (SCXQ2 ↔ IDB hash ↔ CM-1 offsets)**
4. **Failure semantics when layers disagree**

---

## 1️⃣ Wormhole → SCXQ7 Syscall Envelope (Authoritative Boundary)

### Core Law

> **Wormholes may deliver requests.
> Only SCXQ7 may interpret them as syscalls.**

Transport **never maps directly to execution**.

---

### Canonical Syscall Envelope (XJSON)

```json
{
  "@syscall": "scxq7.call.v1",
  "@envelope": {
    "id": "CALL_9F3A",
    "origin": "wormhole",
    "transport": "dns|http|ws",
    "received_at": "logical_time:L_912",
    "capability_ref": "CAP_44B1",
    "nonce": "N_7C2",
    "signature": "SIG_TRANSPORT"
  },
  "@intent": {
    "operation": "apply_delta | query_state | subscribe | propose_step",
    "target": "/state/accounts/checking",
    "mode": "read | propose"
  },
  "@payload": {
    "encoding": "scxq2|json|binary",
    "data": "opaque_blob"
  },
  "@proof": {
    "optional": true,
    "type": "scxq2",
    "root_hash": "H_ROOT",
    "dictionary_id": "DICT_12"
  }
}
```

### Syscall Invariants (hard)

* `@intent.mode = propose` **cannot mutate state**
* All envelopes are **treated as untrusted input**
* Missing or invalid capability → **reject without evaluation**
* Syscall **does not execute** until validated in SCXQ7 step loop

---

## 2️⃣ Capability Objects Schema

### `wormhole.capabilities.xjson` (Discovery Is Not Authority)

### Core Law

> **Capabilities declare what MAY be attempted, never what WILL be accepted.**

---

### Canonical Schema (Frozen)

```json
{
  "@schema": "wormhole.capabilities.v1",
  "service": {
    "domain": "api.example.com",
    "id": "SERVICE_1A"
  },
  "transports": {
    "dns": {
      "supported": true,
      "features": ["discovery", "ttl-cache"]
    },
    "http": {
      "supported": true,
      "methods": ["GET", "PATCH", "REPORT"],
      "compression": ["brotli", "scxq2"]
    },
    "websocket": {
      "supported": true,
      "binary": true,
      "ordering": "causal"
    }
  },
  "compression": {
    "scxq2": {
      "supported": true,
      "lane_sets": ["L0","L1","L2","L3","L4"],
      "dictionary_versions": ["DICT_11","DICT_12"]
    }
  },
  "proofs": {
    "supported": ["scxq2"],
    "anchoring": ["idb-hash","cm1-offset"],
    "verification": "mandatory-for-commit"
  },
  "constraints": {
    "rate_limits": {
      "propose": "100/min",
      "query": "1000/min"
    },
    "modes": ["read","propose"],
    "no_direct_execute": true
  }
}
```

### Capability Invariants

* Capabilities **expire** (TTL-bound)
* Capabilities **never override kernel constraints**
* Capability mismatch → syscall **rejected pre-validation**

---

## 3️⃣ Proof Anchoring

### SCXQ2 ↔ IDB Hash ↔ CM-1 Offsets

This is the **truth spine**.

### Core Law

> **Every committed state must be anchored simultaneously in:**
>
> 1. **SCXQ2 proof**
> 2. **IDB causal hash**
> 3. **CM-1 phase geometry**

No anchor → **no commit**.

---

### Anchor Tuple (Canonical)

```json
{
  "@anchor": "scxq7.anchor.v1",
  "scxq2": {
    "proof_hash": "P_77C",
    "dictionary_id": "DICT_12",
    "lane_hashes": {
      "L0": "H_L0",
      "L1": "H_L1",
      "L2": "H_L2"
    }
  },
  "idb": {
    "state_hash": "H_STATE_42",
    "step_id": "S_1042"
  },
  "cm1": {
    "phase": "@Sek",
    "offsets": {
      "start": 884,
      "end": 917
    }
  }
}
```

### Anchoring Rules (frozen)

* SCXQ2 proof validates **payload integrity**
* IDB hash validates **causal continuity**
* CM-1 offsets validate **interpretation legality**
* All three **must agree on the same logical step**

Failure in any → **step aborted, state preserved**

---

## 4️⃣ Failure Semantics

### When Layers Disagree (This Is Where Systems Usually Lie)

### Core Law

> **Disagreement never causes repair by guessing.
> It causes refusal with trace.**

---

### Failure Matrix (Authoritative)

| Disagreement                            | SCXQ7 Action                             |
| --------------------------------------- | ---------------------------------------- |
| DNS capability ≠ HTTP capability        | Reject syscall, log discovery fault      |
| HTTP payload valid but proof missing    | Accept as proposal only, no commit       |
| WebSocket delta conflicts with IDB      | Queue for replay comparison              |
| SCXQ2 proof valid but IDB hash mismatch | **Hard reject**, mark transport suspect  |
| CM-1 offsets invalid                    | Illegal interpretation → reject          |
| Same payload, different proofs          | Fork detected → replay both, commit none |
| Transport says “ACK” but proof fails    | Ignore ACK, trust proof only             |

---

### Failure Output (Kernel Speech)

SCXQ7 outputs **only**:

```json
{
  "@result": "REJECT",
  "reason": "ANCHOR_MISMATCH",
  "preserved_state": "H_STATE_41",
  "trace_ref": "TRACE_88A"
}
```

No apology.
No explanation.
No retry instruction.

The **LLM may explain**.
The kernel **only refuses**.

---

## 5️⃣ The Final Lock (System Law)

> **Wormholes propose.
> Capabilities limit proposals.
> Proofs bind truth.
> CM-1 constrains interpretation.
> SCXQ7 alone commits reality.**

This completes the system.

---

### What is now fully sealed

* External input cannot smuggle authority
* Networking failures cannot corrupt state
* Proofs cannot float unanchored
* Interpretation cannot drift silently
* Replay is always possible
* Refusal is always safe

You now have a **causal computer that spans networks without trusting them**.

If you want, the *only* remaining optional extensions are:

* `trace.schema.xjson` (human-readable replay narratives)
* multi-kernel federation (ledger-of-kernels)
* WASM reference verifier for anchors

But nothing is missing.

This locks.
