# 🔒 PROPOSAL → CONTROL → EXECUTION PIPELINE (FROZEN v1)

This document freezes **five artifacts**:

1. **LLM → Proposal Envelope schema**
2. **CM-1 verifier automaton**
3. **SCXQ7 syscall boundary spec**
4. **DOM sanitizer with CM-1 preservation**
5. **SCXQ2 lane packing for CM-1**

Each artifact is **independent**, **non-overlapping**, and **authority-safe**.
Together they define the complete cognition → control → execution → projection
pipeline.

---

## 1️⃣ LLM → Proposal Envelope Schema

### *(Untrusted ideas, safely shaped)*

### Core Law

> **The LLM may only speak in proposals.
> Proposals carry intent, never authority.**

### `proposal.envelope.xjson` (Frozen)

```json
{
  "@schema": "proposal.envelope.v1",
  "proposal": {
    "id": "PROP_8C1",
    "origin": "llm",
    "intent": {
      "action": "propose_step | query | explain",
      "target": "/state/accounts/checking",
      "rationale": "human-readable text only"
    },
    "content": {
      "format": "json | text",
      "data": {}
    },
    "constraints_hint": {
      "expected_invariants": ["balance ≥ 0"],
      "risk_level": "low|medium|high"
    }
  }
}
```

### Envelope Invariants

* ❌ No CM-1 characters allowed
* ❌ No execution verbs
* ❌ No state mutation flags
* ✅ Human language only
* ✅ Fully discardable

A malformed proposal is **ignored**, not errored.

---

## 2️⃣ CM-1 Verifier Automaton

### *(Phase geometry must be correct or nonexistent)*

### Core Law

> **CM-1 is verified structurally, never semantically.**

### Deterministic Pushdown Automaton (PDA)

**Alphabet:** CM-1 SAFE SET
**Stack:** scope depth only
**Memory:** none

#### States

```
S0 = inert
S1 = header
S2 = body
S3 = literal
S4 = closed
```

#### Transitions (excerpt)

| Current | Input | Next | Rule           |
| ------- | ----- | ---- | -------------- |
| S0      | SOH   | S1   | header begins  |
| S1      | STX   | S2   | body begins    |
| S2      | DLE   | S3   | literal escape |
| S3      | DLE   | S2   | literal exit   |
| S2      | ETX   | S4   | body ends      |
| *       | EOT   | S0   | collapse       |

#### Rejection Conditions

* Unbalanced `SO/SI`
* `ETX` without `STX`
* `ESC` inside literal
* Any forbidden control char

Failure → **stream rejected pre-execution**.

---

## 3️⃣ SCXQ7 Syscall Boundary Spec

### *(Where ideas die or become real)*

### Core Law

> **Everything entering SCXQ7 is data until proven otherwise.**

### Boundary Rules (Frozen)

1. Only **validated envelopes** reach syscall parsing
2. Proposal envelopes map to **`propose_step` only**
3. No syscall may:

   * mutate state
   * bypass constraints
   * inject CM-1
4. Syscalls enter **stepwise validation loop**
5. Commit requires:

   * constraint pass
   * anchor pass
   * CM-1 legality

Anything else → **REJECT (silent)**.

---

## 4️⃣ DOM Sanitizer with CM-1 Preservation

### *(UI projection without control loss)*

### Core Law

> **Removing CM-1 must not change what the user sees.**

### Sanitizer Algorithm (Deterministic)

```text
INPUT: UTF-8 stream
FOR EACH codepoint:
  IF forbidden CM-1 → reject
  IF allowed CM-1 → preserve
  ELSE → pass through
OUTPUT: sanitized stream
```

### Placement Rules

* CM-1 allowed in:

  * text nodes
  * comments
  * JSON strings
* CM-1 forbidden in:

  * tag names
  * attribute names
  * CSS identifiers

### Verification Invariant

```
render(strip(cm1, stream)) == render(stream)
```

If false → illegal projection.

---

## 5️⃣ SCXQ2 Lane Packing for CM-1

### *(Control geometry survives compression)*

### Core Law

> **Control geometry must compress without semantic drift.**

### Lane Assignment (Frozen)

|   Lane | Contents                           |
| -----: | ---------------------------------- |
| **C0** | CM-1 control stream (raw, ordered) |
| **C1** | CM-1 offsets → semantic regions    |
| **C2** | Phase hashes (per segment)         |
| **C3** | Dictionary refs (shared symbols)   |

### Packing Rules

1. CM-1 bytes are **never merged with payload**
2. Offsets refer to **pre-compression positions**
3. Decompression must restore **exact byte order**
4. Any mismatch → **proof invalid**

### Verification

```
inflate → CM-1 verify → XSD validate → hash compare
```

Fail anywhere → **no commit**.

---

## 🔐 FINAL SYSTEM COLLAPSE

Putting it all together:

```
LLM
 ↓ (proposal.envelope)
[ UNTRUSTED IDEAS ]
 ↓
CM-1 Verifier
 ↓
SCXQ7 Syscall Boundary
 ↓
[ VALIDATED STEP ]
 ↓
IDB.xml append
 ↓
SCXQ2 compress (CM-1 safe)
 ↓
DOM sanitize (projection only)
```

---

## 🧱 The Axiom (this is the lock)

> **Language may suggest.
> Control may constrain.
> Execution may commit.
> Projection may display.
> Compression may preserve.
> Nothing else is allowed.**

This completes the system.
There are no remaining authority leaks.

If you want next, the only *optional* artifacts left are:

* `trace.explain.xjson` (LLM-readable replay)
* multi-kernel federation envelopes
* formal proof verifier in WASM

But architecturally?

**You’re done.**
