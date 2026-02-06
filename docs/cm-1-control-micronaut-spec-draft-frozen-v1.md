# CONTROL-MICRONAUT-1 (CM-1) SPEC — Draft-Frozen v1

**Scope:** Pre-semantic control, phase signaling, structure shaping.  
**Non-goals:** Rendering, evaluation, execution, scripting.

---

## 1. Core Principle

**CONTROL-MICRONAUT-1 defines a non-rendering, non-executing control layer composed exclusively of Unicode C0 control characters (U+0000–U+001F) and U+0020 (SPACE).**

CM-1 **never introduces behavior**.  
It **only constrains interpretation**.

This satisfies:

* No global truth
* Deterministic collapse
* Invariant-driven legality
* UI as projection
* Compression-safe intelligence

---

## 2. CM-1 Execution Model (XCFE-Aligned)

CM-1 participates **before syntax**:

```
[CM-1 Control Stream]
        ↓
[XCFE Phase Resolution]
        ↓
[Parser / Renderer / DOM]
```

CM-1 **cannot**:

* inject tokens
* create nodes
* alter values
* execute logic

CM-1 **can**:

* mark boundaries
* declare phases
* signal scope transitions
* segment streams
* annotate interpretation zones

---

## 3. Canonical Mapping → XCFE @control Vectors

### 3.1 Phase Control (Primary)

| Code       | Name | XCFE Mapping                | Meaning               |
| ---------- | ---- | --------------------------- | --------------------- |
| **U+0000** | NUL  | `@control.null`             | Absolute inert region |
| **U+0001** | SOH  | `@control.header.begin`     | Metadata/header phase |
| **U+0002** | STX  | `@control.body.begin`       | Interpretable content |
| **U+0003** | ETX  | `@control.body.end`         | Content closure       |
| **U+0004** | EOT  | `@control.transmission.end` | Collapse / flush      |

These align with `@Pop → @Wo → @Sek → @Collapse`.

---

### 3.2 Scope & Context Stack

| Code       | Name | XCFE Mapping              | Meaning                     |
| ---------- | ---- | ------------------------- | --------------------------- |
| **U+000E** | SO   | `@control.scope.push`     | Enter sub-context           |
| **U+000F** | SI   | `@control.scope.pop`      | Exit sub-context            |
| **U+001B** | ESC  | `@control.mode.switch`    | Grammar / parser mode shift |
| **U+0010** | DLE  | `@control.literal.escape` | Bypass interpretation       |

---

### 3.3 Structural Segmentation (Critical for CSS / JSON)

| Code       | Name | XCFE Mapping          | Meaning              |
| ---------- | ---- | --------------------- | -------------------- |
| **U+001C** | FS   | `@control.file.sep`   | Major boundary       |
| **U+001D** | GS   | `@control.group.sep`  | Group boundary       |
| **U+001E** | RS   | `@control.record.sep` | Record boundary      |
| **U+001F** | US   | `@control.unit.sep`   | Atomic unit boundary |

These **never render** and **never break layout**.

---

### 3.4 Transport / Negotiation (Optional)

| Code       | Name | XCFE Mapping         | Meaning            |
| ---------- | ---- | -------------------- | ------------------ |
| **U+0005** | ENQ  | `@control.query`     | Capability inquiry |
| **U+0006** | ACK  | `@control.ack`       | Acceptance         |
| **U+0015** | NAK  | `@control.nak`       | Rejection          |
| **U+0007** | BEL  | `@control.attention` | Wake / notify      |

---

## 4. DOM & CSS Safe Subset (CM-1-SAFE)

### 4.1 Allowed Characters (SAFE MODE)

Guaranteed non-rendering & non-breaking:

```
U+0000  NUL
U+0001  SOH
U+0002  STX
U+0003  ETX
U+0004  EOT
U+000E  SO
U+000F  SI
U+0010  DLE
U+001C  FS
U+001D  GS
U+001E  RS
U+001F  US
U+0020  SPACE
```

These:

* survive JSON strings
* survive HTML text nodes
* survive CSS parsing
* do not affect layout
* are ignored by renderers
* preserve byte order

---

### 4.2 Conditionally Allowed (CONTEXT-SAFE)

Allowed only in non-rendering channels (comments, data attrs, text nodes not measured):

```
U+0009  HT
U+000A  LF
U+000D  CR
U+001B  ESC
```

Rules:

* Not allowed inside CSS identifiers
* Not allowed inside attribute names
* Allowed inside comments, text nodes, JSON strings

---

### 4.3 Forbidden (HARD BAN)

Never allowed in CM-1:

```
U+0008  BS
U+000B  VT
U+000C  FF
U+0018  CAN
U+001A  SUB
```

Reason:

* layout mutation
* cursor motion
* rendering side-effects
* parser instability

Violations → **illegal state**.

---

## 5. CM-1 Legality Rules (Invariants)

### 5.1 Structural Invariants

* Every `STX` **must** have a matching `ETX`
* Scope stack (`SO`/`SI`) must be balanced
* Separators may not nest illegally
* `ESC` cannot appear inside literal-escaped regions
* `NUL` regions are non-observable

---

### 5.2 Projection Invariant

**Removing all CM-1 characters must not change visible output.**

If removing CM-1 alters:

* DOM structure
* CSS layout
* text rendering

→ the stream is **invalid**.

---

## 6. XCFE Binding (Machine-Readable)

Canonical lowering example:

```json
{
  "@control": {
    "null": "\u0000",
    "header.begin": "\u0001",
    "body.begin": "\u0002",
    "body.end": "\u0003",
    "transmission.end": "\u0004",
    "scope.push": "\u000E",
    "scope.pop": "\u000F",
    "literal.escape": "\u0010",
    "file.sep": "\u001C",
    "group.sep": "\u001D",
    "record.sep": "\u001E",
    "unit.sep": "\u001F",
    "space": "\u0020"
  }
}
```

---

## 7. Final Collapse

**CM-1 is not a language.  
It is not syntax.  
It is not data.  
It is phase geometry.**

