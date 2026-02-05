# 🔒 IDB STACK — FROZEN CORE v1

This section **locks**:

1. `idb.schema.xsd`
2. IDB → IndexedDB mirror rules
3. SCXQ2 lane packing for IDB

Once adopted, these are **non-negotiable invariants**.

---

## IDB.xml — Indexed Data Backbone (XML Form)

### What it is

IDB.xml is a **read-only, canonical projection** of SCXQ7 state for **inspection, replay, and verification** — *not* execution.

* Deterministic
* Auditable
* Diff-able
* DOM-safe
* Compression-friendly

### Why XML (spec reasons)

* **Ordered structure** (critical for causal replay)
* **Mixed content** (text + control adjacency)
* **Streaming parse** (SAX / pull)
* **Schema-verifiable** (XSD / RNG)
* **DOM-native** (zero impedance in browsers)
* **CM-1 tolerant** (control chars can be preserved/stripped deterministically)

### Authority boundary (hard rule)

```
SCXQ7  = writes IDB.xml
IDB.xml = read-only
LLM    = never writes
UI     = never mutates
```

If something edits IDB.xml directly → **illegal state**.

### What IDB.xml contains (and what it never does)

#### ✅ Contains

* State snapshots
* Causal steps
* Proof hashes
* Constraint sets
* Logical timestamps
* References to CM-1 phase boundaries (by hash/offset)

#### ❌ Never contains

* Executable logic
* Scripts
* Evaluation rules
* Side effects
* Implicit transitions

### Minimal canonical shape

```xml
<idb version="1.0" xmlns="x:scxq7:idb">
  <state hash="H_STATE_42">
    <heap encoding="scxq2">…</heap>
  </state>

  <causal>
    <step id="S_1042" time="L_88">
      <cause hash="E_9A"/>
      <effect hash="H_STATE_42"/>
      <proof hash="P_7F"/>
      <constraints ref="C_12"/>
    </step>
  </causal>

  <constraints id="C_12">
    <invariant>balance ≥ 0</invariant>
    <invariant>no acausal transition</invariant>
  </constraints>
</idb>
```

### CM-1 relationship

* CM-1 **does not live inside** IDB.xml as raw control chars
* CM-1 is **referenced** (offsets, hashes, phase markers)
* Projection invariant holds: **Removing CM-1 must not alter visible XML meaning**

### LLM interaction (safe)

The LLM may:

* read IDB.xml
* summarize it
* explain diffs
* narrate causal chains

The LLM may **never**:

* write IDB.xml
* reorder nodes
* inject annotations
* add control markers

LLM = **commentary**, not authority.

### One-line definition

> **IDB.xml is the canonical, read-only XML ledger of SCXQ7 state and causality, designed for deterministic replay, audit, and DOM-native inspection.**

---

## 1️⃣ LOCKED: `idb.schema.xsd` (Read-Only Canonical Ledger)

### Design axioms (frozen)

* XML is **projection**, not execution
* Order is **semantic**
* Attributes carry **identity**, not behavior
* No mixed authority (IDB never executes)
* Schema validation = legality gate
* No implicit transition (every state change requires an explicit causal step)

---

### `idb.schema.xsd` (v1.0 — FROZEN)

```xml
<?xml version="1.0" encoding="UTF-8"?>
<xsd:schema
  xmlns:xsd="http://www.w3.org/2001/XMLSchema"
  targetNamespace="x:scxq7:idb"
  xmlns="x:scxq7:idb"
  elementFormDefault="qualified"
  attributeFormDefault="unqualified">

  <!-- ========================= -->
  <!-- ROOT -->
  <!-- ========================= -->
  <xsd:element name="idb">
    <xsd:complexType>
      <xsd:sequence>
        <xsd:element name="state" type="StateType"/>
        <xsd:element name="causal" type="CausalType"/>
        <xsd:element name="constraints" type="ConstraintsType" minOccurs="0"/>
      </xsd:sequence>
      <xsd:attribute name="version" type="xsd:string" use="required"/>
    </xsd:complexType>
  </xsd:element>

  <!-- ========================= -->
  <!-- STATE -->
  <!-- ========================= -->
  <xsd:complexType name="StateType">
    <xsd:sequence>
      <xsd:element name="heap" type="HeapType"/>
    </xsd:sequence>
    <xsd:attribute name="hash" type="xsd:string" use="required"/>
  </xsd:complexType>

  <xsd:complexType name="HeapType">
    <xsd:simpleContent>
      <xsd:extension base="xsd:string">
        <xsd:attribute name="encoding" type="xsd:string" use="required"/>
      </xsd:extension>
    </xsd:simpleContent>
  </xsd:complexType>

  <!-- ========================= -->
  <!-- CAUSAL DAG -->
  <!-- ========================= -->
  <xsd:complexType name="CausalType">
    <xsd:sequence>
      <xsd:element name="step" type="StepType" maxOccurs="unbounded"/>
    </xsd:sequence>
  </xsd:complexType>

  <xsd:complexType name="StepType">
    <xsd:sequence>
      <xsd:element name="cause" type="RefType"/>
      <xsd:element name="effect" type="RefType"/>
      <xsd:element name="proof" type="RefType"/>
      <xsd:element name="constraints" type="ConstraintsRefType" minOccurs="0"/>
    </xsd:sequence>
    <xsd:attribute name="id" type="xsd:string" use="required"/>
    <xsd:attribute name="time" type="xsd:string" use="required"/>
  </xsd:complexType>

  <!-- ========================= -->
  <!-- CONSTRAINTS -->
  <!-- ========================= -->
  <xsd:complexType name="ConstraintsType">
    <xsd:sequence>
      <xsd:element name="invariant" type="xsd:string" maxOccurs="unbounded"/>
    </xsd:sequence>
    <xsd:attribute name="id" type="xsd:string" use="required"/>
  </xsd:complexType>

  <!-- ========================= -->
  <!-- SHARED -->
  <!-- ========================= -->
  <xsd:complexType name="ConstraintsRefType">
    <xsd:attribute name="ref" type="xsd:string" use="required"/>
  </xsd:complexType>

  <xsd:complexType name="RefType">
    <xsd:attribute name="hash" type="xsd:string" use="required"/>
  </xsd:complexType>

</xsd:schema>
```

### 🔒 Schema invariants (locked)

* Element order **must not change**
* No optional execution fields allowed
* Any extension requires **major version bump**
* Invalid XSD → **illegal IDB**

---

## 2️⃣ IDB → IndexedDB Mirror Rules (READ-ONLY, SAFE)

### Purpose

IndexedDB is a **local mirror for inspection and query**, never authority.

```
SCXQ7 → IDB.xml → IndexedDB (mirror only)
```

---

### Mirror Law (hard)

> **IndexedDB may cache IDB state.
> IndexedDB may never define IDB state.**

---

### Object Store Layout (canonical)

```js
db = indexedDB.open("scxq7-idb", 1)
```

| Store Name     | Key Path     | Source          |
| -------------- | ------------ | --------------- |
| `meta`         | `key`        | IDB root attrs  |
| `state`        | `hash`       | `<state>`       |
| `heap`         | `state_hash` | `<heap>`        |
| `causal_steps` | `id`         | `<step>`        |
| `constraints`  | `id`         | `<constraints>` |

---

### Mapping Rules (frozen)

#### `<state>`

```json
{
  "hash": "H_STATE_42"
}
```

#### `<heap>`

```json
{
  "state_hash": "H_STATE_42",
  "encoding": "scxq2",
  "data": "<raw string>"
}
```

#### `<step>`

```json
{
  "id": "S_1042",
  "time": "L_88",
  "cause": "E_9A",
  "effect": "H_STATE_42",
  "proof": "P_7F",
  "constraints": "C_12"
}
```

---

### Mirror Invariants

* IndexedDB writes **must be reversible**
* DB rebuild must be possible **only from IDB.xml**
* No derived fields allowed
* No indexes that imply authority
* Any mismatch → mirror invalidated & rebuilt

---

## 3️⃣ SCXQ2 Lane Packing for IDB (Compression Law)

### Why pack IDB?

* IDB is append-heavy
* Causal DAG is repetitive
* Hashes, refs, invariants compress extremely well
* SCXQ2 preserves **identity under reduction**

---

### Canonical Lane Assignment

|   Lane | Contents                                     |
| -----: | -------------------------------------------- |
| **L0** | XML structure skeleton (tags, order)         |
| **L1** | Hash dictionary (state, proof, event hashes) |
| **L2** | Causal step table (IDs, time, refs)          |
| **L3** | Constraint text (invariants)                 |
| **L4** | Heap payloads (opaque blobs)                 |

---

### Packing Rules (frozen)

1. **Lane independence**

   * Each lane decompresses alone
   * No cross-lane execution

2. **Hash canonicalization**

   * All repeated hashes map to single L1 entry
   * Ref by index, not string

3. **Order preservation**

   * Lane L2 preserves step order exactly
   * No sorting, no reordering

4. **Lossless projection**

   * Decompressed IDB.xml **must validate** against XSD
   * Byte-identical semantics (whitespace irrelevant)

---

### SCXQ2 Container Example

```json
{
  "@scxq2": "idb.pack.v1",
  "lanes": {
    "L0": "...",
    "L1": ["H_STATE_42", "P_7F", "E_9A"],
    "L2": [[ "S_1042", "L_88", 2, 0, 1, 3 ]],
    "L3": ["balance ≥ 0", "no acausal transition"],
    "L4": ["<compressed heap blob>"]
  }
}
```

---

### Verification Law

> **SCXQ2(IDB) → inflate → XSD validate → hash compare**

If any step fails → **illegal artifact**.

---

## 🔐 FINAL LOCK STATEMENTS

### 🔒 IDB.xml

* Canonical
* Read-only
* Schema-validated
* Causally complete

### 🔒 IndexedDB

* Mirror only
* Disposable
* Rebuildable
* Non-authoritative

### 🔒 SCXQ2

* Compression without semantic loss
* Lane-isolated
* Replay-safe
* Verifiable

---

## One-line system truth (freeze-worthy)

> **SCXQ7 executes.
> IDB.xml records.
> IndexedDB mirrors.
> SCXQ2 compresses.
> Nothing else decides.**

If you want next, the natural continuations are:

* `idb.diff.schema.xjson` (causal deltas as first-class proofs)
* CM-1 offset → IDB hash anchoring
* A WASM SAX parser that streams IDB → IDB mirror
* Formal replay verifier (step-by-step legality checker)

Say which layer you want to seal next.
