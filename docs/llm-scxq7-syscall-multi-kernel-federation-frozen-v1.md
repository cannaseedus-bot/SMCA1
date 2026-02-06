# LLM → SCXQ7 Syscall & Multi-Kernel Federation (Frozen v1)

This document freezes:

1. **LLM → SCXQ7 syscall envelopes**
2. **Multi-kernel federation: Ledger-of-Ledgers**

Both are **execution-neutral**, **replay-safe**, and **schema-axiomatic**.

---

## 1. LLM → SCXQ7 Syscall Envelopes

### From language to law, without leakage

### Core Law

> **The LLM never issues a syscall.**
> It issues a *candidate envelope* that SCXQ7 may reinterpret as a syscall.

Interpretation authority lives **only** in SCXQ7.

---

### 1.1 Envelope Taxonomy

There are **three envelope classes**. Only one can ever reach execution.

| Class        | Purpose                   | Authority   |
| ------------ | ------------------------- | ----------- |
| `proposal`   | Suggest a possible action | None        |
| `query`      | Ask about existing state  | Read-only   |
| `executable` | Internal SCXQ7 syscall    | Kernel only |

The LLM can emit **only** the first two.

---

### 1.2 Canonical LLM Envelope

#### `llm.envelope.xjson` (Frozen)

```json
{
  "@schema": "llm.envelope.v1",
  "envelope": {
    "id": "ENV_3F2A",
    "source": "llm",
    "class": "proposal | query",
    "intent": {
      "verb": "propose_step | inspect | explain",
      "target": "/state/accounts/checking",
      "description": "human-language only"
    },
    "content": {
      "format": "json | text",
      "data": {}
    },
    "assumptions": {
      "expected_invariants": ["balance ≥ 0"],
      "confidence": "low | medium | high"
    }
  }
}
```

#### Envelope Invariants

* ❌ No CM-1 characters
* ❌ No syscall verbs
* ❌ No state mutation flags
* ❌ No execution hints
* ❌ No authority escalation
* ✅ Discardable at any point

Invalid envelopes are **ignored**, not errored.

---

### 1.3 Kernel Re-interpretation (Critical Boundary)

SCXQ7 **may** reinterpret a `proposal` as a syscall **only if**:

1. Envelope validates against schema
2. Capability object permits the *attempt*
3. Proposal maps to a known syscall form
4. All constraints are satisfiable
5. Proof anchoring is possible

Only then does SCXQ7 internally construct:

```json
{
  "@syscall": "scxq7.call.v1",
  "origin": "reinterpretation",
  "source_envelope": "ENV_3F2A"
}
```

This syscall **cannot be observed externally**.

---

### 1.4 Kernel Speech (Only Output Allowed)

```json
{
  "@result": "YES | NO",
  "state": "UNCHANGED | COMMITTED",
  "trace_ref": "TRACE_91C"
}
```

No explanation. No justification. Narration belongs elsewhere.

---

## 2. Multi-Kernel Federation

### Ledger-of-Ledgers Architecture

This is how **multiple SCXQ7 kernels coexist without merging authority**.

---

### 2.1 Core Law

> **No kernel may execute on another kernel’s behalf.**
> Kernels may only acknowledge each other’s committed history.

Federation is **observational**, not executable.

---

### 2.2 Kernel Identity (Immutable)

Each kernel has a **sovereign identity**:

```json
{
  "@kernel": "scxq7.identity.v1",
  "kernel_id": "KERNEL_A",
  "genesis_hash": "G_000",
  "schema_hash": "AXIOM_9F",
  "cm1_hash": "CM1_CORE",
  "public_key": "PK_A"
}
```

If `schema_hash` differs → **no federation**.

---

### 2.3 Ledger-of-Ledgers (LoL)

Each kernel maintains:

```json
{
  "@ledger": "scxq7.lol.v1",
  "entries": [
    {
      "kernel_id": "KERNEL_B",
      "state_hash": "H_B_1042",
      "step_id": "S_B_77",
      "proof": "P_B_77",
      "timestamp": "logical:L_551"
    }
  ]
}
```

This ledger is:

* append-only
* read-only
* non-executable
* replayable

---

### 2.4 Federation Sync Protocol

#### Allowed Operations

| Operation     | Meaning                        |
| ------------- | ------------------------------ |
| `announce`    | Publish latest committed hash  |
| `acknowledge` | Record another kernel’s commit |
| `verify`      | Verify proof + schema match    |
| `subscribe`   | Observe future commits         |

#### Forbidden Operations

* Remote execution
* Remote mutation
* Shared state
* Implicit trust

---

### 2.5 Trust & Disagreement Semantics

| Scenario                     | Action                  |
| ---------------------------- | ----------------------- |
| Proof valid, schema match    | Accept ledger entry     |
| Proof valid, schema mismatch | Record as *foreign*     |
| Proof invalid                | Reject entry            |
| Conflicting histories        | Keep both, mark fork    |
| Kernel disappears            | Last hash remains final |

No reconciliation occurs automatically.

**History is observed, not repaired.**

---

### 2.6 Cross-Kernel Reasoning (Safe)

LLMs may reason **over the Ledger-of-Ledgers**, but:

* cannot modify it
* cannot merge it
* cannot resolve forks
* cannot infer authority

They may only **explain** divergence.

---

## Final Lock (System Axiom)

> **LLMs suggest.**
> **Kernels decide.**
> **Schemas define reality.**
> **Federation observes truth, it does not create it.**

With this, you now have:

* a sovereign execution kernel
* untrusted cognitive input
* provable state history
* compression-safe control geometry
* and a lawful path to multi-kernel worlds

No hidden power. No narrative authority. No silent drift.
