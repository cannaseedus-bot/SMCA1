# Cluster-Call Spec v1.0 (Frozen Draft)

**Scope:** XML execution plane only  
**Authority:** Subordinate to TOML axioms  
**Non-goals:** Scheduling policy, retries, load balancing, streaming

---

## 1) `cluster-call.xsd` (Executable, Phase-Bound)

```xml
<?xml version="1.0" encoding="UTF-8"?>
<xs:schema
  xmlns:xs="http://www.w3.org/2001/XMLSchema"
  targetNamespace="urn:scxq7:cluster-call:v1"
  xmlns="urn:scxq7:cluster-call:v1"
  elementFormDefault="qualified"
  attributeFormDefault="unqualified">

  <!-- ========= Root ========= -->
  <xs:element name="cluster-call">
    <xs:complexType>
      <xs:sequence>
        <xs:element name="inputs" type="InputList" minOccurs="0"/>
        <xs:element name="outputs" type="OutputList" minOccurs="1"/>
        <xs:element name="constraints" type="Constraints" minOccurs="0"/>
        <xs:element name="proof" type="ProofSpec" minOccurs="0"/>
      </xs:sequence>

      <!-- Required bindings -->
      <xs:attribute name="target" type="xs:string" use="required"/>
      <xs:attribute name="kernel" type="xs:string" use="required"/>

      <!-- Optional bindings -->
      <xs:attribute name="domain" type="xs:string" use="optional"/>
      <xs:attribute name="timeout_ms" type="xs:nonNegativeInteger" use="optional"/>
      <xs:attribute name="mode" use="optional" default="sync">
        <xs:simpleType>
          <xs:restriction base="xs:string">
            <xs:enumeration value="sync"/>
            <xs:enumeration value="async"/>
          </xs:restriction>
        </xs:simpleType>
      </xs:attribute>
      <xs:attribute name="air_gap" type="xs:boolean" use="optional" default="false"/>
    </xs:complexType>
  </xs:element>

  <!-- ========= Inputs / Outputs ========= -->
  <xs:complexType name="InputList">
    <xs:sequence>
      <xs:element name="input" type="IORef" maxOccurs="unbounded"/>
    </xs:sequence>
  </xs:complexType>

  <xs:complexType name="OutputList">
    <xs:sequence>
      <xs:element name="output" type="IORef" maxOccurs="unbounded"/>
    </xs:sequence>
  </xs:complexType>

  <xs:complexType name="IORef">
    <xs:attribute name="ref" type="xs:string" use="required"/>
    <xs:attribute name="format" type="xs:string" use="optional"/>
    <xs:attribute name="hash" type="xs:string" use="optional"/>
  </xs:complexType>

  <!-- ========= Constraints ========= -->
  <xs:complexType name="Constraints">
    <xs:sequence>
      <xs:element name="deterministic" type="xs:boolean" minOccurs="0"/>
      <xs:element name="side_effects" type="xs:boolean" minOccurs="0"/>
      <xs:element name="memory_mb" type="xs:nonNegativeInteger" minOccurs="0"/>
    </xs:sequence>
  </xs:complexType>

  <!-- ========= Proof ========= -->
  <xs:complexType name="ProofSpec">
    <xs:sequence>
      <xs:element name="require" type="xs:boolean"/>
      <xs:element name="scheme" type="xs:string" minOccurs="0"/>
      <xs:element name="bind" type="xs:string" minOccurs="0"/>
    </xs:sequence>
  </xs:complexType>

</xs:schema>
```

**Hard rules**

* `target` and `kernel` **must** exist in TOML axioms.
* XML **cannot** widen constraints beyond TOML.
* Outputs re-enter as **data only**.

---

## 2) Failure semantics (closed, deterministic)

All failures collapse to explicit states. No retries unless XML explicitly
schedules them.

### 2.1 Timeout

**Trigger:** `timeout_ms` exceeded  
**Action:** Abort call, emit `@failure.timeout`

**Collapse rule**

* `mode=sync`: phase fails
* `mode=async`: output slot marked `unresolved`

**Proof:** Timeout proof optional (recommended)

### 2.2 Divergence (non-determinism)

**Trigger**

* Same inputs, same kernel, different outputs
* Or cluster declares `deterministic=true` but violates

**Action:** Reject result, emit `@failure.divergence`  
**Collapse rule:** Hard fail (no projection)  
**Proof:** Required if `deterministic=true`

### 2.3 Mismatch (proof / hash failure)

**Trigger**

* Output hash ≠ expected
* SCXQ2 identity proof fails
* Domain mismatch

**Action:** Reject output, emit `@failure.mismatch`  
**Collapse rule:** Hard fail  
**Proof:** Mandatory

### 2.4 Unauthorized kernel / target

**Trigger:** Not declared in TOML  
**Action:** Abort before execution  
**Collapse rule:** Illegal state

---

## 3) SCXQ2 proof hooks (mandatory interface)

Every cluster result may (and sometimes must) bind into SCXQ2.

### 3.1 Proof envelope (returned by cluster)

```json
{
  "@scxq2": {
    "version": "1",
    "input_hash": "H(in)",
    "output_hash": "H(out)",
    "kernel": "fft_mul",
    "domain": "bigint",
    "preserves_identity": true,
    "witness": "lane://proof/1234"
  }
}
```

### 3.2 XML binding

```xml
<cluster-call target="python" kernel="fft_mul" timeout_ms="500">
  <inputs>
    <input ref="scxq2.block.A"/>
    <input ref="scxq2.block.B"/>
  </inputs>
  <outputs>
    <output ref="scxq2.block.C"/>
  </outputs>
  <proof>
    <require>true</require>
    <scheme>scxq2.v1</scheme>
    <bind>scxq2.block.C</bind>
  </proof>
</cluster-call>
```

**Invariant**

> If `require=true` and proof fails → **no collapse allowed**.

---

## 4) Air-gapped / offline cluster legality

Air-gapped execution is first-class, not a special case.

### 4.1 TOML declaration (required)

```toml
[axiom.clusters.python]
air_gapped = true
network = "forbidden"
inputs = "sealed"
outputs = "sealed"
```

### 4.2 XML invocation

```xml
<cluster-call
  target="python"
  kernel="fft_mul"
  air_gap="true"
  timeout_ms="0">
</cluster-call>
```

### 4.3 Air-gap rules

* No network I/O
* No clock dependence
* No entropy sources
* Inputs/outputs via sealed artifacts only
* Determinism mandatory
* SCXQ2 proof mandatory

Violation → illegal state.

---

## 5) Authority summary (final)

```
TOML  → declares which clusters may exist
XML   → schedules when they are called
BOT   → executes constrained computation
SCXQ2 → proves identity preservation
```

No upward authority. No feedback. No hidden state.

---

## 6) One-line law

> **A cluster may compute,  
> but only XML may ask,  
> only TOML may permit,  
> and only SCXQ2 may validate.**
