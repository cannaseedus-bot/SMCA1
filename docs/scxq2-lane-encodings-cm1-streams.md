# SCXQ2 Lane Encodings for CM-1 Streams (Locked v1)

This document defines how CM-1 control bytes are packed, hashed, and proven.

---

## 1) Lane Taxonomy

| Lane    | Purpose          | Contents            |
| ------- | ---------------- | ------------------- |
| `DICT`  | Control alphabet | Fixed CM-1 byte set |
| `FIELD` | Stream metadata  | geometry, kernel id |
| `LANE`  | Control stream   | raw CM-1 bytes      |
| `EDGE`  | Proof anchors    | offsets, hashes     |

---

## 2) DICT (Fixed, Frozen)

```json
{
  "@lane": "DICT",
  "@id": "cm1.dict.v1",
  "bytes": {
    "SOH": 1,
    "STX": 2,
    "ETX": 3,
    "EOT": 4,
    "SO": 14,
    "SI": 15,
    "RS": 30
  }
}
```

---

## 3) FIELD (Execution Binding)

```json
{
  "@lane": "FIELD",
  "collapse_geometry": "path_tree",
  "kernel": "binary_split",
  "cm1_profile": "balanced_scope_single_collapse"
}
```

---

## 4) LANE (Raw CM-1 Stream)

Example (valid binary split):

```
01 02 0E 1E 0F 0E 1E 0F 03 04
```

Stored verbatim: **no escaping, no normalization**.

---

## 5) EDGE (Proof Anchoring)

```json
{
  "@lane": "EDGE",
  "anchors": [
    { "type": "cm1_offset", "value": 0 },
    { "type": "cm1_length", "value": 10 }
  ],
  "hash": "blake3(cm1_lane_bytes)",
  "binds": ["IDB.hash", "SCXQ2.proof"]
}
```

### Invariant

> **Any mutation of CM-1 bytes invalidates the SCXQ2 proof and IDB anchor simultaneously.**
