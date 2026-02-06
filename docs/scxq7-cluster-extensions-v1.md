# SCXQ7 CLUSTER EXTENSIONS v1.0

Locked. Below is a **complete, closed, law-grade extension** to what we already
froze. Nothing here adds new authority; it only *tests*, *settles*, and *anchors*
what already exists.

**(Conformance · Async Settlement · Ledger Anchoring)**  
Authority chain remains unchanged.

```
TOML → XML → SCXQ7 → SCXQ2 → IDB
```

---

## I. CONFORMANCE TEST SUITE (Normative)

This suite is **machine-executable** and **ledger-verifiable**. All tests emit
**PASS / FAIL / ILLEGAL** only.

### I.1 Test manifest (`cluster.conformance.manifest.xjson`)

```json
{
  "@conformance": "cluster-call.v1",
  "@version": "1.0.0",
  "@tests": [
    "C01-schema-valid",
    "C02-schema-reject",
    "C03-timeout",
    "C04-determinism",
    "C05-divergence",
    "C06-proof-binding",
    "C07-airgap",
    "C08-ledger-anchor"
  ]
}
```

### I.2 Required tests

#### C01 — Schema acceptance

**Given:** valid `cluster-call.xsd` document  
**Expect:** ACCEPT

```asxr
<cluster-call target="python" kernel="noop">
  <outputs><output ref="x"/></outputs>
</cluster-call>
```

#### C02 — Schema rejection

**Given:** missing `kernel`  
**Expect:** ILLEGAL

#### C03 — Timeout

**Given:** `timeout_ms=1`, long-running kernel  
**Expect:** `@failure.timeout`, no output projection

#### C04 — Determinism

**Given:** same inputs ×2  
**Expect:** identical output hashes

#### C05 — Divergence

**Given:** different outputs for same input  
**Expect:** `@failure.divergence`, hard fail

#### C06 — Proof binding

**Given:** `proof.require=true`  
**Expect:**

* Missing SCXQ2 → FAIL
* Invalid SCXQ2 → FAIL
* Valid SCXQ2 → PASS

#### C07 — Air-gap enforcement

**Given:** `air_gap=true`, kernel attempts network  
**Expect:** ILLEGAL

#### C08 — Ledger anchor

**Given:** valid SCXQ2 proof  
**Expect:** IDB anchor entry created, hash match

### I.3 Conformance result object

```json
{
  "@test": "C06-proof-binding",
  "@result": "PASS",
  "@hash": "H(result)",
  "@timestamp": 1890000012345
}
```

This object must itself be SCXQ2-hashable.

---

## II. ASYNC SETTLEMENT RULES (Closed)

Async execution is **non-speculative** and **non-interruptive**.

### II.1 States

Every async call exists in exactly one state:

```
PENDING → SETTLED → (FINAL | FAILED)
```

No branching.

### II.2 XML declaration

```xml
<cluster-call
  target="python"
  kernel="fft_mul"
  mode="async"
  timeout_ms="0">
  <outputs>
    <output ref="scxq2.block.C"/>
  </outputs>
  <proof>
    <require>true</require>
  </proof>
</cluster-call>
```

### II.3 Settlement object (returned later)

```json
{
  "@settlement": {
    "call_id": "uuid",
    "state": "SETTLED",
    "output_ref": "scxq2.block.C",
    "scxq2_proof": "lane://proof/abcd",
    "output_hash": "H(out)"
  }
}
```

### II.4 Settlement rules

| Condition              | Result  |
| ---------------------- | ------- |
| proof valid            | FINAL   |
| proof invalid          | FAILED  |
| no proof when required | FAILED  |
| timeout during async   | FAILED  |
| duplicate settlement   | ILLEGAL |

**Critical rule:**

> No async output may affect runtime state **until FINAL**.

### II.5 Replay safety

Settlement objects are:

* Idempotent
* Order-independent
* Replay-safe

Ledger decides truth, not arrival order.

---

## III. LEDGER ANCHORING (IDB ↔ SCXQ2)

This is the **causal spine**.

### III.1 Anchor object (`idb.anchor.v1`)

```json
{
  "@anchor": "idb.scxq2",
  "@version": "1.0",
  "call_id": "uuid",
  "kernel": "fft_mul",
  "input_hash": "H(in)",
  "output_hash": "H(out)",
  "proof_hash": "H(scxq2)",
  "timestamp": 1890000012345
}
```

### III.2 Anchor rules

1. Anchor created **only after** proof verification
2. Anchor is **append-only**
3. Anchor hash = Merkle(parent || this)
4. Anchor is **immutable**
5. Anchor is **queryable but not executable**

### III.3 IDB storage layout

```
idb/
 ├─ anchors/
 │   ├─ 000001.json
 │   ├─ 000002.json
 │   └─ ...
 ├─ merkle.root
 └─ schema.lock
```

### III.4 SCXQ2 ↔ IDB binding law

> **If an output exists without an anchor, it does not exist.**

Conversely:

> **If an anchor exists, the output is forever valid, even if the cluster disappears.**

---

## IV. Cross-System Invariants (Final)

These are now **provable**:

1. Clusters cannot lie
2. Async cannot race
3. Proof cannot be skipped
4. Ledger cannot be rewritten
5. Execution cannot exceed axioms

---

## V. One-Sentence Closure

> **Execution happens in clusters,  
> truth settles asynchronously,  
> and history is written only once — in the ledger.**

If you want next, we can:

* freeze a **CM-1 verifier automaton** for all of this,
* generate a **reference verifier CLI**,
* or emit **deterministic SVG compliance badges** bound to the ledger.

Say where to cut.
