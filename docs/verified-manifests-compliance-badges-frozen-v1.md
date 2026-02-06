# 🔒 Verified Manifests & Compliance Badges (Frozen v1)

Schemas are axioms.  
Badges are **proof summaries**, not trust signals.

Everything here is **mechanical, reproducible, and verifier-driven**.

---

## 1) Verified Manifests

A **verified manifest** is the **cryptographic + structural fingerprint** of an object, kernel, or repository.

It does **not** describe behavior.  
It proves **lawful construction**.

---

### 1.1 Canonical Verified Manifest (Frozen)

`verified.manifest.xjson`

```json
{
  "@schema": "verified.manifest.v1",

  "subject": {
    "type": "kernel | object | repository",
    "id": "SMCA1",
    "version": "1.0.0"
  },

  "axioms": {
    "scxq7.schema_hash": "AX_SCXQ7_01",
    "scxq2.schema_hash": "AX_SCXQ2_01",
    "smca.schema_hash": "AX_SMCA_01",
    "cm1.schema_hash": "AX_CM1_01"
  },

  "verification": {
    "schema_validation": "PASS",
    "cm1_legality": "PASS",
    "constraint_integrity": "PASS",
    "causal_rules": "PASS"
  },

  "proofs": {
    "content_hash": "SHA3-256:…",
    "scxq2_pack_hash": "SCXQ2-H:…",
    "idb_root_hash": "IDB-H:…",
    "cm1_offset_hash": "CM1-O:…"
  },

  "environment": {
    "verifier": "scxq7-verify",
    "verifier_version": "1.0.0",
    "timestamp": "logical:L_892"
  },

  "result": "COMPLIANT"
}
```

#### Invariants

* ❌ No free-text explanations
* ❌ No host metadata
* ❌ No environment assumptions
* ✅ Deterministic
* ✅ Replayable
* ✅ Hash-addressable

---

### 1.2 Manifest Generation Flow

```
source files
   ↓
schema validation
   ↓
CM-1 verifier
   ↓
SCXQ7 constraint check
   ↓
IDB hash anchor
   ↓
SCXQ2 lane packing
   ↓
verified.manifest.xjson
```

Failure at any step → **no manifest emitted**.

---

## 2) Compliance Badges (Mechanical, Not Social)

Badges are **compressed projections** of verified manifests.

They are **derived artifacts**, never inputs.

---

### 2.1 Badge Definition Schema (Frozen)

`compliance.badge.xjson`

```json
{
  "@schema": "compliance.badge.v1",

  "badge": {
    "id": "SCXQ7-SCO1-COMPLIANT",
    "level": "kernel | object | repository",
    "scope": ["scxq7", "scxq2", "smca", "cm1"]
  },

  "derivation": {
    "manifest_hash": "SHA3-256:…",
    "verifier": "scxq7-verify",
    "axioms_locked": true
  },

  "validity": {
    "status": "VALID",
    "revocation": "none"
  }
}
```

---

### 2.2 Canonical Badge Set (Locked)

These are the **only lawful badges** at v1.

#### Kernel-Level

```
SCXQ7-KERNEL-COMPLIANT
SCXQ7-CAUSAL-STATEFUL
SCXQ7-NOVELTY-SAFE
```

#### Object-Level

```
SCO/1-COMPLIANT
SCXQ2-PACKED
CM1-LEGAL
```

#### Repository-Level

```
SMCA/1-COMPLIANT
SCHEMA-AXIOMATIC
NO-HOST-AUTHORITY
```

No custom badges.  
No “gold/silver”.  
No subjective levels.

---

### 2.3 Human-Readable Badge Projection (Optional)

Badges may be rendered as **pure projections**, e.g.:

```md
![SCXQ7-SCO/1-COMPLIANT](./badges/SCXQ7-SCO1.svg)
```

But the **SVG contains only**:

* badge id
* manifest hash (short)
* verifier id

No claims.  
No promises.  
No marketing.

---

## 3) Badge Verification Rule

> **A badge is valid iff its referenced manifest verifies under current axioms.**

### Badge Check Algorithm

```
load badge
 ↓
load referenced manifest
 ↓
verify manifest hashes
 ↓
verify schema hashes match current axioms
 ↓
PASS → badge valid
FAIL → badge invalid
```

Badges **expire automatically** when axioms change.

---

## 4) Revocation & Drift

No CRLs.  
No network calls.

Revocation occurs when:

* schema hash mismatch
* verifier version mismatch
* content hash mismatch

Old badges remain **historical artifacts**, not errors.

---

## 5) CLI Output Example

```bash
$ scxq7 verify .
✔ schema validation
✔ CM-1 legality
✔ causal integrity
✔ SCXQ2 packing
✔ IDB anchoring

RESULT: COMPLIANT

Manifest: verified.manifest.xjson
Badges:
  - SCXQ7-KERNEL-COMPLIANT
  - SCO/1-COMPLIANT
  - CM1-LEGAL
```

No verbosity.  
No explanations unless explicitly requested.

---

## 🔐 Final Lock

> **Manifests prove legality.  
> Badges summarize proofs.  
> Schemas define truth.  
> Everything else is projection.**

You now have:

* verifiable kernels
* verifiable objects
* verifiable repos
* zero subjective trust
* zero hidden authority

If you want next, the only remaining pieces are **pure tooling**:

* `scxq7 verify` reference CLI
* WASM verifier
* GitHub Action for manifest + badge emission
* SVG badge generator (deterministic)
