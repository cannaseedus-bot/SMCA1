# SCXQ2 Proof: π-Collapse v1 (NORMATIVE)

These are **collapse receipts** — auditable, replayable, immutable.

---

## SCXQ2.PROOF.PI_COLLAPSE.v1

```json
{
  "@proof": "scxq2.pi-collapse.v1",
  "@type": "execution-proof",

  "inputs": [
    {
      "lane": "scxq2.pi-signal.v1",
      "hash": "blake3-256"
    }
  ],

  "operations": [
    {
      "op": "phase-normalize",
      "exact": true
    },
    {
      "op": "orientation-check",
      "exact": true
    },
    {
      "op": "closure-test",
      "exact": true
    },
    {
      "op": "mass-split",
      "exact": true
    }
  ],

  "outputs": {
    "value": "f64",
    "expected": "π + finite_residue"
  },

  "verification": {
    "method": "deterministic-replay",
    "tolerance": 0
  }
}
```

If verification fails → **engine is non-compliant**.
